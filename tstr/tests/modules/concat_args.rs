use tstr::{ts, TS};

#[allow(dead_code)]
type ConcatIdents = TS!("foobarbazqux");

#[allow(dead_code)]
type ConcatWithNum = TS!("3,7,15,31");

#[test]
fn test_concat() {
    const _: ConcatIdents = ts!(concat!(foo, bar, baz, qux));
    const _: ConcatIdents = ts!(concat!(concat!(foo, bar), baz, qux));
    const _: ConcatIdents = ts!(concat!(foo, concat!(bar, baz,), qux));
    const _: ConcatIdents = ts!(concat!(foo, bar, concat!(baz, qux)));

    const _: ConcatIdents = ts!(concat!("foo", bar, r##"baz"##, qux));
    const _: ConcatIdents = ts!(concat!(concat!("foo", bar), r##"baz"##, qux));
    const _: ConcatIdents = ts!(concat!("foo", concat!(bar, r##"baz"##,), qux));
    const _: ConcatIdents = ts!(concat!("foo", bar, concat!(r##"baz"##, qux)));

    const _: ConcatWithNum = ts!(concat!(0b11, ",", 0o7, ",", 15, ",", 0x1F));
    const _: ConcatWithNum = ts!(concat!(0b1_1, ",", 0o_7, ",", 1_5, ",", 0x1_F));
    const _: ConcatWithNum = ts!(concat!(0b1_1, ",", 0o_7, ",15,31"));
}

#[test]
fn test_stringify() {
    let _: TS!("0b11") = ts!(stringify!(0b11));
    let _: TS!("0o7") = ts!(stringify!(0o7));
    let _: TS!("15") = ts!(stringify!(15));
    let _: TS!("0x1F") = ts!(stringify!(0x1F));
    let _: TS!(r#""hello""#) = ts!(stringify!("hello"));
}
