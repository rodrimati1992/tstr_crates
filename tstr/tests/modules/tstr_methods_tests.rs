use crate::modules::utils::{assert_type, must_panic};

use tstr::{IsTStr, TS, TStr, TStrArg};

use std::cmp::{Ord, Ordering, PartialOrd};

type AString = TS!(1234);

type BString = TS!(12345);

#[test]
fn conversion_into_tstr_test() {
    fn foo<S: IsTStr>() {
        assert_type::<TStr<S::Arg>>(&const { TStr::<S::Arg>::from_gen(S::VAL) });

        must_panic(|| {
            assert_type::<BString>(&const { TStr::<S::Arg>::from_gen(S::VAL) });
        });
    }

    foo::<AString>();
}

#[test]
fn conversion_from_generic_test() {
    fn foo<S: IsTStr>() {
        assert_type::<S>(&const { TStr::<S::Arg>::new().to_gen::<S>() });

        must_panic(|| assert_type::<BString>(&const { TStr::<S::Arg>::new().to_gen::<S>() }));
    }

    foo::<AString>();
}

#[test]
fn fmt_test() {
    macro_rules! test_case {
        ($s:literal) => {{
            type Type = TS!($s);
            const CONST: &str = $s;

            assert_eq!(format!("{}", Type::new()), format!("{}", CONST));
            assert_eq!(format!("{:?}", Type::new()), format!("{:?}", CONST));
            assert_eq!(format!("{:#?}", Type::new()), format!("{:#?}", CONST));
        }};
    }

    test_case! {"\nfoo\r\t\0"}
}

#[test]
fn default_test() {
    fn _blanket<L: TStrArg>() {
        _ = <TStr<L> as Default>::default();
    }
}

#[test]
fn hash_test() {
    use std::collections::HashMap;

    fn case<L: IsTStr>() {
        let val = TStr::<L::Arg>::new();
        let mut map = HashMap::from([(val, 3)]);

        assert_eq!(map.insert(val, 5), Some(3));
        assert_eq!(map.len(), 1);

        assert_eq!(map.insert(val, 8), Some(5));
        assert_eq!(map.len(), 1);

        assert_eq!(map.remove(&val), Some(8));
        assert!(map.is_empty());
    }

    case::<AString>();
    case::<BString>();
}

#[test]
fn cmp_test() {
    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: Ordering) {
        let lhs = TStr::<L::Arg>::new();
        let rhs = TStr::<R::Arg>::new();

        assert_eq!(
            PartialOrd::partial_cmp(&lhs, &rhs),
            Some(expected),
            "partial_cmp"
        );

        assert_eq!(Ord::cmp(&lhs, &lhs), Ordering::Equal, "cmp");
    }

    case::<AString, AString>(Ordering::Equal);
    case::<AString, BString>(Ordering::Less);
    case::<BString, AString>(Ordering::Greater);
    case::<BString, BString>(Ordering::Equal);
}

#[test]
fn eq_ne_test() {
    fn assert_is_eq<T: std::cmp::Eq>(_: T) {}

    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: bool) {
        let lhs = TStr::<L::Arg>::new();
        let rhs = TStr::<R::Arg>::new();
        assert_is_eq(lhs);
        assert_is_eq(rhs);

        assert_eq!(lhs == rhs, expected, "eq");
        assert_eq!(lhs != rhs, !expected, "ne");
    }

    case::<AString, AString>(true);
    case::<AString, BString>(false);
    case::<BString, AString>(false);
    case::<BString, BString>(true);
}
