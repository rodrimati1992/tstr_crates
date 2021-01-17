use proc_macro2::{Span, TokenStream};

use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    LitInt, LitStr,
};

use super::{Inputs, TStr};

impl Parse for Inputs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);

        let crate_path = content.parse::<proc_macro2::TokenStream>()?;

        let mut strings = Vec::<TStr>::new();
        while !input.is_empty() {
            strings.push(input.parse()?);
        }

        Ok(Self {
            crate_path,
            strings,
        })
    }
}

impl Parse for TStr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let (string, span) = if lookahead.peek(kw::concat) {
            input.parse::<kw::concat>()?;
            let (span, content) = parse_post_macro_name(input)?;
            let mut value = String::new();

            while !content.is_empty() {
                let tstr = content.parse::<TStr>()?;

                value.push_str(&tstr.string);

                if !content.is_empty() {
                    content.parse::<syn::Token!(,)>()?;
                }
            }

            (value, span)
        } else if lookahead.peek(kw::stringify) {
            input.parse::<kw::stringify>()?;
            let (span, content) = parse_post_macro_name(input)?;
            (content.parse::<TokenStream>()?.to_string(), span)
        } else if lookahead.peek(syn::Ident::peek_any) {
            let ident = input.parse::<syn::Ident>()?;
            let mut value = ident.to_string();
            if value.starts_with("r#") {
                value.drain(..2);
            }
            (value, ident.span())
        } else if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            (lit.value(), lit.span())
        } else if lookahead.peek(LitInt) {
            let lit = input.parse::<LitInt>()?;
            (lit.base10_digits().to_string(), lit.span())
        } else {
            return Err(lookahead.error());
        };
        Ok(Self { string, span })
    }
}

fn parse_post_macro_name(input: ParseStream) -> syn::Result<(Span, ParseBuffer)> {
    input.parse::<syn::Token!(!)>()?;
    let content;
    let paren = parenthesized!(content in input);
    Ok((paren.span, content))
}

mod kw {
    syn::custom_keyword!(concat);
    syn::custom_keyword!(stringify);
}
