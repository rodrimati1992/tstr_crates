use crate::{__TStrRepr, TStr};

use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{Debug, Display},
    hash::Hash,
};

use typewit::Identity;

/// Trait for generic [`TStr`]s.
pub trait IsTStr:
    Identity<Type = TStr<<Self as IsTStr>::Arg>>
    + 'static
    + crate::strlike::StrLike<__TStr = Self>
    + Copy
    + Clone
    + Debug
    + Display
    + Default
    + Hash
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Send
    + Sized
    + Sync
    + core::marker::Unpin
{
    /// The type parameter of `TStr`
    type Arg: TStrArg;

    /// Constructs a TStr.
    const VAL: Self;

    /// The length of this string when encoded to utf8
    const LENGTH: usize;

    /// This string converted to a uf8-encoded byte slice
    const BYTES: &[u8];

    /// This string converted to a string
    const STR: &str;

    /// Coerces `Self` to `TStr<Self::Arg>`, only necessary in generic contexts
    ///
    /// The const equivalent of this trait method is the
    /// [`TStr::from_gen`](crate::TStr::from_gen) constructor.
    ///
    /// While it's always possible to construct a `TStr` through its
    /// [`new`](crate::TStr::new) constructor,
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
    ///     pub fn new(val: T, tstr: N) -> Self {
    ///         Self{ val, name: tstr.to_tstr() }
    ///     }
    /// }
    /// ```
    ///
    fn to_tstr(self) -> TStr<Self::Arg> {
        <Self as Identity>::TYPE_EQ.to_right(self)
    }

    /// Coerces a `TStr` into `Self`, only necessary in generic contexts.
    ///
    /// The const equivalent of this trait method is the
    /// [`TStr::to_gen`](crate::TStr::to_gen) method.
    ///
    /// While it's always possible to construct `Self` through the
    /// [`VAL`](crate::IsTStr::VAL) associated constant,
    /// this function ensures that it's the same string as the argument.
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
    ///     fn name(&self) -> N {
    ///         N::from_tstr(self.name)
    ///     }
    /// }
    /// ```
    ///
    fn from_tstr(tstr: TStr<Self::Arg>) -> Self {
        <Self as Identity>::TYPE_EQ.to_left(tstr)
    }

    /// Gets the length of the string in utf8
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::len`](crate::len) function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, ts};
    ///
    /// assert!(ts!(4).len() == 1);
    ///
    /// assert!(ts!("hello").len() == 5);
    ///
    /// assert!(ts!(rustacean).len() == 9);
    ///
    /// ```
    fn len(self) -> usize {
        Self::LENGTH
    }

    /// Gets the `&'static str` equivalent of this [`TStr`]
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::to_str`](crate::to_str) function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, TStr, ts};
    ///
    /// let foo: TStr<_> = ts!(foo);
    /// assert_eq!(foo.to_str(), "foo");
    ///
    /// let bar_str: &str = ts!("bar").to_str();
    /// assert_eq!(bar_str, "bar");
    ///
    /// ```
    ///
    fn to_str(self) -> &'static str {
        Self::STR
    }

    /// Gets the `&'static [u8]` equivalent of this [`TStr`]
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::to_bytes`](crate::to_bytes) function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, TStr, ts};
    ///
    /// let foo: TStr<_> = ts!(foo);
    /// assert_eq!(foo.to_bytes(), "foo".as_bytes());
    ///
    /// let bar_str: &[u8] = ts!("bar").to_bytes();
    /// assert_eq!(bar_str, "bar".as_bytes());
    ///
    /// ```
    ///
    fn to_bytes(self) -> &'static [u8] {
        Self::BYTES
    }

    /// Compares two [`TStr`]s for equality
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::eq`](crate::eq) function.
    ///
    /// This method exists to allow comparing any `TStr` to any other,
    /// because the `for<Rhs: IsTStr> PartialEq<Rhs>` supertrait can't be written on stable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::{IsTStr, ts};
    ///
    /// assert!( ts!("foo").tstr_eq(ts!("foo")));
    ///
    /// assert!(!ts!("foo").tstr_eq(ts!("bar")));
    ///
    /// ```
    ///
    fn tstr_eq<Rhs: IsTStr>(self, rhs: Rhs) -> bool {
        crate::eq(self, rhs)
    }

    /// Compares two [`TStr`]s for inequality
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::ne`](crate::ne) function.
    ///
    /// This method exists to allow comparing any `TStr` to any other,
    /// because the `for<Rhs: IsTStr> PartialEq<Rhs>` supertrait can't be written on stable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::{IsTStr, ts};
    ///
    /// assert!(!ts!("foo").tstr_ne(ts!("foo")));
    ///
    /// assert!( ts!("foo").tstr_ne(ts!("bar")));
    ///
    /// ```
    ///
    fn tstr_ne<Rhs: IsTStr>(self, rhs: Rhs) -> bool {
        crate::ne(self, rhs)
    }

    /// Compares two [`TStr`]s for ordering
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::cmp`](crate::cmp) function.
    ///
    /// This method exists to allow comparing any `TStr` to any other,
    /// because the `for<Rhs: IsTStr> PartialOrd<Rhs>` supertrait can't be written on stable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tstr::{IsTStr, ts};
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(ts!("foo").tstr_cmp(ts!("foo")), Ordering::Equal);
    ///
    /// assert_eq!(ts!("foo").tstr_cmp(ts!("bar")), Ordering::Greater);
    ///
    /// assert_eq!(ts!("bar").tstr_cmp(ts!("foo")), Ordering::Less);
    ///
    /// ```
    ///
    fn tstr_cmp<Rhs: IsTStr>(self, rhs: Rhs) -> Ordering {
        crate::cmp(self, rhs)
    }

    /// Compares two [`TStr`]s for equality,
    /// returning a proof of (in)equality of `Self` and `Rhs`
    ///
    /// The const equivalent of this trait is the
    /// [`tstr::type_eq`](crate::type_eq) function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{IsTStr, TStr, TS};
    /// use std::marker::PhantomData as PD;
    ///
    ///
    /// assert_eq!(typecast_arg(Guess::<TS!(bar)>(PD)), Err(Guess::<TS!(bar)>(PD)));
    /// assert_eq!(typecast_arg(Guess::<TS!(bar)>(PD)), Err(Guess::<TS!(bar)>(PD)));
    ///
    /// assert_eq!(typecast_arg(Guess::<Answer>(PD)), Ok(Guess::<Answer>(PD)));
    ///
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Guess<S>(PD<S>);
    ///
    /// type Answer = TS!(hello);
    ///
    /// fn typecast_arg<S>(guess: Guess<S>) -> Result<Guess<Answer>, Guess<S>>
    /// where
    ///     S: IsTStr
    /// {
    ///     match S::VAL.type_eq(Answer::new()).eq() {
    ///         Some(te) => Ok(te.map(GuessFn).to_right(guess)),
    ///         None => Err(guess),
    ///     }
    /// }
    ///
    /// tstr::typewit::type_fn!{
    ///     // type-level function from any `S` to `Guess<S>`
    ///     struct GuessFn;
    ///     impl<S> S => Guess<S>
    /// }
    ///
    /// ```
    ///
    fn type_eq<Rhs: IsTStr>(self, rhs: Rhs) -> typewit::TypeCmp<Self, Rhs> {
        crate::type_eq(self, rhs)
    }
}

impl<S> IsTStr for TStr<S>
where
    S: TStrArg,
{
    type Arg = S;

    const VAL: Self = Self::new();

    const LENGTH: usize = S::__LENGTH;

    const BYTES: &[u8] = S::__BYTES;

    const STR: &str = S::__STR;
}

/// For bounding the type parameter of [`TStr`].
///
/// You only need this trait if you're using using `TStr` explicitly in the code,
/// it's usually better have a type parameter bounded by
/// the [`IsTStr`] trait instead of using `TStr` directly.
///
/// # Example
///
/// This example shows a usecase where you'll need to use this trait,
/// implementing traits for `TStr`.
///
/// ```rust
/// use tstr::{IsTStr, TStr, TStrArg, ts};
///
/// assert_eq!("hello".my_as_str(), "hello");
/// assert_eq!(ts!(world).my_as_str(), "world");
///
///
/// trait MyAsStr {
///     fn my_as_str(&self) -> &str;
/// }
///
/// impl MyAsStr for &str {
///     fn my_as_str(&self) -> &str { self }
/// }
///
/// impl<S: TStrArg> MyAsStr for TStr<S> {
///     fn my_as_str(&self) -> &str { self.to_str() }
/// }
/// ```
///
pub trait TStrArg: __TStrRepr + 'static {
    /// Implementation detail
    #[doc(hidden)]
    const __LENGTH: usize;

    /// Implementation detail
    #[doc(hidden)]
    const __BYTES: &[u8];

    /// Implementation detail
    #[doc(hidden)]
    const __STR: &str;

    /// Implementation detail
    #[doc(hidden)]
    type __WithRhs<Rhs: TStrArg>: __TStrArgBinary<Lhs = Self, Rhs = Rhs>;

    /// Implementation detail
    #[cfg(feature = "str_generics")]
    #[doc(hidden)]
    type __WithLhsArgs<const LEFT_S: &'static str>: __TStrArgBinary<Lhs = crate::___<LEFT_S>, Rhs = Self>;

    /// Implementation detail
    #[cfg(not(feature = "str_generics"))]
    #[doc(hidden)]
    type __WithLhsArgs<LeftS: __TStrRepr, const LEFT_LEN: usize>: __TStrArgBinary<Lhs = crate::___<LeftS, LEFT_LEN>, Rhs = Self>;
}

// implemented for `(Lhs, Rhs)`, does binary operations on a pair of type arguments of TStrs
#[doc(hidden)]
pub trait __TStrArgBinary {
    #[doc(hidden)]
    type Lhs: __TStrRepr;

    #[doc(hidden)]
    type Rhs: __TStrRepr;

    #[doc(hidden)]
    const __EQ: bool;

    #[doc(hidden)]
    const __CMP: core::cmp::Ordering;

    #[doc(hidden)]
    const __TYPE_CMP: typewit::TypeCmp<crate::TStr<Self::Lhs>, crate::TStr<Self::Rhs>>;
}

pub(crate) type __ToTStrArgBinary<L, R> = <L as TStrArg>::__WithRhs<R>;

typewit::inj_type_fn! {
    pub(crate) struct TStrFn;

    impl<S> S => crate::TStr<S>;
}
