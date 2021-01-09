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

////////////////////////////////////////////////////////////////////////////////








////////////////////////////////////////////////////////////////////////////////

