use typewit::{Identity, TypeEq};

use crate::{IsTStr, TStr};

macro_rules! cmp_assertc_docs {
    () => {
        concat!(
            "[**examples below**](#examples)",
            "\n\n",
            "This macro is only evaluated at compile-time if used in a context that requires it ",
            "(eg: in the expression assigned to a `const _: () = `)",
            "\n\n",
            "# Arguments\n",
            "\n",
            "The `$left` and `$right` arguments can each be (independently of the other):\n",
            "- `&str`\n",
            "- `&&str`\n",
            "- `TStr<_>`\n",
            "- `&TStr<_>`\n",
            "\n\n",
            "The rest of the arguments are formatting arguments for the panic message",
            " for when the assertion fails, ",
            "[described in the section below](#formatting)\n",
            "\n\n",
            "# Formatting ",
            "\n\n",
            "This uses the same syntax for formatting arguments as ",
            "[`const_panic::concat_panic`](macro@const_panic::concat_panic).",
            "\n\n",
            "By default, this only supports primitive types as formatting arguments, ",
            "to format arrays or custom types you must enable ",
            r#"`const_panic`'s `"non_basic"` feature."#,
            "\n\n",
            "To print user-defined types, ",
            "they must implement the [`const_panic::fmt::PanicFmt`]",
            " trait as described in its docs.\n",
            "\n\n",
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cmp_assert_inner {
    ($left:expr, $right:expr, $is_equal:ident, $operator:literal, $($($fmt:tt)+)?) => (
        match ($left, $right) {
            (left, right) => {
                let left = $crate::assertions::__coerce_assert_comparable(left);
                let right = $crate::assertions::__coerce_assert_comparable(right);

                if let $is_equal = $crate::utils::str_eq(left, right) {
                    $crate::__p::concat_panic!{
                        display: $crate::__p::concat!(
                            "\nassertion failed: LEFT ",
                            $operator,
                            " RIGHT\n left: `",
                        ),
                        left,
                        "`\nright: `",
                        right,
                        "`\n",
                        $( ": ", $($fmt)+)?
                    }
                }
            }
        }
    )
}

/// For asserting that two `&str` and/or [`TStr`] are equal.
///
#[doc = cmp_assertc_docs!()]
///
/// # Example
///
/// ### Passing
///
/// ```rust
/// use tstr::ts;
///
/// // compile-time asserts
/// const { tstr::assertc_eq!(ts!(foo), ts!("foo")) }
/// const { tstr::assertc_eq!(ts!(foo), "foo") }
/// const { tstr::assertc_eq!("foo", ts!("foo")) }
/// const { tstr::assertc_eq!("foo", "foo") }
///
/// // runtime asserts
/// tstr::assertc_eq!(ts!(foo), ts!("foo"));
/// tstr::assertc_eq!(ts!(foo), "foo");
/// tstr::assertc_eq!("foo", ts!("foo"));
/// tstr::assertc_eq!("foo", "foo");
/// ```
///
/// ### Failing
///
/// ```rust,compile_fail
/// use tstr::{IsTStr, TS};
///
/// const fn expects_bar<S: IsTStr>() {
///     const {
///         let expected = "bar";
///         tstr::assertc_eq!(
///             S::TSTR, expected,
///             "unfortunately you passed ", S::TSTR, " when ", expected, " was expected"
///         )
///     };
/// }
///
/// # fn main() {
/// expects_bar::<TS!(foo)>();
/// # }
/// ```
///
/// The above code fails with this error:
/// ```text
/// error[E0080]: evaluation panicked:
///               assertion failed: LEFT == RIGHT
///                left: `"foo"`
///               right: `"bar"`
///               : unfortunately you passed "foo" when "bar" was expected
///   --> tstr/src/assertions.rs:130:9
///    |
/// 9  | /         tstr::assertc_eq!(
/// 10 | |             S::TSTR, expected,
/// 11 | |             "unfortunately you passed ", S::TSTR, " when ", expected, " was expected"
/// 12 | |         )
///    | |_________^ evaluation of `expects_bar::<tstr::TStr<tstr::___<tstr::__<'f', 'o', 'o'>, 3>>>::{constant#0}` failed here
/// ```
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_panic")))]
macro_rules! assertc_eq {
    ($left:expr, $right:expr $(, $($fmt:tt)* )? ) => (
        $crate::__cmp_assert_inner!{$left, $right, false, "==", $($($fmt)*)?}
    );
}

/// For asserting that two `&str` and/or [`TStr`] are unequal.
///
#[doc = cmp_assertc_docs!()]
///
/// # Example
///
/// ### Passing
///
/// ```rust
/// use tstr::ts;
///
/// // compile-time asserts
/// const { tstr::assertc_ne!(ts!(foo), ts!("bar")) }
/// const { tstr::assertc_ne!(ts!(foo), "qux") }
/// const { tstr::assertc_ne!("foo", ts!("hello")) }
/// const { tstr::assertc_ne!("foo", "world") }
///
/// // runtime asserts
/// tstr::assertc_ne!(ts!(foo), ts!("bar"));
/// tstr::assertc_ne!(ts!(foo), "qux");
/// tstr::assertc_ne!("foo", ts!("hello"));
/// tstr::assertc_ne!("foo", "world");
/// ```
///
/// ### Failing
///
/// ```rust,compile_fail
/// use tstr::{IsTStr, TS};
///
/// const fn expects_not_bar<S: IsTStr>() {
///     const {
///         tstr::assertc_ne!(
///             S::TSTR, "bar",
///             "unfortunately you passed ", S::TSTR, ", the only value that isn't allowed"
///         )
///     };
/// }
///
/// # fn main() {
/// expects_not_bar::<TS!(bar)>();
/// # }
/// ```
///
/// The above code fails with this error:
/// ```text
/// error[E0080]: evaluation panicked:
///               assertion failed: LEFT != RIGHT
///                left: `"bar"`
///               right: `"bar"`
///               : unfortunately you passed "bar", the only value that isn't allowed
///   --> tstr/src/assertions.rs:196:9
///    |
/// 8  | /         tstr::assertc_ne!(
/// 9  | |             S::TSTR, "bar",
/// 10 | |             "unfortunately you passed ", S::TSTR, ", the only value that isn't allowed"
/// 11 | |         )
///    | |_________^ evaluation of `expects_not_bar::<tstr::TStr<tstr::___<tstr::__<'b', 'a', 'r'>, 3>>>::{constant#0}` failed here
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_panic")))]
#[macro_export]
macro_rules! assertc_ne {
    ($left:expr, $right:expr $(, $($fmt:tt)* )? ) => (
        $crate::__cmp_assert_inner!{$left, $right, true, "!=", $($($fmt)*)?}
    );
}

////////////////////////////////////////////////////////////////////////////////

/// Trait for the types that the `tstr` assert macros can compare.
///
/// This trait is sealed and cannot be implemented oI thought idempotence only meant that the first time an operation is done will give the same resuutside of the `tstr` crate.
pub trait AssertComparable<'a, 'b: 'a>: Sized {
    #[doc(hidden)]
    // needed for __AssertComparableWitness to have a type that impls IsTStr,
    // `&str` and `&&str` use a dummy associated type
    type __TStr: IsTStr;

    #[doc(hidden)]
    const __ASSERT_COMPARABLE_WITNESS: __AssertComparableWitness<'a, 'b, Self>;
}

impl<'a, 'b: 'a> AssertComparable<'a, 'b> for &'b str {
    type __TStr = crate::Empty;

    const __ASSERT_COMPARABLE_WITNESS: __AssertComparableWitness<'a, 'b, Self> =
        __AssertComparableWitness::Str(TypeEq::NEW);
}

impl<'a, 'b: 'a> AssertComparable<'a, 'b> for &'a &'b str {
    type __TStr = crate::Empty;

    const __ASSERT_COMPARABLE_WITNESS: __AssertComparableWitness<'a, 'b, Self> =
        __AssertComparableWitness::RefStr(TypeEq::NEW);
}

impl<'a, 'b: 'a, S> AssertComparable<'a, 'b> for TStr<S>
where
    TStr<S>: IsTStr,
{
    type __TStr = TStr<S>;

    const __ASSERT_COMPARABLE_WITNESS: __AssertComparableWitness<'a, 'b, Self> =
        __AssertComparableWitness::TStr(<TStr<S> as Identity>::TYPE_EQ);
}

impl<'a, 'b: 'a, S> AssertComparable<'a, 'b> for &'a TStr<S>
where
    TStr<S>: IsTStr,
{
    type __TStr = TStr<S>;

    const __ASSERT_COMPARABLE_WITNESS: __AssertComparableWitness<'a, 'b, Self> =
        __AssertComparableWitness::RefTStr(<TStr<S> as Identity>::TYPE_EQ.in_ref());
}

type ACToTstr<'a, 'b, T> = TStr<<<T as AssertComparable<'a, 'b>>::__TStr as IsTStr>::Arg>;

#[doc(hidden)]
pub enum __AssertComparableWitness<'a, 'b, T: AssertComparable<'a, 'b>> {
    Str(TypeEq<T, &'b str>),
    RefStr(TypeEq<T, &'a &'b str>),
    TStr(TypeEq<T, ACToTstr<'a, 'b, T>>),
    RefTStr(TypeEq<T, &'a ACToTstr<'a, 'b, T>>),
}

#[doc(hidden)]
pub const fn __coerce_assert_comparable<'a, 'b, A>(this: A) -> &'a str
where
    'b: 'a,
    A: AssertComparable<'a, 'b>,
{
    match A::__ASSERT_COMPARABLE_WITNESS {
        __AssertComparableWitness::Str(te) => te.to_right(this),
        __AssertComparableWitness::RefStr(te) => te.to_right(this),
        __AssertComparableWitness::TStr(te) => te.to_right(this).to_str(),
        __AssertComparableWitness::RefTStr(te) => te.to_right(this).to_str(),
    }
}
