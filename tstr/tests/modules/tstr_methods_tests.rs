use crate::modules::utils::{assert_type, must_panic};

use tstr::{IsTStr, TS, TStr};

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
        let tstr = TStr::<S::Arg>::new();

        assert_type::<S>(&tstr.to_gen::<S>());

        must_panic(|| assert_type::<BString>(&tstr.to_gen::<S>()));
    }

    foo::<AString>();
}
