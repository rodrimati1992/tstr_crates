#![deny(repr_transparent_external_private_fields)]

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
            "partial_cmp: {}",
            line!()
        );

        assert_eq!(Ord::cmp(&lhs, &lhs), Ordering::Equal, "cmp");

        // comparing TStr and str
        assert_eq!(
            PartialOrd::partial_cmp(lhs.to_str(), &rhs),
            Some(expected),
            "partial_cmp: {}",
            line!()
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs.to_str(), &rhs),
            Some(expected),
            "partial_cmp: {}",
            line!()
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, rhs.to_str()),
            Some(expected),
            "partial_cmp: {}",
            line!()
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, &rhs.to_str()),
            Some(expected),
            "partial_cmp: {}",
            line!()
        );

        assert_eq!(
            PartialOrd::partial_cmp(lhs.to_str(), &lhs),
            Some(Ordering::Equal),
            "cmp"
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs.to_str(), &lhs),
            Some(Ordering::Equal),
            "cmp"
        );
        assert_eq!(
            PartialOrd::partial_cmp(&&lhs.to_str(), &lhs),
            Some(Ordering::Equal),
            "cmp"
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, lhs.to_str()),
            Some(Ordering::Equal),
            "cmp"
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, &lhs.to_str()),
            Some(Ordering::Equal),
            "cmp"
        );
        assert_eq!(
            PartialOrd::partial_cmp(&lhs, &&lhs.to_str()),
            Some(Ordering::Equal),
            "cmp"
        );
    }

    case::<AString, AString>(Ordering::Equal);
    case::<AString, BString>(Ordering::Less);
    case::<BString, AString>(Ordering::Greater);
    case::<BString, BString>(Ordering::Equal);
}

#[test]
fn eq_ne_test() {
    use std::cmp::{Eq, PartialEq};

    fn assert_is_eq<'a, T: Eq + PartialEq<str> + PartialEq<&'a str>>(_: T) {}

    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: bool) {
        let lhs = TStr::<L::Arg>::new();
        let rhs = TStr::<R::Arg>::new();
        assert_is_eq(lhs);
        assert_is_eq(rhs);

        assert_eq!(lhs == rhs, expected, "eq");
        assert_eq!(lhs != rhs, !expected, "ne");

        // comparing str and TStr
        assert_eq!(*lhs.to_str() == rhs, expected, "eq");
        assert_eq!(lhs.to_str() == rhs, expected, "eq");
        assert_eq!(&lhs.to_str() == rhs, expected, "eq");

        assert_eq!(*lhs.to_str() != rhs, !expected, "ne");
        assert_eq!(lhs.to_str() != rhs, !expected, "ne");
        assert_eq!(&lhs.to_str() != rhs, !expected, "ne");

        assert_eq!(lhs == *rhs.to_str(), expected, "eq");
        assert_eq!(lhs == rhs.to_str(), expected, "eq");
        assert_eq!(lhs == &rhs.to_str(), expected, "eq");

        assert_eq!(lhs != *rhs.to_str(), !expected, "ne");
        assert_eq!(lhs != rhs.to_str(), !expected, "ne");
        assert_eq!(lhs != &rhs.to_str(), !expected, "ne");
    }

    case::<AString, AString>(true);
    case::<AString, BString>(false);
    case::<BString, AString>(false);
    case::<BString, BString>(true);
}

#[test]
fn ensure_usable_in_std_asserts_test() {
    std::assert_eq!(AString::VAL, "1234");
    std::assert_ne!(AString::VAL, "12345");
    must_panic(|| std::assert_ne!(AString::VAL, "1234"));
    must_panic(|| std::assert_eq!(AString::VAL, "12345"));
}

#[allow(dead_code)]
#[repr(transparent)]
pub struct TStrInTransparentWrapper<N: IsTStr, T> {
    // since TStr is zero-sized, it can be put alongside the non-zero-sized
    // wrapped value in a `#[repr(transparent)]` type.
    pub name: TStr<N::Arg>,
    pub value: T,
}
