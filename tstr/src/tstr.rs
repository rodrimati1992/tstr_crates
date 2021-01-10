use core::{
    fmt::{self, Debug},
    marker::PhantomData,
};

///
/// A type-level string type, equivalent to a `&'static str` const parameter.
///
pub struct TStr<T>(pub(crate) PhantomData<T>);

impl<T> TStr<T> {
    /// Constructs the TStr.
    pub const NEW: Self = TStr(PhantomData);
}

#[cfg(feature = "const_generics")]
macro_rules! const_generics_using {
    () => {
        /// Got getting the `&'static str` value of this `TStr`.
        ///
        /// You can use this as the bound for a generic `TStr` parameter.
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        pub trait StrValue {
            /// The `&'static str` value of this `TStr`.
            const STR: &'static str;
        }

        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        impl<const S: &'static str> StrValue for StrValue<__<S>> {
            const STR: &'static str = S;
        }

        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        impl<T> TStr<T>
        where
            Self: StrValue,
        {
            /// The `&'static str` value of this `TStr`.
            pub const STR: &'static str = <Self as StrValue>::Str;
        }
    };
}
#[cfg(feature = "const_generics")]
const_generics_using! {}

impl<T> Copy for TStr<T> {}

impl<T> Clone for TStr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Debug for TStr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TStr").finish()
    }
}

impl<T> core::cmp::PartialEq for TStr<T> {
    #[inline(always)]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> core::cmp::Eq for TStr<T> {}

impl<T> core::cmp::PartialOrd for TStr<T> {
    #[inline(always)]
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<T> core::cmp::Ord for TStr<T> {
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}
