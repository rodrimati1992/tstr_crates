use crate::modules::utils::{assert_type, assert_typename};

use tstr::TS;
use tstr::strlike::{StrLike, as_str};

type One = TS!(1);

#[test]
fn unref_test() {
    type AsOut<T> = <T as tstr::strlike::__AsStrLike>::Target;

    assert_typename::<AsOut<One>, One>();
    assert_typename::<AsOut<&One>, One>();
    assert_typename::<AsOut<&&One>, One>();

    assert_typename::<AsOut<str>, str>();
    assert_typename::<AsOut<&str>, str>();
    assert_typename::<AsOut<&&str>, str>();
}

#[test]
fn as_str_macro_test() {
    const fn _assert_const_and_lifetimes<'a, S: tstr::IsTStr>(tt: &'a S, ss: &'a str) {
        let _: &'a str = as_str!(tt);
        let _: &'a str = as_str!(&tt);
        let _: &'a str = as_str!(&&tt);

        let _: &'a str = as_str!(ss);
        let _: &'a str = as_str!(&ss);
        let _: &'a str = as_str!(&&ss);
    }

    assert_type::<&str>(&as_str!(One::new()));
    assert_type::<&str>(&as_str!(&One::new()));
    assert_type::<&str>(&as_str!(&&One::new()));
    assert_type::<&str>(&as_str!(&&&One::new()));

    assert_type::<&str>(&as_str!(""));
    assert_type::<&str>(&as_str!(&""));
    assert_type::<&str>(&as_str!(&&""));
    assert_type::<&str>(&as_str!(&&&""));

    #[track_caller]
    fn case<'a, S: ?Sized + StrLike>(val: &'a S, expected: &'a str) {
        assert_eq!(as_str!(val), expected);
        assert_eq!(val.as_str(), expected);
    }

    case(&One::new(), "1");
    case("hello", "hello");
}
