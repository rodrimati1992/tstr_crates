#[macro_export]
macro_rules! TS {
    ($token:tt) => {
        $crate::__ts_impl!(($crate) $token)
    };
}

#[macro_export]
macro_rules! ts {
    ($token:tt) => {
        <$crate::__ts_impl!(($crate) $token)>::NEW
    };
}
