use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    LitInt, LitStr,
};

use super::Inputs;

impl Parse for Inputs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);

        let crate_path = content.parse::<proc_macro2::TokenStream>()?;

        let lookahead = input.lookahead1();
        let (string, span) = if lookahead.peek(syn::Ident::peek_any) {
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

        Ok(Self {
            crate_path,
            string,
            span,
        })
    }
}
