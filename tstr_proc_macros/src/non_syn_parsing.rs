use std::iter::once;

#[allow(unused_imports)]
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use super::{Inputs, TStr};

pub(crate) fn parse_inputs(ts: TokenStream) -> Result<Inputs, Error> {
    let mut iter = ts.into_iter();

    let crate_path = match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            group.stream()
        }
        Some(x) => {
            return Err(Error::new(
                x.span(),
                &format!("Expected parentheses: found {}", x),
            ))
        }
        None => {
            return Err(Error::new(
                Span::call_site(),
                "Expected parentheses, found nothing",
            ))
        }
    };

    Ok(Inputs {
        crate_path,
        strings: iter
            .map(parse_str_from_token_tree)
            .collect::<Result<_, _>>()?,
    })
}

fn parse_str_from_token_tree(tt: TokenTree) -> Result<TStr, Error> {
    const IN_MSG: &str = "Expected one of: string literal, integer literal, identifier";
    match tt {
        TokenTree::Ident(ident) => {
            let mut string = ident.to_string();
            let trimmed = string.trim_start_matches("r#");
            if trimmed.len() != string.len() {
                string = trimmed.to_string();
            }

            Ok(TStr {
                string,
                span: ident.span(),
            })
        }
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
            parse_str_from_token_tree(group.stream().into_iter().next().unwrap())
        }
        TokenTree::Literal(lit) => parse_literal(lit),
        x => Err(Error::new(x.span(), &format!("{}\nFound: {}", IN_MSG, x))),
    }
}

fn parse_literal(lit: Literal) -> Result<TStr, Error> {
    let span = lit.span();
    let string = lit.to_string();

    let string = if string.starts_with('"') {
        parse_string(&string, span)?
    } else if string.starts_with('r') {
        parse_raw_string(&string, span)?
    } else {
        parse_integer(&string, span)?
    };

    Ok(TStr { string, span })
}

fn parse_string<'a>(input: &'a str, span: Span) -> Result<String, Error> {
    if !input.ends_with('"') {
        return Err(Error::new(
            span,
            "Somehow there's no terminating quote character?",
        ));
    }

    let make_err = |rem: &str, error: &str| -> Error {
        let pos = rem.as_ptr() as usize - input.as_ptr() as usize;

        let upto = input[..pos]
            .chars()
            .rev()
            .take(10)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        Error::new(span, &format!("Error: {}    After: {}", error, upto,))
    };

    let mut rem = &input[1..input.len() - 1];
    let mut out = String::new();

    loop {
        let end_copied = rem.find('\\').unwrap_or(rem.len());
        out.push_str(&rem[..end_copied]);

        rem = &rem[end_copied..];

        if rem.is_empty() {
            break;
        }

        // The byte after the '\\' character
        let b = get_byte(rem, 1);

        // Now we're at the character right after the matched one.
        rem = &rem[2..];

        out.push(match b {
            b'x' => {
                if let Some(hex) = rem.get(..2) {
                    let num = u8::from_str_radix(hex, 16)
                        .ok()
                        .filter(|&x| x < 128)
                        .ok_or_else(|| {
                            make_err(
                                rem,
                                &format!("expected values from \\x00 to \\x7F, found: {}", hex),
                            )
                        })?;
                    out.push(num as char);
                } else {
                    return Err(make_err(rem, "invalid ascii escape"));
                }
                rem = &rem[2..];
                continue;
            }
            b'u' => {
                if let Some(end_brace) = rem.bytes().position(|b| b == b'}') {
                    let c: char = u32::from_str_radix(&rem[1..end_brace], 16)
                        .ok()
                        .and_then(std::char::from_u32)
                        .ok_or_else(|| {
                            make_err(
                                rem,
                                &format!("Invalid unicode escape: {}", &rem[..end_brace]),
                            )
                        })?;
                    out.push(c);

                    rem = &rem[end_brace + 1..];
                } else {
                    return Err(make_err(rem, "Expected closing brace for unicode escape"));
                }
                continue;
            }
            b'n' => '\n',
            b'r' => '\r',
            b't' => '\t',
            b'\\' => '\\',
            b'0' => '\0',
            b'\'' => '\'',
            b'"' => '"',
            b'\r' | b'\n' => {
                rem = rem.trim_start();
                continue;
            }
            _ => return Err(make_err(rem, "invalid escape")),
        });
    }

    Ok(out)
}

fn get_byte(s: &str, at: usize) -> u8 {
    match s.as_bytes().get(at) {
        Some(&x) => x,
        None => 0,
    }
}

fn parse_raw_string(input: &str, span: Span) -> Result<String, Error> {
    let input = &input[1..];

    let hash_count = input
        .bytes()
        .position(|b| b != b'#')
        .filter(|&p| input.as_bytes()[p] == b'"')
        .ok_or_else(|| {
            Error::new(
                span,
                "Couldn't find initial '\"' character in raw string literal.",
            )
        })?;

    let end_quote = input
        .bytes()
        .rev()
        .position(|b| b != b'#')
        .map(|p| input.len() - 1 - p)
        .filter(|&p| input.as_bytes()[p] == b'"')
        .ok_or_else(|| {
            Error::new(
                span,
                "Couldn't find final '\"' character in raw string literal.",
            )
        })?;

    Ok(input[hash_count + 1..end_quote].to_string())
}

fn parse_integer(input: &str, span: Span) -> Result<String, Error> {
    fn make_err(input: &str, span: Span) -> Error {
        Error::new(
            span,
            &format!("could not parse as integer literal: {}", input),
        )
    }

    let input = input.replace('_', "");
    let input = input.as_str();

    let input_bytes = input.as_bytes();
    if input_bytes.get(0) == Some(&b'0') {
        let radix = match input_bytes.get(1) {
            Some(b'x') => 16,
            Some(b'o') => 8,
            Some(b'b') => 2,
            Some(_) => {
                return Err(Error::new(
                    span,
                    &format!("Unknown integer prefix: {}", &input[..2]),
                ))
            }
            None => return Ok(String::from("0")),
        };
        u128::from_str_radix(&input[2..], radix).map_err(|_| make_err(input, span))
    } else {
        input.parse::<u128>().map_err(|_| make_err(input, span))
    }
    .map(|i| i.to_string())
}

pub(crate) struct Error {
    span: Span,
    message: String,
}

impl Error {
    fn new(span: Span, message: &str) -> Self {
        Self {
            span,
            message: message.to_string(),
        }
    }

    pub(crate) fn into_compile_error(&self) -> TokenStream {
        let Error { ref message, span } = *self;

        let mut out = TokenStream::new();

        out.extend(crate::ident_token("compile_error", span));

        out.extend(crate::punct_token('!', span));

        let msg_paren = crate::paren(span, |ts| {
            let mut msg = Literal::string(message);
            msg.set_span(self.span);
            let msg = TokenTree::from(msg);
            ts.extend(once(msg))
        });
        out.extend(once(msg_paren));

        out
    }
}

trait TokenTreeExt: Sized {
    fn into_token_tree(self) -> TokenTree;

    fn set_span_recursive(self, span: Span) -> TokenTree {
        let mut tt = self.into_token_tree();

        tt.set_span(span);
        if let TokenTree::Group(group) = tt {
            let delim = group.delimiter();
            let stream = group.stream().set_span_recursive(span);
            tt = TokenTree::Group(Group::new(delim, stream));
        }
        tt.set_span(span);
        tt
    }
}

impl TokenTreeExt for TokenTree {
    fn into_token_tree(self) -> TokenTree {
        self
    }
}

pub trait TokenStreamExt: Sized {
    fn into_token_stream(self) -> TokenStream;

    fn set_span_recursive(self, span: Span) -> TokenStream {
        self.into_token_stream()
            .into_iter()
            .map(|tt| tt.set_span_recursive(span))
            .collect()
    }
}

impl TokenStreamExt for TokenStream {
    fn into_token_stream(self) -> TokenStream {
        self
    }
}
