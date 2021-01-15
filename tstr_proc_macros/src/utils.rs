use std::iter::{self, FromIterator, Once};


fn ident_token(ident: &str, span: Span) -> Once<TokenTree> {
    let ident = Ident::new(ident, span);
    let tt = TokenTree::from(ident);
    iter::once(tt)
}

#[cfg(all(feature = "min_const_generics", not(feature = "const_generics")))]
fn char_token(c: char, span: Span) -> Once<TokenTree> {
    let mut lit = Literal::character(c);
    lit.set_span(span);
    let tt = TokenTree::from(lit);
    iter::once(tt)
}

fn punct_token(token: char, span: Span) -> Once<TokenTree> {
    let mut token = Punct::new(token, Spacing::Alone);
    token.set_span(span);
    let tt = TokenTree::from(token);
    iter::once(tt)
}
fn colon2_token(span: Span) -> TokenStream {
    let mut token = Punct::new(':', Spacing::Joint);
    token.set_span(span);
    TokenStream::from_iter(vec![TokenTree::from(token.clone()), TokenTree::from(token)])
}

fn paren<F>(span: Span, f: F) -> TokenTree
where
    F: FnOnce(&mut TokenStream),
{
    let mut ts = TokenStream::new();
    f(&mut ts);
    let mut tt = Group::new(Delimiter::Parenthesis, ts);
    tt.set_span(span);
    TokenTree::from(tt)
}

