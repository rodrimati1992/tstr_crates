use crate::{___, __TStrArgBinary, IsTStr, TStr};

impl<const S: &'static str> __TStrArg for crate::___<S> {
    #[doc(hidden)]
    const __LENGTH: usize = S.len();

    #[doc(hidden)]
    const __BYTES: &[u8] = S.as_bytes();

    #[doc(hidden)]
    const __STR: &str = S.len();

    #[doc(hidden)]
    type __WithRhs<Rhs: __TStrArg> = <Rhs as __TStrArg>::__WithLhsArgs<S>;

    #[doc(hidden)]
    type __WithLhsArgs<const S1: &'static str> = (crate::___<S1>, crate::___<S2>);
}

/// private implementation detail
#[doc(hidden)]
pub trait __TStrRepr {}

impl<const S: &'static str> __TStrRepr for __<S> {}

////////////////////////////////////////////////////////////////////////////////

impl<const S1: &'static str, const S2: &'static str> __TStrArgBinary
    for (crate::___<S1>, crate::___<S2>)
{
    #[doc(hidden)]
    const __EQ: bool = crate::utils::str_eq(S1, S2);

    #[doc(hidden)]
    const __CMP: core::cmp::Ordering = crate::utils::str_cmp(S1, S2);
}
