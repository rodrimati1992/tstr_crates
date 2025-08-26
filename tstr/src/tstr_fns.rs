use crate::tstr_trait::__ToTStrArgBinary;
use crate::{__TStrArgBinary, IsTStr};

use typewit::Identity;

/// Gets the length of the [`IsTStr`] argument in utf8
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::len`]
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

/// Converts an [`IsTStr`] to a `&'static str`
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::to_str`]
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

/// Converts an  [`IsTStr`] to an utf8-encoded `&'static [u8]`
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::to_bytes`]
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

/// Compares two [`IsTStr`]s for equality
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::tstr_eq`]
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

/// Compares two [`IsTStr`]s for inequality
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::tstr_ne`]
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

/// Compares two [`IsTStr`]s for ordering
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::tstr_cmp`]
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

/// Compares two [`IsTStr`]s for equality,
/// returning a proof of the (in)equality of the arguments.
///
/// This is a non-associated function for `const` compatibility,
/// the (non-`const`) trait method equivalent of this is [`IsTStr::type_eq`]
///
/// # Example
///
/// ```rust
/// use tstr::{IsTStr, TStr, TS, ts};
/// use tstr::typewit::TypeCmp;
///
///
/// assert_eq!(is_right_guess(Guess(ts!(foo))), None);
/// assert_eq!(is_right_guess(Guess(ts!(bar))), None);
/// assert_eq!(is_right_guess(Guess(ts!(world))), None);
///
/// assert!(is_right_guess(Guess(ts!(hello))).is_some_and(|x| x.0 == "hello"));
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Guess<S: IsTStr>(S);
///
/// const fn is_right_guess<S: IsTStr>(guess: Guess<S>) -> Option<Guess<impl IsTStr>> {
///     match typecast_guess(guess) {
///         Ok(ret @ Guess::<TS!(hello)>{..}) => Some(ret),
///         Err(_) => None,
///     }
/// }
///
/// /// Coerces `Guess<A>` to `Guess<B>` if `A == B`, returns `Err(guess)` if `A != B`.
/// const fn typecast_guess<A, B>(guess: Guess<A>) -> Result<Guess<B>, Guess<A>>
/// where
///     A: IsTStr,
///     B: IsTStr,
/// {
///     tstr::typewit::type_fn!{
///         // type-level function from `S` to `Guess<S>`
///         struct GuessFn;
///         impl<S: IsTStr> S => Guess<S>
///     }
///     
///     match tstr::type_eq(A::VAL, B::VAL) {
///         TypeCmp::Eq(te) => Ok(
///             // te is a `TypeEq<A, B>`, a value-level proof that both args are the same type.
///             te               
///             .map(GuessFn)    // : TypeEq<Guess<A>, Guess<B>>
///             .to_right(guess) // : Guess<B>
///         ),
///         TypeCmp::Ne(_) => Err(guess),
///     }
/// }
///
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
