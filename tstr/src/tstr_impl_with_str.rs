use crate::{___, __TStrArg, __TStrArgBinary};

use typewit::const_marker::Str;

impl<const S: &'static str> __TStrArg for crate::___<S> {
    #[doc(hidden)]
    const __LENGTH: usize = S.len();

    #[doc(hidden)]
    const __BYTES: &[u8] = S.as_bytes();

    #[doc(hidden)]
    const __STR: &str = S;

    #[doc(hidden)]
    type __WithRhs<Rhs: __TStrArg> = <Rhs as __TStrArg>::__WithLhsArgs<S>;

    type __WithLhsArgs<const LEFT_S: &'static str> = (crate::___<LEFT_S>, crate::___<S>);
}

/// private implementation detail
#[doc(hidden)]
pub trait __TStrRepr {}

impl<const S: &'static str> __TStrRepr for ___<S> {}

////////////////////////////////////////////////////////////////////////////////

impl<const S1: &'static str, const S2: &'static str> __TStrArgBinary
    for (crate::___<S1>, crate::___<S2>)
{
    type Lhs = crate::___<S1>;

    type Rhs = crate::___<S2>;

    #[doc(hidden)]
    const __EQ: bool = crate::utils::str_eq(S1, S2);

    #[doc(hidden)]
    const __CMP: core::cmp::Ordering = crate::utils::str_cmp(S1, S2);

    #[doc(hidden)]
    const __TYPE_CMP: typewit::TypeCmp<crate::TStr<Self::Lhs>, crate::TStr<Self::Rhs>> =
        Str::<S1>.equals(Str::<S2>).map(WithStrFn);
}

typewit::inj_type_fn! {
    struct WithStrFn;

    impl<const S: &'static str> Str<S> => crate::TStr<crate::___<S>>
}
