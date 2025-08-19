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

use crate::{__TStrArgBinary, IsTStr};

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
pub struct TStr<S>(pub(crate) PhantomData<fn() -> S>);

impl<S> TStr<S> {
    /// Constructs the TStr.
    pub const fn new() -> Self {
        TStr(PhantomData)
    }
}

impl<S> TStr<S> {
    /// Gets the length of the string in utf8
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::ts;
    ///
    /// const _: () = assert!(ts!(4).len() == 1);
    ///
    /// const _: () = assert!(ts!("hello").len() == 5);
    ///
    /// const _: () = assert!(ts!(rustacean).len() == 9);
    ///
    /// ```
    pub const fn len(self) -> usize
    where
        Self: IsTStr,
    {
        <Self as IsTStr>::LENGTH
    }

    /// Gets the `&'static str` equivalent of this TStr
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{TStr, ts};
    ///
    /// let foo: TStr<_> = ts!(foo);
    /// assert_eq!(foo.to_str(), "foo");
    ///
    /// const BAR_STR: &str = ts!("bar").to_str();
    /// assert_eq!(BAR_STR, "bar");
    ///
    /// ```
    ///
    pub const fn to_str(self) -> &'static str
    where
        Self: IsTStr,
    {
        Self::STR
    }

    /// Gets the `&'static [u8]` equivalent of this TStr
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{TStr, ts};
    ///
    /// let foo: TStr<_> = ts!(foo);
    /// assert_eq!(foo.to_bytes(), "foo".as_bytes());
    ///
    /// const BAR_STR: &[u8] = ts!("bar").to_bytes();
    /// assert_eq!(BAR_STR, "bar".as_bytes());
    ///
    /// ```
    ///
    pub const fn to_bytes(self) -> &'static [u8]
    where
        Self: IsTStr,
    {
        Self::BYTES
    }

    /// Compares two `TStr`s for equality
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::ts;
    ///
    /// const _: () = assert!( ts!("foo").const_eq(ts!("foo")));
    ///
    /// const _: () = assert!(!ts!("foo").const_eq(ts!("bar")));
    ///
    /// ```
    ///
    pub const fn const_eq<S2>(self, _: S2) -> bool
    where
        Self: IsTStr,
        S2: IsTStr,
    {
        crate::tstr_trait::__ToTStrArgBinary::<<Self as IsTStr>::Arg, S2::Arg>::__EQ
    }

    /// Compares two `TStr`s for inequality
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::ts;
    ///
    /// const _: () = assert!(!ts!("foo").const_ne(ts!("foo")));
    ///
    /// const _: () = assert!( ts!("foo").const_ne(ts!("bar")));
    ///
    /// ```
    ///
    pub const fn const_ne<S2>(self, _: S2) -> bool
    where
        Self: IsTStr,
        S2: IsTStr,
    {
        !crate::tstr_trait::__ToTStrArgBinary::<<Self as IsTStr>::Arg, S2::Arg>::__EQ
    }

    /// Compares two `TStr`s for ordering
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::ts;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(const { ts!("foo").const_cmp(ts!("foo")) }, Ordering::Equal);
    ///
    /// assert_eq!(const { ts!("foo").const_cmp(ts!("bar")) }, Ordering::Greater);
    ///
    /// assert_eq!(const { ts!("bar").const_cmp(ts!("foo")) }, Ordering::Less);
    ///
    /// ```
    ///
    pub const fn const_cmp<R>(self, _: R) -> Ordering
    where
        Self: IsTStr,
        R: IsTStr,
    {
        crate::tstr_trait::__ToTStrArgBinary::<<Self as IsTStr>::Arg, R::Arg>::__CMP
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
        self.const_eq(*other)
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
        Some(self.const_cmp(*other))
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
        const_panic::StdWrapper(self.to_str()).to_panicval(fmtarg)
    }

    /// Formats a TStr
    pub const fn to_panicvals(&self, fmtarg: FmtArg) -> [PanicVal<'static>; 1]
    where
        Self: IsTStr,
    {
        [self.to_panicval(fmtarg)]
    }
}
