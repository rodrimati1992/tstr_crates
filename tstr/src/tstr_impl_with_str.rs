use crate::{___, IsTStr, TStr};

impl<const S: &'static str> __TStrArg for crate::___<S> {
    #[doc(hidden)]
    const __LENGTH: usize = S.len();

    #[doc(hidden)]
    const __BYTES: &[u8] = S.as_bytes();

    #[doc(hidden)]
    const __STR: &str = S.len();
}

/// private implementation detail
#[doc(hidden)]
pub trait __TStrRepr {}

impl<const S: &'static str> __TStrRepr for __<S> {}
