use crate::{Make, TStr};

/// Trait for bounding
pub trait IsTStr: typewit::Identity<Type = TStr<<Self as IsTStr>::Arg>> + Make {
    /// The type parameter of `TStr`
    type Arg;

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
pub trait __TStrArg: crate::__TStrRepr {
    #[doc(hidden)]
    const __LENGTH: usize;

    #[doc(hidden)]
    const __BYTES: &[u8];

    #[doc(hidden)]
    const __STR: &str;
}
