/// The type of a type-level string.
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
#[macro_export]
macro_rules! TS {
    ($token:tt) => {
        $crate::__ts_impl!(($crate) $token)
    };
}

/// A type-level string as a `TStr` value.
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
#[macro_export]
macro_rules! ts {
    ($token:tt) => {
        <$crate::__ts_impl!(($crate) $token)>::NEW
    };
}
