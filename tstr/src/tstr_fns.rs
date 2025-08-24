use crate::tstr_trait::__ToTStrArgBinary;
use crate::{__TStrArgBinary, IsTStr};

use typewit::Identity;

/// Gets the length of the [`TStr`](crate::TStr) in utf8
///
/// The trait method equivalent of this const function is [`IsTStr::len`]
///
/// # Example
///
/// ```rust
/// use tstr::ts;
///
/// const _: () = assert!(tstr::len(ts!(4)) == 1);
///
/// const _: () = assert!(tstr::len(ts!("hello")) == 5);
///
/// const _: () = assert!(tstr::len(ts!(rustacean)) == 9);
///
/// ```
pub const fn len<S: IsTStr>(_: S) -> usize {
    S::LENGTH
}

/// Converts a [`TStr`](crate::TStr) to a `&'static str`
///
/// The trait method equivalent of this const function is [`IsTStr::to_str`]
///
/// # Example
///
/// ```rust
/// use tstr::{TStr, ts};
///
/// let foo: TStr<_> = ts!(foo);
/// assert_eq!(tstr::to_str(foo), "foo");
///
/// const BAR_STR: &str = tstr::to_str(ts!("bar"));
/// assert_eq!(BAR_STR, "bar");
///
/// ```
///
pub const fn to_str<S: IsTStr>(_: S) -> &'static str {
    S::STR
}

/// Converts a [`TStr`](crate::TStr) to a `&'static [u8]`
///
/// The trait method equivalent of this const function is [`IsTStr::to_bytes`]
///
/// # Example
///
/// ```rust
/// use tstr::{TStr, ts};
///
/// let foo: TStr<_> = ts!(foo);
/// assert_eq!(tstr::to_bytes(foo), "foo".as_bytes());
///
/// const BAR_STR: &[u8] = tstr::to_bytes(ts!("bar"));
/// assert_eq!(BAR_STR, "bar".as_bytes());
///
/// ```
///
pub const fn to_bytes<S: IsTStr>(_: S) -> &'static [u8] {
    S::BYTES
}

/// Compares two [`TStr`](crate::TStr)s for equality
///
/// The trait method equivalent of this const function is [`IsTStr::tstr_eq`]
///
/// # Examples
///
/// ```rust
/// use tstr::ts;
///
/// const _: () = assert!( tstr::eq(ts!("foo"), ts!("foo")));
///
/// const _: () = assert!(!tstr::eq(ts!("foo"), ts!("bar")));
///
/// ```
///
pub const fn eq<Lhs, Rhs>(_: Lhs, _: Rhs) -> bool
where
    Lhs: IsTStr,
    Rhs: IsTStr,
{
    __ToTStrArgBinary::<Lhs::Arg, Rhs::Arg>::__EQ
}

/// Compares two [`TStr`](crate::TStr)s for inequality
///
/// The trait method equivalent of this const function is [`IsTStr::tstr_ne`]
///
/// # Examples
///
/// ```rust
/// use tstr::ts;
///
/// const _: () = assert!(!tstr::ne(ts!("foo"), ts!("foo")));
///
/// const _: () = assert!( tstr::ne(ts!("foo"), ts!("bar")));
///
/// ```
///
pub const fn ne<Lhs, Rhs>(_: Lhs, _: Rhs) -> bool
where
    Lhs: IsTStr,
    Rhs: IsTStr,
{
    !__ToTStrArgBinary::<Lhs::Arg, Rhs::Arg>::__EQ
}

/// Compares two [`TStr`](crate::TStr)s for ordering
///
/// The trait method equivalent of this const function is [`IsTStr::tstr_cmp`]
///
/// # Examples
///
/// ```rust
/// use tstr::ts;
/// use core::cmp::Ordering;
///
/// assert_eq!(const { tstr::cmp(ts!("foo"), ts!("foo")) }, Ordering::Equal);
///
/// assert_eq!(const { tstr::cmp(ts!("foo"), ts!("bar")) }, Ordering::Greater);
///
/// assert_eq!(const { tstr::cmp(ts!("bar"), ts!("foo")) }, Ordering::Less);
///
/// ```
///
pub const fn cmp<Lhs, Rhs>(_: Lhs, _: Rhs) -> core::cmp::Ordering
where
    Lhs: IsTStr,
    Rhs: IsTStr,
{
    __ToTStrArgBinary::<Lhs::Arg, Rhs::Arg>::__CMP
}

/// Compares two [`TStr`](crate::TStr)s for equality,
/// returning a proof of (in)equality of the arguments.
///
/// The trait method equivalent of this const function is [`IsTStr::type_eq`]
///
/// # Example
///
/// ```rust
/// use tstr::{IsTStr, TS};
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
/// const fn typecast_arg<S: IsTStr>(guess: Guess<S>) -> Result<Guess<Answer>, Guess<S>> {
///     match tstr::type_eq(S::VAL, Answer::VAL).eq() {
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
pub const fn type_eq<Lhs, Rhs>(_: Lhs, _: Rhs) -> typewit::TypeCmp<Lhs, Rhs>
where
    Lhs: IsTStr,
    Rhs: IsTStr,
{
    const {
        __ToTStrArgBinary::<Lhs::Arg, Rhs::Arg>::__TYPE_CMP
            .join_left(<Lhs as Identity>::TYPE_EQ)
            .join_right(<Rhs as Identity>::TYPE_EQ.flip())
    }
}
