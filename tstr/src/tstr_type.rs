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

use crate::{IsTStr, TStrArg};

/// A type-level string type, emulates a `&'static str` const parameter.
///
/// This type is zero-sized and has an alignment of 1.
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

// const layout assertions
const _: () = assert!(size_of::<crate::TS!("")>() == 0);
const _: () = assert!(align_of::<crate::TS!("")>() == 1);

impl<S> TStr<S> {
    /// Constructs the TStr.
    pub const fn new() -> Self {
        TStr(PhantomData)
    }
}

impl<S: TStrArg> TStr<S> {
    /// Coerces an `impl IsTStr` into a `TStr`, only necessary in generic contexts
    ///
    /// The trait method equivalent of this const function is
    /// [`IsTStr::to_tstr`](crate::IsTStr::to_tstr).
    ///
    /// While it's always possible to construct a `TStr` through its
    /// [`new`](crate::TStr::new) constructor,
    /// this method ensures that it's the same string as the argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, TStr};
    ///
    /// #[repr(transparent)]
    /// struct Foo<T, N: IsTStr> {
    ///     val: T,
    ///     // since TStr is zero-sized, it can be put in `#[repr(transparent)]` types
    ///     // next to the wrapped non-Zero-Sized-Type.
    ///     name: TStr<N::Arg>,
    /// }
    ///
    /// impl<T, N: IsTStr> Foo<T, N> {
    ///     pub fn new(val: T, tstr: N) -> Self {
    ///         Self{ val, name: TStr::from_gen(tstr) }
    ///     }
    /// }
    /// ```
    ///
    pub const fn from_gen<G>(tstr: G) -> Self
    where
        G: IsTStr<Arg = S>,
    {
        <G as typewit::Identity>::TYPE_EQ.to_right(tstr)
    }
    /// Coerces a `TStr` into an `impl IsTStr`, only necessary in generic contexts
    ///
    /// While it's always possible to construct an `IsTStr` through its
    /// [`VAL`](crate::IsTStr::VAL) associated constant,
    /// this method ensures that it's the same string as `Self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, TStr};
    ///
    /// #[repr(transparent)]
    /// struct Foo<T, N: IsTStr> {
    ///     val: T,
    ///     name: TStr<N::Arg>,
    /// }
    ///
    /// impl<T, N: IsTStr> Foo<T, N> {
    ///     const fn name(&self) -> N {
    ///         self.name.to_gen()
    ///     }
    /// }
    /// ```
    ///
    pub const fn to_gen<G>(self) -> G
    where
        G: IsTStr<Arg = S>,
    {
        <G as typewit::Identity>::TYPE_EQ.to_left(self)
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
    S: TStrArg,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.to_str(), f)
    }
}

impl<S> fmt::Display for TStr<S>
where
    S: TStrArg,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.to_str(), f)
    }
}

impl<S, S2> core::cmp::PartialEq<S2> for TStr<S>
where
    S: TStrArg,
    S2: IsTStr,
{
    #[inline(always)]
    fn eq(&self, other: &S2) -> bool {
        self.tstr_eq(*other)
    }
}

impl<S> core::cmp::Eq for TStr<S> where S: TStrArg {}

impl<S, S2> core::cmp::PartialOrd<S2> for TStr<S>
where
    S: TStrArg,
    S2: IsTStr,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &S2) -> Option<Ordering> {
        Some(self.tstr_cmp(*other))
    }
}

impl<S> core::cmp::Ord for TStr<S>
where
    S: TStrArg,
{
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

// rustc expands #[derive(Hash)] on unit structs into this
impl<S> Hash for TStr<S>
where
    S: TStrArg,
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
    S: TStrArg,
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
        S: TStrArg,
    {
        const_panic::StdWrapper(crate::to_str(*self)).to_panicval(fmtarg)
    }

    /// Formats a TStr
    pub const fn to_panicvals(&self, fmtarg: FmtArg) -> [PanicVal<'static>; 1]
    where
        S: TStrArg,
    {
        [self.to_panicval(fmtarg)]
    }
}
