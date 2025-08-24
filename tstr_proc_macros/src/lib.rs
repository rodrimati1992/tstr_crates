#![allow(clippy::or_fun_call)]
#![allow(clippy::useless_conversion)]

extern crate proc_macro;

#[cfg(not(feature = "proc_macro2_"))]
use proc_macro as used_proc_macro;

#[cfg(feature = "proc_macro2_")]
use proc_macro2 as used_proc_macro;

#[allow(unused_imports)]
use used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[cfg(feature = "syn_")]
mod use_syn;

#[cfg(not(feature = "syn_"))]
mod non_syn_parsing;

#[cfg(not(feature = "str_generics"))]
mod nested_tuple_compute;

mod utils;

#[cfg(not(feature = "str_generics"))]
mod min_const_generics;

#[cfg(not(feature = "str_generics"))]
use min_const_generics::output_tstr_param;

#[doc(hidden)]
#[proc_macro]
pub fn __ts_impl(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
            output_tstr(&crate_path, &strings[0], &mut out);
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

    output_tstr_param(crate_path, tstr, out);

    out.extend(punct_token('>', span));
}

#[cfg(feature = "str_generics")]
fn output_tstr_param(crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    use crate::utils::{colon2_token, ident_token, punct_token};

    let string = tstr.string.as_str();
    let span = tstr.span;

    out.extend(crate_path.clone());
    out.extend(colon2_token(span));
    out.extend(ident_token("___", span));
    out.extend(punct_token('<', span));

    let mut lit = Literal::string(&string);
    lit.set_span(span);
    out.extend([TokenTree::from(lit)]);

    out.extend(punct_token('>', span));
}

struct Inputs {
    crate_path: TokenStream,
    strings: Vec<TStr>,
}

struct TStr {
    string: String,
    span: Span,
}
