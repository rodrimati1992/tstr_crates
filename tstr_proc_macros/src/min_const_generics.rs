use crate::{
    TStr,
    used_proc_macro::{Span, TokenStream},
    utils::{char_token, colon2_token, ident_token, punct_token, usize_token},
};

pub(crate) fn output_tstr_param(crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    let string = tstr.string.chars().collect::<Vec<char>>();
    let span = tstr.span;

    out.extend(crate_path.clone());
    out.extend(colon2_token(span));
    out.extend(ident_token("___", span));
    out.extend(punct_token('<', span));
    out.extend(crate::nested_tuple_compute::compute(
        &string,
        span,
        &mut |string, ts| {
            write_chars(ts, string, crate_path, span);
        },
    ));
    out.extend(punct_token(',', span));
    out.extend(usize_token(tstr.string.len(), span));
    out.extend(punct_token('>', span));
}

fn write_chars(ts: &mut TokenStream, string: &[char], crate_path: &TokenStream, span: Span) {
    ts.extend(crate_path.clone());
    ts.extend(colon2_token(span));
    ts.extend(ident_token("__", span));
    ts.extend(punct_token('<', span));
    for &c in string {
        ts.extend(char_token(c, span));
        ts.extend(punct_token(',', span));
    }
    ts.extend(punct_token('>', span));
}
