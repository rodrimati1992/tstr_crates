//! Types for asserting properties of type-level strings.

use core::marker::PhantomData;

/// For asserting the (in)equality of two type-level strings.
///
/// # Warning
///
/// From testing the associated constants from this type,
/// these assertions might not be evaluated in functions that
/// aren't reachable by public functions.
///
/// # Examples
///
/// For examples, you can look at each associated constant below.
///
///
///
pub struct Assert<A, B>(core::marker::PhantomData<(A, B)>);

#[cfg(feature = "cmp_traits")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp_traits")))]
impl<A, B> Assert<A, B>
where
    A: crate::TStrEq<B>,
{
    /// Asserts that the `A` and `B` type-level strings compare equal.
    pub const EQUAL: EqualityProof<A, B> = {
        ["Expected the type parameters to be equal"][A::NE as usize];
        EqualityProof(PhantomData)
    };
}

#[cfg(feature = "cmp_traits")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp_traits")))]
impl<A, B> Assert<A, B>
where
    A: crate::TStrEq<B>,
{
    /// Asserts that the `A` and `B` type-level strings compare not equal.
    ///
    /// # Example
    ///
    /// This uses types from the `for_examples` module,
    /// which can be seen in the docs with the "for_examples" feature.
    ///
    /// ```rust
    /// use tstr::for_examples::Foo;
    /// use tstr::{Assert, ts};
    ///
    /// use std::ops::Index;
    ///
    /// let this = Foo::new(3, 5, "8");
    ///
    /// assert_eq!(this.get_two(ts!(bar), ts!(qux), Assert::NOT_EQUAL), (&3, &"8"))
    ///
    /// ```
    ///
    /// The same method call errors when we try to get two references to the same field.
    // For some reason it fails to compile with ```rust
    // but compiles without errors with ```compile_fail (causing a cargo test failure)
    /// ```ignore
    /// use tstr::for_examples::Foo;
    /// use tstr::{Assert, ts};
    /// use std::ops::Index;
    ///
    /// # pub fn main() {
    ///
    /// let this = Foo::new(3, 5, "8");
    ///
    /// assert_eq!(this.get_two(ts!(bar), ts!(bar), Assert::NOT_EQUAL), (&3, &3))
    ///
    /// # }
    /// ```
    ///
    /// Truncated error:
    /// ```text
    /// error[E0080]: erroneous constant used
    ///   --> src/asserts.rs:55:45
    ///    |
    /// 11 | assert_eq!(this.get_two(ts!(bar), ts!(bar), Assert::NOT_EQUAL), (&3, &3))
    ///    |                                             ^^^^^^^^^^^^^^^^^ referenced constant has errors
    ///
    /// ```
    ///
    pub const NOT_EQUAL: InequalityProof<A, B> = {
        ["Expected the type parameters to not be equal"][A::EQ as usize];
        InequalityProof(PhantomData)
    };
}

macro_rules! declare_assert_res {
    (
        $(#[$meta:meta])*
        struct $struct:ident<$L:ident, $R:ident>;
    )=> {
        $(#[$meta])*
        pub struct $struct<$L, $R>(PhantomData<($L, $R)>);

        impl<$L, $R> Copy for $struct<$L, $R> {}

        impl<$L, $R> Clone for $struct<$L, $R> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<$L, $R> $struct<$L, $R> {
            /// Infers the type parameters by passing them as arguments.
            pub const fn infer(self, _: &$L, _: &$R){}
        }
    };
}

#[cfg(feature = "cmp_traits")]
declare_assert_res! {
    /// Value-level proof that the `L` and `R` type-level strings compared equal.
    ///
    /// Constructed with [`Àssert::EQUAL`]
    ///
    /// [`Àssert::EQUAL`]: ./struct.Assert.html#associatedconstant.EQUAL
    struct EqualityProof<L,R>;
}

#[cfg(feature = "cmp_traits")]
declare_assert_res! {
    /// Value-level proof that the `L` and `R` type-level strings compared not equal.
    ///
    /// Constructed with [`Àssert::NOT_EQUAL`]
    ///
    /// [`Àssert::NOT_EQUAL`]: ./struct.Assert.html#associatedconstant.NOT_EQUAL
    struct InequalityProof<L, R>;
}
