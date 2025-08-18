use core::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use crate::IsTStr;

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
///     fn get(&self, _field_name: N) -> &Self::Field;
///     fn set(&mut self, _field_name: N, val: Self::Field);
/// }
///
/// impl<A, B, C> Access<TS!(0)> for (A, B, C) {
///     type Field = A;
///
///     fn get(&self, _field_name: TS!(0)) -> &A {
///         &self.0
///     }
///     fn set(&mut self, _field_name: TS!(0), val: A){
///         self.0 = val;
///     }
/// }
///
/// impl<A, B, C> Access<TS!(1)> for (A, B, C) {
///     type Field = B;
///
///     fn get(&self, _field_name: TS!(1)) -> &B {
///         &self.1
///     }
///     fn set(&mut self, _field_name: TS!(1), val: B){
///         self.1 = val;
///     }
/// }
///
/// impl<A, B, C> Access<TS!(2)> for (A, B, C) {
///     type Field = C;
///
///     fn get(&self, _field_name: TS!(2)) -> &C {
///         &self.2
///     }
///     fn set(&mut self, _field_name: TS!(2), val: C){
///         self.2 = val;
///     }
/// }
///
/// ```
///
pub struct TStr<S>(pub(crate) PhantomData<fn() -> S>);

impl<S> TStr<S> {
    /// Constructs the TStr.
    pub const fn new() -> Self {
        TStr(PhantomData)
    }
}

impl<S> TStr<S> {
    /// Gets the `&'static str` equivalent of this TStr
    ///
    /// # Example
    ///
    /// ```rust
    /// use tstr::{TStr, ts};
    ///
    /// let foo: TStr<_> = ts!(foo);
    /// assert_eq!(foo.to_str(), "foo");
    ///
    /// assert_eq!(ts!("bar").to_str(), "bar");
    ///
    /// ```
    ///
    pub const fn to_str(self) -> &'static str
    where
        Self: IsTStr,
    {
        Self::STR
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

impl<S> Debug for TStr<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TStr").finish()
    }
}

impl<S> core::cmp::PartialEq for TStr<S> {
    #[inline(always)]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<S> core::cmp::Eq for TStr<S> {}

impl<S> core::cmp::PartialOrd for TStr<S> {
    #[inline(always)]
    fn partial_cmp(&self, _other: &Self) -> Option<core::cmp::Ordering> {
        Some(core::cmp::Ordering::Equal)
    }
}

impl<S> core::cmp::Ord for TStr<S> {
    #[inline(always)]
    fn cmp(&self, _other: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}
