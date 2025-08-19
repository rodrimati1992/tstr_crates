use crate::{__TStrRepr, Make, TStr};

/// Trait for bounding
pub trait IsTStr: typewit::Identity<Type = TStr<<Self as IsTStr>::Arg>> + Make {
    /// The type parameter of `TStr`
    type Arg: __TStrArg;

    /// Constructs a TStr.
    const TSTR: TStr<Self::Arg> = TStr::new();

    /// The length of this string when encoded to utf8
    const LENGTH: usize;

    /// This string converted to a uf8-encoded byte slice
    const BYTES: &[u8];

    /// This string converted to a string
    const STR: &str;
}

#[diagnostic::do_not_recommend]
impl<S> IsTStr for TStr<S>
where
    S: __TStrArg,
{
    type Arg = S;

    const LENGTH: usize = S::__LENGTH;

    const BYTES: &[u8] = S::__BYTES;

    const STR: &str = S::__STR;
}

/// implementation detail of tstr crate
#[doc(hidden)]
pub trait __TStrArg: __TStrRepr {
    #[doc(hidden)]
    const __LENGTH: usize;

    #[doc(hidden)]
    const __BYTES: &[u8];

    #[doc(hidden)]
    const __STR: &str;

    #[doc(hidden)]
    type __WithRhs<Rhs: __TStrArg>: __TStrArgBinary;

    #[cfg(feature = "str_generics")]
    type __WithLhsArgs<const LEFT_S: &'static str>: __TStrArgBinary;

    #[cfg(not(feature = "str_generics"))]
    type __WithLhsArgs<LeftS: __TStrRepr, const LEFT_LEN: usize>: __TStrArgBinary;
}

pub trait __TStrArgBinary {
    #[doc(hidden)]
    const __EQ: bool;

    #[doc(hidden)]
    const __CMP: core::cmp::Ordering;
}

pub(crate) type __ToTStrArgBinary<L, R> = <L as __TStrArg>::__WithRhs<R>;
