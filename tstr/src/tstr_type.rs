use core::{
    cmp::Ordering,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[cfg(feature = "const_panic")]
use const_panic::{
    PanicVal,
    fmt::{FmtArg, PanicFmt},
};

use crate::IsTStr;

/// A type-level string type, emulates a `&'static str` const parameter.
///
/// # Examples
///
/// ### Accessing Fields
///
/// This example demonstrates how you can use `TStr` to implement a generic accessor trait.
///
/// ```rust
/// use tstr::{IsTStr, TS, TStr, ts};
///
/// fn main() {
///     let mut tup = (3, 5, 8);
///     
///     assert_eq!(tup.get(ts!(0)), &3);
///     assert_eq!(tup.get(ts!(1)), &5);
///     assert_eq!(tup.get(ts!(2)), &8);
///
///     let old_0 = replace(&mut tup, ts!(0), 333);
///     let old_1 = replace(&mut tup, ts!(1), 555);
///     let old_2 = replace(&mut tup, ts!(2), 888);
///     
///     assert_eq!(tup.get(ts!(0)), &333);
///     assert_eq!(tup.get(ts!(1)), &555);
///     assert_eq!(tup.get(ts!(2)), &888);
///
///     assert_eq!(old_0, 3);
///     assert_eq!(old_1, 5);
///     assert_eq!(old_2, 8);
///     
/// }
///
/// fn replace<T, N>(this: &mut T, name: N, replacement: T::Field) -> T::Field
/// where
///     N: IsTStr,
///     T: Access<N, Field: Clone>,
/// {
///     let ret = this.get(name).clone();
///     this.set(name, replacement);
///     ret
/// }
///
///
/// trait Access<N> {
///     type Field;
///
///     fn get(&self, _field_name: N) -> &Self::Field;
///     fn set(&mut self, _field_name: N, val: Self::Field);
/// }
///
/// impl_access_for_tuple3!{ 0: A }
/// impl_access_for_tuple3!{ 1: B }
/// impl_access_for_tuple3!{ 2: C }
///
/// macro_rules! impl_access_for_tuple3 {
///     ($field:tt: $field_type:ty) => {
///         impl<A, B, C> Access<TS!($field)> for (A, B, C) {
///             type Field = $field_type;
///     
///             fn get(&self, _field_name: TS!($field)) -> &$field_type {
///                 &self.$field
///             }
///             fn set(&mut self, _field_name: TS!($field), val: $field_type){
///                 self.$field = val;
///             }
///         }
///     };
/// } use impl_access_for_tuple3;
///
/// ```
///
/// ### Parsing integers
///
/// Parsing integers from `TStr`s, since the primitive integers all have
/// [`const fn from_str_radix`](u32::from_str_radix) functions,
/// parsing them doesn't require direct support from `TStr` itself.
///
/// ```rust
/// use tstr::ts;
///
/// // parses the number at compile-time!
/// const NUMBER: u32 = tstr::unwrap!(u32::from_str_radix(tstr::to_str(ts!(1234)), 10));
///
/// assert_eq!(NUMBER, 1234u32);
/// ```
///
///
pub struct TStr<S>(pub(crate) PhantomData<fn() -> S>);

impl<S> TStr<S> {
    /// Constructs the TStr.
    pub const fn new() -> Self {
        TStr(PhantomData)
    }
}

impl<S> Copy for TStr<S> {}

impl<S> Clone for TStr<S> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Default for TStr<S> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Debug for TStr<S>
where
    Self: IsTStr,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl<S> fmt::Display for TStr<S>
where
    Self: IsTStr,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.to_str(), f)
    }
}

impl<S, S2> core::cmp::PartialEq<S2> for TStr<S>
where
    Self: IsTStr,
    S2: IsTStr,
{
    #[inline(always)]
    fn eq(&self, other: &S2) -> bool {
        self.tstr_eq(*other)
    }
}

impl<S> core::cmp::Eq for TStr<S> where Self: IsTStr {}

impl<S, S2> core::cmp::PartialOrd<S2> for TStr<S>
where
    Self: IsTStr,
    S2: IsTStr,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &S2) -> Option<Ordering> {
        Some(self.tstr_cmp(*other))
    }
}

impl<S> core::cmp::Ord for TStr<S>
where
    Self: IsTStr,
{
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

// rustc expands #[derive(Hash)] on unit structs into this
impl<S> Hash for TStr<S>
where
    Self: IsTStr,
{
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
    }
}

#[cfg(feature = "const_panic")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_panic")))]
impl<S> PanicFmt for TStr<S>
where
    Self: IsTStr,
{
    type This = Self;
    type Kind = const_panic::IsCustomType;

    const PV_COUNT: usize = 1;
}

#[cfg(feature = "const_panic")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_panic")))]
impl<S> TStr<S> {
    /// Formats a TStr
    pub const fn to_panicval(&self, fmtarg: FmtArg) -> PanicVal<'static>
    where
        Self: IsTStr,
    {
        const_panic::StdWrapper(crate::to_str(*self)).to_panicval(fmtarg)
    }

    /// Formats a TStr
    pub const fn to_panicvals(&self, fmtarg: FmtArg) -> [PanicVal<'static>; 1]
    where
        Self: IsTStr,
    {
        [self.to_panicval(fmtarg)]
    }
}
