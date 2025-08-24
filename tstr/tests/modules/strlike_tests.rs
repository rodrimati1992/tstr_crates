use crate::modules::utils::{assert_type, assert_typename};

use tstr::TS;
use tstr::strlike::{StrLike, as_strlike};

type One = TS!(1);

#[test]
fn as_str_test() {
    const fn _const<'a>(tt: &'a One, ss: &'a str) {
        let _: &'a str = tstr::strlike::as_str(tt);
        let _: &'a str = tstr::strlike::as_str(ss);
    }

    #[track_caller]
    fn case<'a, S: ?Sized + StrLike>(val: &'a S, expected: &'a str) {
        assert_eq!(tstr::strlike::as_str(val), expected);
        assert_eq!(val.as_str(), expected);
    }

    case(&One::new(), "1");
    case("hello", "hello");
}

#[test]
fn unref_test() {
    type AsOut<T> = <T as tstr::strlike::AsStrLike>::Target;

    assert_typename::<AsOut<One>, One>();
    assert_typename::<AsOut<&One>, One>();
    assert_typename::<AsOut<&&One>, One>();

    assert_typename::<AsOut<str>, str>();
    assert_typename::<AsOut<&str>, str>();
    assert_typename::<AsOut<&&str>, str>();
}

#[test]
fn as_strlike_test() {
    const fn _assert_const_and_lifetimes<'a>(tt: &'a One, ss: &'a str) {
        let _: &'a One = as_strlike!(tt);
        let _: &'a One = as_strlike!(&tt);
        let _: &'a One = as_strlike!(&&tt);

        let _: &'a str = as_strlike!(ss);
        let _: &'a str = as_strlike!(&ss);
        let _: &'a str = as_strlike!(&&ss);
    }

    assert_type::<&One>(&as_strlike!(One::new()));
    assert_type::<&One>(&as_strlike!(&One::new()));
    assert_type::<&One>(&as_strlike!(&&One::new()));
    assert_type::<&One>(&as_strlike!(&&&One::new()));

    assert_type::<&str>(&as_strlike!(""));
    assert_type::<&str>(&as_strlike!(&""));
    assert_type::<&str>(&as_strlike!(&&""));
    assert_type::<&str>(&as_strlike!(&&&""));
}
