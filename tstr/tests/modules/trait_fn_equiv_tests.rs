use crate::modules::utils::{assert_type, must_panic};

use tstr::{IsTStr, TS, TStr};

use std::cmp::Ordering;

type AString = TS!(1234);
const A_STRING: &str = "1234";

type BString = TS!(12345);
const B_STRING: &str = "12345";

#[test]
fn conversion_into_tstr_test() {
    #[track_caller]
    fn foo<S: IsTStr>() {
        assert_type::<TStr<S::Arg>>(&S::VAL.to_tstr());
        assert_type::<TStr<S::Arg>>(&TStr::<S::Arg>::from_gen(S::VAL));

        must_panic(|| assert_type::<BString>(&S::VAL.to_tstr()));
        must_panic(|| assert_type::<BString>(&TStr::<S::Arg>::from_gen(S::VAL)));
    }

    foo::<AString>();
}

#[test]
fn conversion_from_generic_test() {
    #[track_caller]
    fn foo<S: IsTStr>() {
        let tstr = TStr::<S::Arg>::new();

        assert_type::<S>(&S::from_tstr(tstr));
        assert_type::<S>(&tstr.to_gen::<S>());

        must_panic(|| assert_type::<BString>(&S::from_tstr(tstr)));
        must_panic(|| assert_type::<BString>(&tstr.to_gen::<S>()));
    }

    foo::<AString>();
}

#[test]
fn cmp_test() {
    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: Ordering) {
        assert_eq!(L::VAL.tstr_cmp(R::VAL), expected, "trait");
        assert_eq!(tstr::cmp(L::VAL, R::VAL), expected, "free");
    }

    case::<AString, AString>(Ordering::Equal);
    case::<AString, BString>(Ordering::Less);
    case::<BString, AString>(Ordering::Greater);
    case::<BString, BString>(Ordering::Equal);
}

#[test]
fn eq_ne_test() {
    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: bool) {
        assert_eq!(L::VAL.tstr_eq(R::VAL), expected, "trait");
        assert_eq!(tstr::eq(L::VAL, R::VAL), expected, "free");

        assert_eq!(L::VAL.tstr_ne(R::VAL), !expected, "trait");
        assert_eq!(tstr::ne(L::VAL, R::VAL), !expected, "free");
    }

    case::<AString, AString>(true);
    case::<AString, BString>(false);
    case::<BString, AString>(false);
    case::<BString, BString>(true);
}

#[test]
fn len_tobytes_tostr_test() {
    #[track_caller]
    fn case<L: IsTStr>(expected: &str) {
        assert_eq!(L::VAL.to_str(), expected, "trait,to_str");
        assert_eq!(tstr::to_str(L::VAL), expected, "free,to_str");

        assert_eq!(L::VAL.to_bytes(), expected.as_bytes(), "trait,to_bytes");
        assert_eq!(tstr::to_bytes(L::VAL), expected.as_bytes(), "free,to_bytes");

        assert_eq!(L::VAL.len(), expected.len(), "trait,len");
        assert_eq!(tstr::len(L::VAL), expected.len(), "free,len");
    }

    case::<AString>(A_STRING);
    case::<BString>(B_STRING);
}

#[test]
fn type_eq_test() {
    struct IsEqual(bool);

    #[track_caller]
    fn case<L: IsTStr, R: IsTStr>(expected: IsEqual) {
        assert_eq!(L::VAL.type_eq(R::VAL).is_eq(), expected.0, "trait,len");
        assert_eq!(
            tstr::type_eq(L::VAL, R::VAL).is_eq(),
            expected.0,
            "free,len"
        );
    }

    case::<AString, AString>(IsEqual(true));
    case::<AString, BString>(IsEqual(false));
    case::<BString, AString>(IsEqual(false));
    case::<BString, BString>(IsEqual(true));
}
