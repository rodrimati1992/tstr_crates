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
fn assertc_eq_basic_test() {
    must_panic(|| tstr::assertc_eq!(ts!("0"), &ts!("1")));
    must_panic(|| tstr::assertc_eq!(ts!("0"), ts!("1")));
    must_panic(|| tstr::assertc_eq!(&ts!("0"), ts!("1")));
    must_panic(|| tstr::assertc_eq!(&ts!("0"), "1"));
    must_panic(|| tstr::assertc_eq!(ts!("0"), "1"));
    must_panic(|| tstr::assertc_eq!("0", &ts!("1")));
    must_panic(|| tstr::assertc_eq!("0", ts!("1")));
    must_panic(|| tstr::assertc_eq!(&"0", &"1"));
    must_panic(|| tstr::assertc_eq!(&"0", "1"));
    must_panic(|| tstr::assertc_eq!("0", "1"));
    must_panic(|| tstr::assertc_eq!("0", &"1"));

    const _: () = tstr::assertc_eq!(ts!("0"), ts!("0"));
    const _: () = tstr::assertc_eq!(ts!("0"), "0");
    const _: () = tstr::assertc_eq!("0", ts!("0"));
    const _: () = tstr::assertc_eq!("0", "0");
}

#[test]
fn assertc_eq_lifetime_test() {
    const fn constness<'a>(lt: &'a ()) {
        let foo: &'a str = constrain(lt, "foo");
        let bar: &'a str = constrain(lt, "foo");

        tstr::assertc_eq!(foo, bar);
    }

    constness(&())
}

#[test]
fn assertc_eq_some_formatting_test() {
    const _: () = tstr::assertc_eq!("0", "0", {}: "hello", {?}: "world");

    let foo = 10u8;
    let bar = "huh?";
    must_panic(|| tstr::assertc_eq!("0", "1", foo, {?}: foo, {}: bar, {?}: "world"));
}

#[test]
fn assertc_ne_basic_test() {
    const _: () = tstr::assertc_ne!(ts!("0"), ts!("1"));
    const _: () = tstr::assertc_ne!(ts!("0"), "1");
    const _: () = tstr::assertc_ne!("0", ts!("1"));
    const _: () = tstr::assertc_ne!("0", "1");

    must_panic(|| tstr::assertc_ne!(ts!("0"), ts!("0")));
    must_panic(|| tstr::assertc_ne!(ts!("0"), "0"));
    must_panic(|| tstr::assertc_ne!("0", ts!("0")));
    must_panic(|| tstr::assertc_ne!("0", "0"));
}

#[test]
fn assertc_ne_lifetime_test() {
    const fn constness<'a>(lt: &'a ()) {
        let foo: &'a str = constrain(lt, "foo");
        let bar: &'a str = constrain(lt, "bar");

        tstr::assertc_ne!(foo, bar);
    }

    constness(&())
}

#[test]
fn assertc_ne_some_formatting_test() {
    const _: () = {
        let foo = 10u8;
        let bar = "huh?";
        tstr::assertc_ne!("0", "1", foo, {?}: foo, {}: bar, {?}: "world");
    };

    must_panic(|| tstr::assertc_ne!("0", "0", {}: "hello", {?}: "world"));
}
