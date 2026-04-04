use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[cfg(feature = "const_panic")]
use const_panic::{
    PanicVal,
    fmt::{FmtArg, PanicFmt},
};

use crate::{IsTStr, TStrArg, strlike::StrLike};

/// A type-level string type, emulates a `&'static str` const parameter.
///
/// This type is zero-sized and has an alignment of 1.
///
/// # Examples
///
/// ### Emulating named parameters
///
/// This example demonstrates how you can use `TStr` to emulate
/// functions with named parameters overloaded by the name of the parameters.
///
/// ```rust
/// use tstr::{IsTStr, TS, TStr, ts};
///
/// use std::{collections::HashMap, hash::RandomState};
///
/// {
///     // equivalent to HashMap::new
///     let mut map = HashMap::make(args!());
///     assert!(map.capacity() == 0);
///     map.insert(0, "");;
/// }
/// {
///     // equivalent to HashMap::with_capacity
///     let mut map = HashMap::make(args!(capacity: 10));
///     assert!(map.capacity() >= 10);
///     map.insert(0, "");;
/// }
/// {
///     // equivalent to HashMap::with_hasher
///     let mut map = HashMap::make(args!(hasher: RandomState::new()));
///     assert!(map.capacity() == 0);
///     map.insert(0, "");;
/// }
/// {
///     // equivalent to HashMap::with_capacity_and_hasher
///     let mut map = HashMap::make(args!(capacity: 10, hasher: RandomState::new()));
///     assert!(map.capacity() >= 10);
///     map.insert(0, "");;
/// }
///
/// /// The struct that encodes a named parameter by taking two arguments:
/// //  - a `TStr` that encodes the name at the type level
/// /// - the type of the argument.
/// ///
/// /// The parameter list as a whole is a tuple of this struct.
/// #[repr(transparent)]
/// pub struct NamedParameter<N: IsTStr, T> {
///     // since TStr is zero-sized, it can be put alongside the non-zero-sized
///     // wrapped value in a `#[repr(transparent)]` type.
///     name: TStr<N::Arg>,
///     pub value: T,
/// }
///
/// impl<N, T> NamedParameter<N, T>
/// where
///     N: IsTStr
/// {
///     pub const fn new(name: N, value: T) -> Self {
///         Self {
///             // `TStr::from_gen` is needed for constructing a `TStr` from a
///             // generic `IsTStr` type
///             name: TStr::from_gen(name),
///             value,
///         }
///     }
/// }
///
/// /// Custom trait for constructors.
/// trait Make<Args>: Sized {
///     fn make(args: Args) -> Self;
/// }
///
/// // make constructor with no parameters
/// impl<K, V> Make<args_ty!()> for HashMap<K, V> {
///     fn make(args_pat!{}: args_ty!()) -> Self {
///         HashMap::new()
///     }
/// }
///
/// // make constructor with a capacity parameter
/// impl<K, V> Make<args_ty!(capacity: usize)> for HashMap<K, V> {
///     fn make(args_pat!{capacity}: args_ty!(capacity: usize)) -> Self {
///         HashMap::with_capacity(capacity)
///     }
/// }
///
/// // make constructor with a hasher parameter
/// impl<K, V, S> Make<args_ty!(hasher: S)> for HashMap<K, V, S> {
///     fn make(args_pat!{hasher}: args_ty!(hasher: S)) -> Self {
///         HashMap::with_hasher(hasher)
///     }
/// }
///
/// // make constructor with capacity and hasher parameters
/// impl<K, V, S> Make<args_ty!(capacity: usize, hasher: S)> for HashMap<K, V, S> {
///     fn make(args_pat!{capacity, hasher}: args_ty!(capacity: usize, hasher: S)) -> Self {
///         HashMap::with_capacity_and_hasher(capacity, hasher)
///     }
/// }
///
/// macro_rules! args {
///     ($($name:ident: $val:expr),* $(,)?) => (
///         ($(NamedParameter::new(ts!($name), $val),)*)
///     )
/// } use args;
///
/// macro_rules! args_ty {
///     ($($name:ident: $field_ty:ty),* $(,)?) => (
///         ($(NamedParameter<TS!($name), $field_ty>,)*)
///     )
/// } use args_ty;
///
/// macro_rules! args_pat {
///     ($($name:ident),* $(,)?) => (
///         ($( NamedParameter::<TS!($name), _> { value: $name, .. }, )*)
///     )
/// } use args_pat;
///
/// ```
///
/// ### Parsing integers
///
/// Parsing integers from `TStr`s, since the primitive integers all have
/// [`const fn from_str_radix`](u32::from_str_radix) functions,
/// parsing them doesn't require direct support from `TStr` itself.
///
/// (this example requires the `"const_panic"` feature because it uses `tstr::unwrap`)
#[cfg_attr(not(feature = "const_panic"), doc = "```ignore")]
#[cfg_attr(feature = "const_panic", doc = "```rust")]
/// use tstr::ts;
///
/// // parses the number at compile-time!
/// const NUMBER: u32 = tstr::unwrap!(u32::from_str_radix(tstr::to_str(ts!(1234)), 10));
///
/// assert_eq!(NUMBER, 1234u32);
/// ```
///
/// # Serde
///
/// `TStr` implements `serde::{Serialize, Deserialize}` when  the `"serde"` feature is enabled.
///
///
pub struct TStr<S>(#[doc(hidden)] pub PhantomData<fn() -> S>);

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
    ///     // since TStr is zero-sized, it can be put in `#[repr(transparent)]` types
    ///     // next to the wrapped non-Zero-Sized-Type.
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

// TStr<_> == (str|TStr<_>)
impl<S, S2> PartialEq<S2> for TStr<S>
where
    S: TStrArg,
    S2: ?Sized + StrLike,
{
    #[inline(always)]
    fn eq(&self, other: &S2) -> bool {
        <str as PartialEq>::eq(self.as_str(), other.as_str())
    }
}

impl<S> PartialEq<&str> for TStr<S>
where
    S: TStrArg,
{
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        <str as PartialEq>::eq(self.as_str(), other)
    }
}

impl<S> PartialEq<&&str> for TStr<S>
where
    S: TStrArg,
{
    #[inline(always)]
    fn eq(&self, other: &&&str) -> bool {
        <str as PartialEq>::eq(self.as_str(), other)
    }
}

impl<S2> PartialEq<TStr<S2>> for str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn eq(&self, other: &TStr<S2>) -> bool {
        <str as PartialEq>::eq(self, other.as_str())
    }
}

impl<S2> PartialEq<TStr<S2>> for &str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn eq(&self, other: &TStr<S2>) -> bool {
        <str as PartialEq>::eq(self, other.as_str())
    }
}

impl<S2> PartialEq<TStr<S2>> for &&str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn eq(&self, other: &TStr<S2>) -> bool {
        <str as PartialEq>::eq(self, other.as_str())
    }
}

impl<S> Eq for TStr<S> where S: TStrArg {}

// comparing TStr<_> and (str|TStr<_>) for ordering
impl<S, S2> PartialOrd<S2> for TStr<S>
where
    S: TStrArg,
    S2: ?Sized + StrLike,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &S2) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self.as_str(), other.as_str())
    }
}

impl<S> PartialOrd<&str> for TStr<S>
where
    S: TStrArg,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self.as_str(), other)
    }
}

impl<S> PartialOrd<&&str> for TStr<S>
where
    S: TStrArg,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &&&str) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self.as_str(), other)
    }
}

impl<S2> PartialOrd<TStr<S2>> for str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &TStr<S2>) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self, other.as_str())
    }
}

impl<S2> PartialOrd<TStr<S2>> for &str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &TStr<S2>) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self, other.as_str())
    }
}
impl<S2> PartialOrd<TStr<S2>> for &&str
where
    S2: TStrArg,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &TStr<S2>) -> Option<Ordering> {
        <str as PartialOrd>::partial_cmp(self, other.as_str())
    }
}

impl<S> Ord for TStr<S>
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
    /// [`const_panic`]-based-formatting of `TStr`
    pub const fn to_panicval(&self, fmtarg: FmtArg) -> PanicVal<'static>
    where
        S: TStrArg,
    {
        const_panic::StdWrapper(crate::to_str(*self)).to_panicval(fmtarg)
    }

    /// [`const_panic`]-based-formatting of `TStr`
    pub const fn to_panicvals(&self, fmtarg: FmtArg) -> [PanicVal<'static>; 1]
    where
        S: TStrArg,
    {
        [self.to_panicval(fmtarg)]
    }
}
