extern crate proc_macro;

use std::iter::{self, FromIterator, Once};

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};

use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    Ident, LitInt, LitStr,
};

#[doc(hidden)]
#[proc_macro]
pub fn __ts_impl(input_tokens: TokenStream1) -> TokenStream1 {
    let input_tokens = TokenStream2::from(input_tokens);
    match syn::parse2::<Inputs>(input_tokens) {
        Ok(Inputs {
            crate_path,
            string,
            span,
        }) => {
            let mut out = TokenStream2::new();
            out.extend(crate_path.clone());
            out.extend(colon2_token(span));
            out.extend(ident_token("TStr", span));
            out.extend(punct_token('<', span));

            out.extend(crate_path.clone());
            out.extend(colon2_token(span));
            out.extend(ident_token("__", span));

            out.extend(punct_token('<', span));
            #[cfg(not(feature = "const_generics"))]
            {
                use proc_macro2::{Delimiter, Group};
                let mut chars = TokenStream2::new();

                for b in string.bytes() {
                    chars.extend(crate_path.clone());
                    chars.extend(colon2_token(span));
                    chars.extend(ident_token(BYTE_NAME[b as usize], span));
                    chars.extend(punct_token(',', span));
                }

                let tt = Group::new(Delimiter::Parenthesis, chars);
                let tt = TokenTree::from(tt);
                out.extend(iter::once(tt));
            }
            #[cfg(feature = "const_generics")]
            {
                use proc_macro2::Literal;
                let mut lit = Literal::string(&string);
                out.extend(iter::once(TokenTree::from(lit)));
            }
            out.extend(punct_token('>', span));
            out.extend(punct_token('>', span));
            // panic!("{}",out)
            out
        }
        Err(e) => e.into_compile_error(),
    }
    .into()
}

fn ident_token(ident: &str, span: Span) -> Once<TokenTree> {
    let ident = syn::Ident::new(ident, span);
    let tt = TokenTree::from(ident);
    iter::once(tt)
}
fn punct_token(token: char, span: Span) -> Once<TokenTree> {
    let mut token = proc_macro2::Punct::new(token, proc_macro2::Spacing::Alone);
    token.set_span(span);
    let tt = TokenTree::from(token);
    iter::once(tt)
}
fn colon2_token(span: Span) -> TokenStream2 {
    let mut token = proc_macro2::Punct::new(':', proc_macro2::Spacing::Joint);
    token.set_span(span);
    TokenStream2::from_iter(vec![TokenTree::from(token.clone()), TokenTree::from(token)])
}

struct Inputs {
    crate_path: TokenStream2,
    string: String,
    span: Span,
}

impl Parse for Inputs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);

        let crate_path = content.parse::<TokenStream2>()?;

        let lookahead = input.lookahead1();
        let (string, span) = if lookahead.peek(Ident::peek_any) {
            let ident = input.parse::<Ident>()?;
            (ident.to_string(), ident.span())
        } else if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            (lit.value(), lit.span())
        } else if lookahead.peek(LitInt) {
            let lit = input.parse::<LitInt>()?;
            (lit.base10_digits().to_string(), lit.span())
        } else {
            return Err(lookahead.error());
        };

        Ok(Self {
            crate_path,
            string,
            span,
        })
    }
}

#[cfg(not(feature = "const_generics"))]
const BYTE_NAME: [&str; 256] = [
    "__0x00", "__0x01", "__0x02", "__0x03", "__0x04", "__0x05", "__0x06", "__0x07", "__0x08",
    "__0x09", "__0x0A", "__0x0B", "__0x0C", "__0x0D", "__0x0E", "__0x0F", "__0x10", "__0x11",
    "__0x12", "__0x13", "__0x14", "__0x15", "__0x16", "__0x17", "__0x18", "__0x19", "__0x1A",
    "__0x1B", "__0x1C", "__0x1D", "__0x1E", "__0x1F", "__0x20", "__0x21", "__0x22", "__0x23",
    "__0x24", "__0x25", "__0x26", "__0x27", "__0x28", "__0x29", "__0x2A", "__0x2B", "__0x2C",
    "__0x2D", "__0x2E", "__0x2F", "__0", "__1", "__2", "__3", "__4", "__5", "__6", "__7", "__8",
    "__9", "__0x3A", "__0x3B", "__0x3C", "__0x3D", "__0x3E", "__0x3F", "__0x40", "__A", "__B",
    "__C", "__D", "__E", "__F", "__G", "__H", "__I", "__J", "__K", "__L", "__M", "__N", "__O",
    "__P", "__Q", "__R", "__S", "__T", "__U", "__V", "__W", "__X", "__Y", "__Z", "__0x5B",
    "__0x5C", "__0x5D", "__0x5E", "___", "__0x60", "__a", "__b", "__c", "__d", "__e", "__f", "__g",
    "__h", "__i", "__j", "__k", "__l", "__m", "__n", "__o", "__p", "__q", "__r", "__s", "__t",
    "__u", "__v", "__w", "__x", "__y", "__z", "__0x7B", "__0x7C", "__0x7D", "__0x7E", "__0x7F",
    "__0x80", "__0x81", "__0x82", "__0x83", "__0x84", "__0x85", "__0x86", "__0x87", "__0x88",
    "__0x89", "__0x8A", "__0x8B", "__0x8C", "__0x8D", "__0x8E", "__0x8F", "__0x90", "__0x91",
    "__0x92", "__0x93", "__0x94", "__0x95", "__0x96", "__0x97", "__0x98", "__0x99", "__0x9A",
    "__0x9B", "__0x9C", "__0x9D", "__0x9E", "__0x9F", "__0xA0", "__0xA1", "__0xA2", "__0xA3",
    "__0xA4", "__0xA5", "__0xA6", "__0xA7", "__0xA8", "__0xA9", "__0xAA", "__0xAB", "__0xAC",
    "__0xAD", "__0xAE", "__0xAF", "__0xB0", "__0xB1", "__0xB2", "__0xB3", "__0xB4", "__0xB5",
    "__0xB6", "__0xB7", "__0xB8", "__0xB9", "__0xBA", "__0xBB", "__0xBC", "__0xBD", "__0xBE",
    "__0xBF", "__0xC0", "__0xC1", "__0xC2", "__0xC3", "__0xC4", "__0xC5", "__0xC6", "__0xC7",
    "__0xC8", "__0xC9", "__0xCA", "__0xCB", "__0xCC", "__0xCD", "__0xCE", "__0xCF", "__0xD0",
    "__0xD1", "__0xD2", "__0xD3", "__0xD4", "__0xD5", "__0xD6", "__0xD7", "__0xD8", "__0xD9",
    "__0xDA", "__0xDB", "__0xDC", "__0xDD", "__0xDE", "__0xDF", "__0xE0", "__0xE1", "__0xE2",
    "__0xE3", "__0xE4", "__0xE5", "__0xE6", "__0xE7", "__0xE8", "__0xE9", "__0xEA", "__0xEB",
    "__0xEC", "__0xED", "__0xEE", "__0xEF", "__0xF0", "__0xF1", "__0xF2", "__0xF3", "__0xF4",
    "__0xF5", "__0xF6", "__0xF7", "__0xF8", "__0xF9", "__0xFA", "__0xFB", "__0xFC", "__0xFD",
    "__0xFE", "__0xFF",
];
