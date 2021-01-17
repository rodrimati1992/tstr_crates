use crate::{
    used_proc_macro::{Span, TokenStream},
    utils::{char_token, colon2_token, ident_token, punct_token},
    TStr,
};

pub(crate) fn output_tstr_param(crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    let string = tstr.string.chars().collect::<Vec<char>>();
    let span = tstr.span;

    out.extend(crate::nested_tuple_compute::compute(
        &string,
        span,
        &mut |string, ts| {
            write_chars(ts, string, crate_path, span);
        },
    ));
}

fn write_chars(ts: &mut TokenStream, string: &[char], crate_path: &TokenStream, span: Span) {
    const TY: &[&str; 9] = &["", "__a", "__b", "__c", "__d", "__e", "__f", "__g", "__"];

    ts.extend(crate_path.clone());
    ts.extend(colon2_token(span));
    ts.extend(ident_token(TY[string.len()], span));
    ts.extend(punct_token('<', span));
    for &c in string {
        ts.extend(char_token(c, span));
        ts.extend(punct_token(',', span));
    }
    ts.extend(punct_token('>', span));
}
