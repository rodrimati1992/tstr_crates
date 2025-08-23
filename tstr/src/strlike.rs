//! Abstractions over `&str` and `TStr<_>`

use crate::{IsTStr, TStr};

use typewit::{Identity, TypeEq};

/// Trait for "`str` or `TStr<_>`", used as a bound by a few `tstr` items.
///
/// This trait is sealed and cannot be implemented outside of the `tstr` crate.
///
/// # Example
///
/// ```rust
/// use tstr::{strlike::StrLike, assertc_eq, ts};
///
/// const {
///     assertc_eq!(assert_valid("hello"), "hello");
///     assertc_eq!(assert_valid(&ts!(world)), "world");
/// }
///
/// const fn assert_valid(s: &(impl StrLike + ?Sized)) -> &str {
///     let ret = tstr::strlike::as_str(s);
///     
///     assert!(matches!(ret.as_bytes(), b"hello" | b"world"));
///     
///     ret
/// }
/// ```
///
pub trait StrLike: AsStrLike<Target = Self> {
    #[doc(hidden)]
    // needed for __StrLikeWitness to have a type that impls IsTStr,
    // `&str` and `&&str` use a dummy associated type
    type __TStr: IsTStr;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self>;
}

impl StrLike for str {
    #[doc(hidden)]
    type __TStr = crate::__Empty;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self> = __StrLikeWitness::Str(TypeEq::NEW);
}

impl<S: crate::TStrArg> StrLike for TStr<S> {
    #[doc(hidden)]
    type __TStr = TStr<S>;

    #[doc(hidden)]
    const __STR_LIKE_WITNESS: __StrLikeWitness<Self> =
        __StrLikeWitness::TStr(<TStr<S> as Identity>::TYPE_EQ);
}

type ACToTstr<T> = TStr<<<T as StrLike>::__TStr as IsTStr>::Arg>;

#[doc(hidden)]
pub enum __StrLikeWitness<T: ?Sized + StrLike> {
    Str(TypeEq<T, str>),
    TStr(TypeEq<T, ACToTstr<T>>),
}

/// Converts the argument into a `&str`
///
/// # Example
///
/// ```rust
/// use tstr::{strlike, ts};
///
/// assert_eq!(STRS, ["foo", "bar"]);
///
/// const STRS: [&str; 2] = [
///     strlike::as_str("foo"),
///     strlike::as_str(&ts!(bar)),
/// ];
///
/// ```
///
pub const fn as_str<A>(this: &A) -> &str
where
    A: StrLike + ?Sized,
{
    match A::__STR_LIKE_WITNESS {
        __StrLikeWitness::Str(te) => te.in_ref().to_right(this),
        __StrLikeWitness::TStr(te) => crate::to_str(*te.in_ref().to_right(this)),
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Gets the [`StrLike`] type that `Self` is or points to.
///
/// This trait powers the [`as_strlike`] macro.
pub trait AsStrLike {
    /// The [`StrLike`] type that `Self` is/points to.
    type Target: StrLike + ?Sized;
}

impl<T: ?Sized + AsStrLike> AsStrLike for &T {
    type Target = T::Target;
}
impl AsStrLike for str {
    type Target = Self;
}
impl<S: crate::TStrArg> AsStrLike for TStr<S> {
    type Target = Self;
}

/// Coerces an `impl `[`AsStrLike`] argument to `&impl `[`StrLike`].
///
/// # Example
///
/// ```rust
/// use tstr::{strlike::{as_str, as_strlike}, ts};
///
/// // this doesn't compile because `&str` doesn't impl `StrLike`, `str` does.
/// // assert_eq!(as_str(&"hello"), "hello");
///
/// // coerces the argument to `&str` from any amount of layers of references
/// assert_eq!(as_str(as_strlike!("foo")), "foo");
/// assert_eq!(as_str(as_strlike!(&"bar")), "bar");
/// assert_eq!(as_str(as_strlike!(&&"baz")), "baz");
/// assert_eq!(as_str(as_strlike!(&&&"qux")), "qux");
///
///
/// // this doesn't compile because `&TStr<_>` doesn't impl `StrLike`, `TStr<_>` does.
/// // assert_eq!(as_str(&&ts!(hello)), "hello");
///
/// // coerces the argument to `&TStr<_>` from any amount of layers of references
/// assert_eq!(as_str(as_strlike!(ts!(foo))), "foo");
/// assert_eq!(as_str(as_strlike!(&ts!(bar))), "bar");
/// assert_eq!(as_str(as_strlike!(&&ts!(baz))), "baz");
///
/// ```
#[doc(inline)]
pub use crate::__as_strlike as as_strlike;

#[doc(hidden)]
#[macro_export]
macro_rules! __as_strlike {
    ($reff:expr) => {
        match &$reff {
            reff => $crate::strlike::__as_strlike_func(reff, reff),
        }
    };
}

#[doc(hidden)]
pub const fn __as_strlike_func<'a, D: ?Sized + AsStrLike>(
    _: &D,
    reff: &'a D::Target,
) -> &'a D::Target {
    reff
}
