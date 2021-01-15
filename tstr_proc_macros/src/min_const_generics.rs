#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
fn output_tstr_param(crate_path: &TokenStream, tstr: &TStr, out: &mut TokenStream) {
    let string = tstr.string.chars().collect::<Vec<char>>();
    let span = tstr.span;
    
    const CHUNK_SIZE: usize = 8;
    if string.len() < CHUNK_SIZE {
        write_chars(out, &string, &crate_path, span)
    } else {
        let tt = paren(span, |out| {
            for chunk in string.chunks(CHUNK_SIZE) {
                write_chars(out, chunk, &crate_path, span);
                out.extend(punct_token(',', span));
            }
        });
        out.extend(iter::once(tt));
    }
}


#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
fn write_chars(ts: &mut TokenStream, string: &[char], crate_path: &TokenStream, span: Span) {
    const TY: &[&str; 9] = &[
        "NO THIS SHOULDN'T HAPPEN",
        "__a",
        "__b",
        "__c",
        "__d",
        "__e",
        "__f",
        "__g",
        "__",
    ];

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
