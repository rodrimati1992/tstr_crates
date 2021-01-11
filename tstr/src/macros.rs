/// The type of a type-level string, always a [`TStr`].
///
/// # Arguments
///
/// You can use any of these as arguments:
///
/// - String literals (eg: `TS!("hello")`, `TS!(r#"world"#)`)
///
/// - Integes (eg: `TS!(0)`, `TS!(100)`), stringifying the integer.
///
/// - Single identifiers (eg: `TS!(foo)`, `TS!(bar)`), stringifying the identifier.
///
/// # Example
///
/// # ToVariant
///
/// This example demonstrates how you can use type-level strings to create a
/// `GetVariant` trait which gets the data in a variant if the enum is that variant.
///
/// ```rust
/// use tstr::TS;
///
/// fn main(){
///     let foo = Enum::Foo(3, 5);
///     let bar = Enum::Bar("hello".to_string());
///     
///     assert_eq!(foo.to_variant(Foo::NEW), Some((3, 5)));
///     assert_eq!(foo.to_variant(Bar::NEW), None);
///     
///     assert_eq!(bar.to_variant(Foo::NEW), None);
///     assert_eq!(bar.to_variant(Bar::NEW), Some("hello".to_string()));
/// }
///
/// type Foo = TS!(Foo);
/// type Bar = TS!(Bar);
///
/// trait ToVariant<V> {
///     type Output;
///     
///     fn to_variant(&self, variant: V) -> Option<Self::Output>;
/// }
///
/// enum Enum {
///     Foo(u32, u64),
///     Bar(String),
/// }
///
/// impl ToVariant<TS!(Foo)> for Enum {
///     type Output = (u32, u64);
///     
///     fn to_variant(&self, variant: TS!(Foo)) -> Option<Self::Output> {
///         match self {
///             Self::Foo(l, r) => Some((*l, *r)),
///             _ => None,
///         }
///     }
/// }
///
/// impl ToVariant<TS!(Bar)> for Enum {
///     type Output = String;
///     
///     fn to_variant(&self, variant: TS!(Bar)) -> Option<Self::Output> {
///         match self {
///             Self::Bar(s) => Some(s.clone()),
///             _ => None,
///         }
///     }
/// }
///
///
/// ```
///
/// # Equivalences
///
/// This example demonstrates which invocations of `TS` produce the same type.
///
/// ```rust
/// use tstr::TS;
///
/// type Hello1 = TS!("hello");
/// type Hello2 = TS!(hello); // This is equivalent to `TS!("hello")`
///
/// type HundredA = TS!("100");
/// type HundredB = TS!(100);   // equivalent to `TS!("100")`
/// type HundredC = TS!(0x64);  // equivalent to `TS!("100")`
/// type HundredD = TS!(0b1100100);  // equivalent to `TS!("100")`
/// ```
///
/// [`TStr`]: ./struct.TStr.html
#[macro_export]
macro_rules! TS {
    ($token:tt) => {
        $crate::__ts_impl!(($crate) $token)
    };
}

/// A type-level string [`TStr`] value.
///
/// # Arguments
///
/// You can use any of these as arguments:
///
/// - String literals (eg: `TS!("hello")`, `TS!(r#"world"#)`)
///
/// - Integes (eg: `TS!(0)`, `TS!(100)`), stringifying the integer.
///
/// - Single identifiers (eg: `TS!(foo)`, `TS!(bar)`), stringifying the identifier.
///
/// # Example
///
/// This uses types from the `for_examples` module,
/// which can be seen in the docs with the "for_examples" feature.
///
/// ```rust
/// use tstr::ts;
/// use tstr::for_examples::{Foo, Bar};
///
/// let this = Foo::new(3, 5, "8");
/// assert_eq!(this[ts!(bar)], 3);
/// assert_eq!(this[ts!(baz)], 5);
/// assert_eq!(this[ts!(qux)], "8");
///
/// let other = Bar::new(13, false, Some('C'));
/// assert_eq!(other[ts!(bar)], 13);
/// assert_eq!(other[ts!(baz)], false);
/// assert_eq!(other[ts!(boom)], Some('C'));
///
/// ```
///
/// [`TStr`]: ./struct.TStr.html
#[macro_export]
macro_rules! ts {
    ($token:tt) => {
        <$crate::__ts_impl!(($crate) $token)>::NEW
    };
}
