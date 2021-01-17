#![allow(clippy::or_fun_call)]
#![allow(clippy::useless_conversion)]

extern crate proc_macro;

#[cfg(not(feature = "proc_macro2_"))]
use proc_macro as used_proc_macro;

#[cfg(feature = "proc_macro2_")]
use proc_macro2 as used_proc_macro;

use std::iter;

#[allow(unused_imports)]
use used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[cfg(feature = "syn_")]
mod use_syn;

#[cfg(not(feature = "syn_"))]
mod non_syn_parsing;

#[cfg(not(feature = "const_generics"))]
mod nested_tuple_compute;

mod utils;

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
mod min_const_generics;

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
use min_const_generics::output_tstr_param;

#[cfg(not(feature = "min_const_generics"))]
mod no_const_generics;

#[cfg(not(feature = "min_const_generics"))]
use no_const_generics::output_tstr_param;

#[doc(hidden)]
#[proc_macro]
pub fn __ts_impl(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use crate::utils::{paren, punct_token};

    let input_tokens = TokenStream::from(input_tokens);

    #[cfg(feature = "syn_")]
    let parsed = syn::parse2::<Inputs>(input_tokens);

    #[cfg(not(feature = "syn_"))]
    let parsed = non_syn_parsing::parse_inputs(input_tokens);

    match parsed {
        Ok(Inputs {
            crate_path,
            strings,
        }) => {
            let mut out = TokenStream::new();
            if strings.len() == 1 {
                output_tstr(&crate_path, &strings[0], &mut out);
            } else {
                let tt = paren(Span::call_site(), |out| {
                    for tstr in &strings {
                        output_tstr(&crate_path, tstr, out);
                        out.extend(punct_token(',', tstr.span));
                    }
                });
                out.extend(iter::once(tt));
            }
            out
        }
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn output_tstr(crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    use crate::utils::{colon2_token, ident_token, punct_token};

    let span = tstr.span;
    out.extend(crate_path.clone());
    out.extend(colon2_token(span));
    out.extend(ident_token("TStr", span));
    out.extend(punct_token('<', span));

    #[cfg(feature = "const_generics")]
    {
        out.extend(crate_path.clone());
        out.extend(colon2_token(span));
        out.extend(ident_token("___", span));
        out.extend(punct_token('<', span));
    }

    output_tstr_param(crate_path, tstr, out);

    #[cfg(feature = "const_generics")]
    {
        out.extend(punct_token('>', span));
    }

    out.extend(punct_token('>', span));
}

#[cfg(feature = "const_generics")]
fn output_tstr_param(_crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    let string = tstr.string.as_str();
    let span = tstr.span;

    let mut lit = Literal::string(&string);
    lit.set_span(span);
    out.extend(iter::once(TokenTree::from(lit)));
}

struct Inputs {
    crate_path: TokenStream,
    strings: Vec<TStr>,
}

struct TStr {
    string: String,
    span: Span,
}
