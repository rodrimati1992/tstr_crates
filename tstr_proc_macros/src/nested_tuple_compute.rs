//! Wraps the input type in nested tuples

use crate::{
    used_proc_macro::{Span, TokenStream},
    utils::{paren, punct_token},
};

use std::iter;

#[cfg(test)]
mod tests;

#[must_use]
pub(crate) fn compute<T, F>(input: &[T], span: Span, func: &mut F) -> TokenStream
where
    F: FnMut(&[T], &mut TokenStream),
{
    let mut out = TokenStream::new();

    if input.is_empty() {
        out.extend(std::iter::once(paren(span, |_| ())));
    } else if input.len() <= CHUNK_SIZE {
        func(input, &mut out);
    } else {
        let tt = paren(span, |out| {
            let lower_power = find_smaller_power(input.len());

            let chunks = input.chunks(lower_power).collect::<Vec<_>>();
            let chunk_count = chunks.len();

            for chunk in chunks
                .into_iter()
                .chain(std::iter::repeat_n(&[][..], CHUNK_SIZE - chunk_count))
            {
                out.extend(compute(chunk, span, func));
                out.extend(punct_token(',', span));
            }
        });
        out.extend(iter::once(tt));
    }

    out
}

const CHUNK_SIZE: usize = 8;

fn find_smaller_power(than: usize) -> usize {
    let mut curr = 1;
    loop {
        let next = curr * 8;
        if next >= than {
            break curr;
        } else {
            curr = next;
        }
    }
}
