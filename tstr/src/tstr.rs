use core::{
    fmt::{self, Debug},
    marker::PhantomData,
};

/// A type-level string type, similar to a `&'static str` const parameter.
///
/// # Examples
///
/// ### Accessing Fields
///
/// This example demonstrates how you can use `TStr` to implement a generic accessor trait.
///
/// ```rust
/// use tstr::TStr;
/// use tstr::{TS, ts};
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
/// fn replace<T, N>(this: &mut T, name: TStr<N>, replacement: T::Field) -> T::Field
/// where
///     T: Access<TStr<N>>,
///     T::Field: Clone,
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
///     fn get(&self, _: N) -> &Self::Field;
///     fn set(&mut self, _: N, val: Self::Field);
/// }
///
/// impl<A, B, C> Access<TS!(0)> for (A, B, C) {
///     type Field = A;
///
///     fn get(&self, _: TS!(0)) -> &A {
///         &self.0
///     }
///     fn set(&mut self, _: TS!(0), val: A){
///         self.0 = val;
///     }
/// }
///
/// impl<A, B, C> Access<TS!(1)> for (A, B, C) {
///     type Field = B;
///
///     fn get(&self, _: TS!(1)) -> &B {
///         &self.1
///     }
///     fn set(&mut self, _: TS!(1), val: B){
///         self.1 = val;
///     }
/// }
///
/// impl<A, B, C> Access<TS!(2)> for (A, B, C) {
///     type Field = C;
///
///     fn get(&self, _: TS!(2)) -> &C {
///         &self.2
///     }
///     fn set(&mut self, _: TS!(2), val: C){
///         self.2 = val;
///     }
/// }
///
/// ```
///
pub struct TStr<T>(pub(crate) PhantomData<fn() -> T>);

impl<T> TStr<T> {
    /// Constructs the TStr.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{TS, TStr};
    ///
    /// type FOO = TS!(foo);
    ///
    /// let foo_1: FOO = TStr::NEW;
    /// let foo_2 = FOO::NEW; // The same as the previous statement
    ///
    /// ```
    pub const NEW: Self = TStr(PhantomData);
}

#[cfg(feature = "const_generics")]
macro_rules! const_generics_using {
    () => {
        /// For getting the `&'static str` value of this `TStr`.
        ///
        /// You can use this as the bound for a generic `TStr` parameter.
        ///
        /// # Example
        ///
        /// ```rust
        /// use tstr::{StrValue, ts};
        ///
        /// asserts(ts!(foo), ts!(bar), ts!(baz));
        ///
        /// fn asserts<A, B, C>(foo: A, bar: B, baz: C)
        /// where
        ///     A: StrValue,
        ///     B: StrValue,
        ///     C: StrValue,
        /// {
        ///     assert_eq!(A::STR, "foo");
        ///     assert_eq!(foo.to_str(), "foo");
        ///
        ///     assert_eq!(B::STR, "bar");
        ///     assert_eq!(bar.to_str(), "bar");
        ///
        ///     assert_eq!(C::STR, "baz");
        ///     assert_eq!(baz.to_str(), "baz");
        ///
        /// }
        ///
        /// ```
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        pub trait StrValue: Debug + Copy + Default + 'static {
            /// The `&'static str` value of this `TStr`.
            const STR: &'static str;

            /// Gets the `&'static str` value of this `TStr`.
            fn to_str(self) -> &'static str {
                Self::STR
            }
        }

        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        impl<const S: &'static str> StrValue for TStr<crate::__<S>> {
            const STR: &'static str = S;
        }

        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
        impl<T> TStr<T>
        where
            Self: StrValue,
        {
            /// The `&'static str` value of this `TStr`.
            pub const STR: &'static str = <Self as StrValue>::STR;
        }
    };
}
#[cfg(feature = "const_generics")]
const_generics_using! {}

impl<T> Copy for TStr<T> {}

impl<T> Clone for TStr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for TStr<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::NEW
    }
}

impl<T> Debug for TStr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TStr").finish()
    }
}

impl<T> core::cmp::PartialEq for TStr<T> {
    #[inline(always)]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> core::cmp::Eq for TStr<T> {}

impl<T> core::cmp::PartialOrd for TStr<T> {
    #[inline(always)]
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<T> core::cmp::Ord for TStr<T> {
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}
