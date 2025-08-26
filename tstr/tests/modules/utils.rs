use std::panic::{AssertUnwindSafe, catch_unwind};

macro_rules! assert_equals_string {
    ($tstr_ty:ty, $string:expr) => {
        const _: () = {
            const X: &[u8] = $string.as_bytes();

            let tstr = <$tstr_ty>::new();
            assert!(matches!(tstr::to_str(tstr).as_bytes(), X));
            assert!(matches!(tstr::to_bytes(tstr), X));
            assert!(tstr::len(tstr) == X.len());
        };
    };
}

#[cfg(feature = "str_generics")]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<tstr::___<$string>> = ts!($input);
        assert_equals_string! {tstr::TStr<tstr::___<$string>>, $string}
    };
}

#[cfg(not(feature = "str_generics"))]
macro_rules! test_case {
    ($input:tt, $chars:ty, $string:expr $(,)*) => {
        const _: tstr::TStr<___<$chars, { $string.len() }>> = ts!($input);
        assert_equals_string! {tstr::TStr<___<$chars, { $string.len() }>>, $string}
    };
}

macro_rules! str_test_case {
    ($string:tt, $tuple:ty $(,)*) => {
        test_case!($string, $tuple, $string);
    };
}

#[track_caller]
pub fn assert_type<Expected: ?Sized>(x: &impl ?Sized) {
    assert_eq!(
        std::any::type_name_of_val(x),
        std::any::type_name::<Expected>(),
    );
}

#[track_caller]
pub fn assert_typename<T: ?Sized, Expected: ?Sized>() {
    assert_eq!(
        std::any::type_name::<T>(),
        std::any::type_name::<Expected>(),
    );
}

#[track_caller]
pub fn must_panic<F, R>(f: F)
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => panic!("expected panic, none happened"),
        Err(_) => {}
    }
}

#[test]
fn test_must_panic_no_panic() {
    must_panic(|| panic!());
}

#[test]
#[should_panic]
fn test_must_panic_panics() {
    must_panic(|| ());
}
