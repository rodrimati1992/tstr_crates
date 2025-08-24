use crate::modules::utils::must_panic;

use const_panic::ArrayString;

use tstr::{TS, ts};

macro_rules! to_str {
    ($($args:tt)*) => (
        ArrayString::<100>::from_panicvals(
            &const_panic::flatten_panicvals!(const_panic::FmtArg::DEBUG; $($args)*)
        ).unwrap().to_str()
    )
}

const fn constrain<'a, T>(_: &'a T, string: &'static str) -> &'a str {
    string
}

const fn _const_and_usable_with_iststr<S: tstr::IsTStr>() {
    tstr::assert_str_eq!(S::VAL, "what", "hello", {?}: "how are you");
    tstr::assert_str_eq!(&S::VAL, "what", "hello", {?}: "how are you");
    tstr::assert_str_eq!(&&S::VAL, "what", "hello", {?}: "how are you");

    tstr::assert_str_ne!("no", S::VAL, 10u8, " ", {#X}: 20u16);
    tstr::assert_str_ne!("no", &S::VAL, 10u8, " ", {#X}: 20u16);
    tstr::assert_str_ne!("no", &&S::VAL, 10u8, " ", {#X}: 20u16);
}

#[test]
fn tstr_panicfmt_test() {
    assert_eq!(to_str!(ts!("foo\nbar")), "\"foo\\nbar\"");
    assert_eq!(to_str!({?}: ts!("foo\nbar")), "\"foo\\nbar\"");
    assert_eq!(to_str!({}: ts!("foo\nbar")), "foo\nbar");

    assert_eq!(to_str!(TS!("foo\nbar") => ts!("foo\nbar")), "\"foo\\nbar\"");
    assert_eq!(
        to_str!(TS!("foo\nbar") => {?}:ts!("foo\nbar")),
        "\"foo\\nbar\""
    );
    assert_eq!(to_str!(TS!("foo\nbar") => {}: ts!("foo\nbar")), "foo\nbar");
}

#[test]
fn assert_str_eq_basic_test() {
    must_panic(|| tstr::assert_str_eq!(ts!("0"), &ts!("1")));
    must_panic(|| tstr::assert_str_eq!(ts!("0"), ts!("1")));
    must_panic(|| tstr::assert_str_eq!(&ts!("0"), ts!("1")));
    must_panic(|| tstr::assert_str_eq!(&ts!("0"), "1"));
    must_panic(|| tstr::assert_str_eq!(ts!("0"), "1"));
    must_panic(|| tstr::assert_str_eq!("0", &ts!("1")));
    must_panic(|| tstr::assert_str_eq!("0", ts!("1")));
    must_panic(|| tstr::assert_str_eq!(&"0", &"1"));
    must_panic(|| tstr::assert_str_eq!(&"0", "1"));
    must_panic(|| tstr::assert_str_eq!("0", "1"));
    must_panic(|| tstr::assert_str_eq!("0", &"1"));

    const _: () = tstr::assert_str_eq!(ts!("0"), ts!("0"));
    const _: () = tstr::assert_str_eq!(ts!("0"), "0");
    const _: () = tstr::assert_str_eq!("0", ts!("0"));
    const _: () = tstr::assert_str_eq!("0", "0");
}

#[test]
fn assert_str_eq_lifetime_test() {
    const fn constness<'a>(lt: &'a ()) {
        let foo: &'a str = constrain(lt, "foo");
        let bar: &'a str = constrain(lt, "foo");

        tstr::assert_str_eq!(foo, bar);
    }

    constness(&())
}

#[test]
fn assert_str_eq_some_formatting_test() {
    const _: () = tstr::assert_str_eq!("0", "0", {}: "hello", {?}: "world");

    let foo = 10u8;
    let bar = "huh?";
    must_panic(|| tstr::assert_str_eq!("0", "1", foo, {?}: foo, {}: bar, {?}: "world"));
}

#[test]
fn assert_str_ne_basic_test() {
    const _: () = tstr::assert_str_ne!(ts!("0"), ts!("1"));
    const _: () = tstr::assert_str_ne!(ts!("0"), "1");
    const _: () = tstr::assert_str_ne!("0", ts!("1"));
    const _: () = tstr::assert_str_ne!("0", "1");

    must_panic(|| tstr::assert_str_ne!(ts!("0"), ts!("0")));
    must_panic(|| tstr::assert_str_ne!(ts!("0"), "0"));
    must_panic(|| tstr::assert_str_ne!("0", ts!("0")));
    must_panic(|| tstr::assert_str_ne!("0", "0"));
}

#[test]
fn assert_str_ne_lifetime_test() {
    const fn constness<'a>(lt: &'a ()) {
        let foo: &'a str = constrain(lt, "foo");
        let bar: &'a str = constrain(lt, "bar");

        tstr::assert_str_ne!(foo, bar);
    }

    constness(&())
}

#[test]
fn assert_str_ne_some_formatting_test() {
    const _: () = {
        let foo = 10u8;
        let bar = "huh?";
        tstr::assert_str_ne!("0", "1", foo, {?}: foo, {}: bar, {?}: "world");
    };

    must_panic(|| tstr::assert_str_ne!("0", "0", {}: "hello", {?}: "world"));
}
