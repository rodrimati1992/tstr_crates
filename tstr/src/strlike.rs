//! Abstractions over `&str` and `TStr<_>`

use crate::{IsTStr, TStr};

use typewit::{Identity, TypeEq};

#[doc(hidden)]
pub trait __StrLikeBase {
    #[doc(hidden)]
    type __Kind;
}

/// Trait for "`str` or `TStr<_>`", used as a bound by a few `tstr` items.
///
/// This trait is sealed and cannot be implemented outside of the `tstr` crate.
///
/// # Example
///
/// ```rust
/// use tstr::{strlike::StrLike, assert_str_eq, ts};
///
/// const {
///     assert_str_eq!(assert_valid("hello"), "hello");
///     assert_str_eq!(assert_valid(&ts!(world)), "world");
/// }
///
/// const fn assert_valid(s: &(impl StrLike + ?Sized)) -> &str {
///     let ret = tstr::strlike::as_str!(s);
///     
///     assert!(matches!(ret.as_bytes(), b"hello" | b"world"));
///     
///     ret
/// }
/// ```
///
pub trait StrLike: __AsStrLike<Target = Self> + __StrLikeBase<__Kind = __StrKind> {
    #[doc(hidden)]
    // needed for __StrLikeWitness to have a type that impls IsTStr,
    // `&str` and `&&str` use a dummy associated type
    type __TStr: IsTStr;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self>;

    /// Converts `self` into a `&str`
    ///
    /// The const equivalent of this trait method is the
    /// [`as_str`](macro@crate::strlike::as_str) macro.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{strlike::StrLike, ts};
    ///
    /// assert_eq!("foo".as_str(), "foo");
    /// assert_eq!(ts!(bar).as_str(), "bar");
    /// ```
    ///
    fn as_str(&self) -> &str {
        as_str!(self)
    }
}

#[doc(hidden)]
pub enum __StrKind {}

#[doc(hidden)]
pub enum __RefKind {}

impl<T: ?Sized> __StrLikeBase for &T {
    type __Kind = __RefKind;
}

//////

impl __StrLikeBase for str {
    type __Kind = __StrKind;
}

impl StrLike for str {
    #[doc(hidden)]
    type __TStr = crate::__Empty;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self> = __StrLikeWitness::Str(TypeEq::NEW);
}

//////

impl<S: crate::TStrArg> __StrLikeBase for TStr<S> {
    type __Kind = __StrKind;
}

impl<S: crate::IsTStr> StrLike for S {
    #[doc(hidden)]
    type __TStr = S;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self> =
        __StrLikeWitness::TStr(<S as Identity>::TYPE_EQ);
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub trait __AsStrLikeHelper {
    #[doc(hidden)]
    type Target: StrLike + ?Sized;
}

// Gets the [`StrLike`] type that `Self` is or points to.
//
// This trait powers the `as_str` macro,
// and uses a hidden trait to have these impls:
// - `impl __AsStrLike for str { type Target = str; }`
// - `impl<S: IsTStr> __AsStrLike for S { type Target = S; }`
// - `impl<T: ?Sized + __AsStrLike> __AsStrLike for &T { type Target = T::Target; }`
//
// The hidden trait is a necessary workaround, because `&T` impls
// are considered to overlap with blanket impls
// (even if the trait can't be implemented for references).
#[doc(hidden)]
pub trait __AsStrLike {
    // The [`StrLike`] type that `Self` is/points to.
    #[doc(hidden)]
    type Target: StrLike + ?Sized;
}

impl<T: ?Sized + __AsStrLike> __AsStrLikeHelper for (__RefKind, &T) {
    type Target = T::Target;
}

impl __AsStrLikeHelper for (__StrKind, str) {
    type Target = str;
}

impl<S: IsTStr> __AsStrLikeHelper for (__StrKind, S) {
    type Target = S;
}

impl<T> __AsStrLike for T
where
    T: ?Sized + __StrLikeBase,
    (<T as __StrLikeBase>::__Kind, T): __AsStrLikeHelper,
{
    type Target = <(<T as __StrLikeBase>::__Kind, T) as __AsStrLikeHelper>::Target;
}

/// Macro version of [`StrLike::as_str`], which can be used in `const`.
///
/// # Example
///
/// ```rust
/// use tstr::{strlike::as_str, ts};
///
/// // accepts `str` with any amount of layers of references
/// assert_eq!(const { as_str!("foo") }, "foo");
/// assert_eq!(const { as_str!(&"bar") }, "bar");
/// assert_eq!(const { as_str!(&&"baz") }, "baz");
/// assert_eq!(const { as_str!(&&&"qux") }, "qux");
///
///
/// // accepts `TStr` with any amount of layers of references
/// assert_eq!(const { as_str!(ts!(foo)) }, "foo");
/// assert_eq!(const { as_str!(&ts!(bar)) }, "bar");
/// assert_eq!(const { as_str!(&&ts!(baz)) }, "baz");
///
/// ```
#[doc(inline)]
pub use crate::__as_str as as_str;

#[doc(hidden)]
#[macro_export]
macro_rules! __as_str {
    ($reff:expr) => {
        match &$reff {
            reff => $crate::strlike::__as_str_fn(reff, reff),
        }
    };
}

type ACToTstr<T> = TStr<<<T as StrLike>::__TStr as IsTStr>::Arg>;

#[doc(hidden)]
pub enum __StrLikeWitness<T: ?Sized + StrLike> {
    Str(TypeEq<T, str>),
    TStr(TypeEq<T, ACToTstr<T>>),
}

#[doc(hidden)]
pub const fn __as_str_fn<'a, D: ?Sized + __AsStrLike>(_: &D, this: &'a D::Target) -> &'a str {
    match <D::Target>::__STR_LIKE_WITNESS {
        __StrLikeWitness::Str(te) => te.in_ref().to_right(this),
        __StrLikeWitness::TStr(te) => crate::to_str(*te.in_ref().to_right(this)),
    }
}
