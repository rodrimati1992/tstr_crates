use const_panic::ArrayString;

use tstr::{TS, ts};

macro_rules! to_str {
    ($($args:tt)*) => (
        ArrayString::<100>::from_panicvals(
            &const_panic::flatten_panicvals!(const_panic::FmtArg::DEBUG; $($args)*)
        ).unwrap().to_str()
    )
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
