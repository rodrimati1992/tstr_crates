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
/// - A comma separated list of string/integer literals, and/or identifiers
/// (eg: `TS!(foo, "bar", 0)`), this evaluates to a tuple of `TStr`s.
///
/// # Examples
///
/// ### ToVariant
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
///
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
/// ### Equivalences
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
///
/// type Tup = TS!(foo, 1, "bar"); // equivalent to `(TS!(foo), TS!(1), TS!(bar))`
///
/// ```
///
/// [`TStr`]: ./struct.TStr.html
#[macro_export]
macro_rules! TS {
    ($($expr:expr),* $(,)* ) => {
        $crate::__ts_impl!(($crate) $($expr)*)
    };
}

/// A type-level string [`TStr`] value.
///
/// # Arguments
///
/// You can use any of these as arguments:
///
/// - String literals (eg: `ts!("hello")`, `ts!(r#"world"#)`)
///
/// - Integes (eg: `ts!(0)`, `ts!(100)`), stringifying the integer.
///
/// - Single identifiers (eg: `ts!(foo)`, `ts!(bar)`), stringifying the identifier.
///
/// - A comma separated list of string/integer literals, and/or identifiers
/// (eg: `ts!(foo, "bar", 0)`), this evaluates to a tuple of `TStr`s.
///
/// # Examples
///
/// ### Indexing
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
/// ### Equivalences
///
/// This example demonstrates which invocations of `ts` produce the same values.
///
/// ```rust
/// use tstr::ts;
///
/// let hello1 = ts!("hello");
/// let hello2 = ts!(hello); // This is equivalent to `ts!("hello")`
///
/// let hundreda = ts!("100");
/// let hundredb = ts!(100);   // equivalent to `ts!("100")`
/// let hundredc = ts!(0x64);  // equivalent to `ts!("100")`
/// let hundredd = ts!(0b1100100);  // equivalent to `ts!("100")`
///
/// let tup = ts!(foo, 1, "bar"); // equivalent to `(ts!(foo), ts!(1), ts!(bar))`
/// ```
///
///
/// [`TStr`]: ./struct.TStr.html
#[macro_export]
macro_rules! ts {
    ($($expr:expr),* $(,)* ) => {
        <$crate::__ts_impl!(($crate) $($expr)*) as $crate::MakeTStr>::MAKE
    };
}

/// Declares `const` and `type` aliases for type-level strings.
///
/// # String Arguments
///
/// You can alias either, one type-level string, or a tuple of type-level strings
///
/// ```rust
/// tstr::alias!{
///     // Aliases the "bar" type-level string as both a const and a type, named Bar.
///     pub Bar = bar;
///
///     // Aliases the "0" type-level string.
///     pub N0 = 0;
///
///     // Aliases the ("foo", "baz") tuple of type-level strings,
///     // Equivalent to `TS!(foo, baz)` and `ts!("foo", "baz")`
///     pub Tup = (foo, baz);
/// }
///
/// # const _: (tstr::TS!(foo), tstr::TS!(baz)) = Tup;
/// ```
///
/// # Examples
///
/// ### Indexing
///
/// This uses types from the `for_examples` module,
/// which can be seen in the docs with the "for_examples" feature.
///
/// ```rust
/// use std::ops::Index;
///
/// use tstr::for_examples::{Foo, Bar};
///
/// tstr::alias!{
///     // Declares both an NBar type alias and an NBar constant of that type.
///     pub NBar = bar;
///
///     // Declares both an NBaz type alias and an NBaz constant of that type.
///     pub NBaz = "baz";
///
///     // Declares both an NQux type alias and an NQux constant of that type.
///     pub NQux = "qux";
///
/// }
///
/// // The macro can also be invoked like this
/// tstr::alias!{ pub NBoom = boom }
///
/// let this = Foo::new(3, 5, "8");
/// assert_eq!(get_bar_baz(&this), (3, 5));
///
/// let other = Bar::new(13, false, Some('C'));
/// assert_eq!(get_bar_baz(&other), (13, false));
///
///
/// type IndexOut<T, N> = <T as Index<N>>::Output;
///
/// fn get_bar_baz<T>(this: &T) -> (IndexOut<T, NBar>, IndexOut<T, NBaz>)
/// where
///     T: Index<NBar> + Index<NBaz>,
///     IndexOut<T, NBar>: Copy,
///     IndexOut<T, NBaz>: Copy,
/// {
///     (this[NBar], this[NBaz])
/// }
///
/// ```
///
#[macro_export]
macro_rules! alias {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis $name:ident = $expr:tt
        );*
        $(;)?
    ) => (
        $(
            $crate::alias!{
                @decide-docs
                (
                    $(#[$attr])*
                    $vis,
                    $name,
                )
                [$expr]
            }
        )*
    );
    (@decide-docs
        $other:tt
        [($($expr:expr),* $(,)*)]
    )=>{
        $crate::alias!{
            @inner
            $other
            [$($expr),*]
            concat!(
                "An alias for `(",
                $(stringify!($expr), ", ",)*
                ")` as a tuple of type level strings.\n\n",
                "Generated by the [`::tstr::alias`] macro."
            )
        }
    };
    (@decide-docs
        $other:tt
        [$expr:expr]
    )=>{
        $crate::alias!{
            @inner
            $other
            [$expr]
            concat!(
                "An alias for `", stringify!($expr), "` as a type level string.\n\n",
                "Generated by the [`::tstr::alias`] macro."
            )
        }
    };
    (@inner
        (
            $(#[$attr:meta])*
            $vis:vis, $name:ident,
        )
        [$($expr:expr),*]
        $autodoc:expr
    )=>{
        $(#[$attr])*
        #[allow(broken_intra_doc_links)]
        #[doc = $autodoc]
        $vis type $name = $crate::TS!($($expr),*);

        $(#[$attr])*
        #[allow(non_upper_case_globals, broken_intra_doc_links)]
        #[doc = $autodoc]
        $vis const $name: $name = <$name as $crate::MakeTStr>::MAKE;
    };
}
