/// For comparing [`TStr`] types for equality, using [`TStrEq::EQ`].
///
/// # Example
///
/// ```rust
/// use tstr::{TS, tstr_eq};
///
/// assert!( tstr_eq!(TS!("foo"), TS!("foo")));
/// assert!(!tstr_eq!(TS!("foo"), TS!("bar")));
///
/// type Foo = TS!("foo");
/// type Bar = TS!("bar");
///
/// assert!( tstr_eq!(Foo, Foo));
/// assert!(!tstr_eq!(Foo, Bar));
/// ```
///
/// [`TStrEq::EQ`]: ./trait.TStrEq.html#associatedconstant.EQ
/// [`TStr`]: ./struct.TStr.html
///
#[macro_export]
macro_rules! tstr_eq {
    ($left:ty, $right:ty $(,)*) => {
        <$left as $crate::TStrEq<$right>>::EQ
    };
}

/// For comparing [`TStr`] types for inequality, using [`TStrEq::NE`]
///
/// # Example
///
/// ```rust
/// use tstr::{TS, tstr_ne};
///
/// assert!(!tstr_ne!(TS!("foo"), TS!("foo")));
/// assert!( tstr_ne!(TS!("foo"), TS!("bar")));
///
/// type Foo = TS!("foo");
/// type Bar = TS!("bar");
///
/// assert!(!tstr_ne!(Foo, Foo));
/// assert!( tstr_ne!(Foo, Bar));
/// ```
///
/// [`TStrEq::NE`]: ./trait.TStrEq.html#associatedconstant.NE
/// [`TStr`]: ./struct.TStr.html
#[macro_export]
macro_rules! tstr_ne {
    ($left:ty, $right:ty $(,)*) => {
        <$left as $crate::TStrEq<$right>>::NE
    };
}

/// For comparing [`TStr`] types for ordering, using [`TStrOrd::CMP`]
///
/// # Example
///
/// ```rust
/// use tstr::{TS, tstr_cmp};
///
/// use std::cmp::Ordering;
///
/// type Aaa = TS!(Aaa);
/// type Bbb = TS!(Bbb);
/// type Ccc = TS!(Ccc);
///
/// assert_eq!(tstr_cmp!(Bbb, Ccc), Ordering::Less);
/// assert_eq!(tstr_cmp!(Bbb, Bbb), Ordering::Equal);
/// assert_eq!(tstr_cmp!(Bbb, Aaa), Ordering::Greater);
/// ```
///
/// [`TStrOrd::CMP`]: ./trait.TStrOrd.html#associatedconstant.CMP
/// [`TStr`]: ./struct.TStr.html
#[cfg(feature = "const_generics")]
#[macro_export]
macro_rules! tstr_cmp {
    ($left:ty, $right:ty $(,)*) => {
        <$left as $crate::TStrOrd<$right>>::CMP
    };
}
