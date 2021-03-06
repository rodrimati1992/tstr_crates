use tstr::{TStrEq, TS};

#[cfg(feature = "const_generics")]
use std::cmp::{Ord, Ordering};

#[cfg(feature = "const_generics")]
use tstr::{StrValue, TStrOrd};

macro_rules! assert_str_eq {
    ($left:ty, $right:ty) => {
        assert!(<$left as TStrEq<$right>>::EQ);

        #[cfg(feature = "const_generics")]
        assert_eq!(<$left as TStrOrd<$right>>::CMP, Ordering::Equal);
    };
}

macro_rules! assert_str_ne {
    ($left:ty, [$($right:ty),* $(,)*]) => {
        $(assert!(<$left as TStrEq<$right>>::NE);)*

        #[cfg(feature = "const_generics")]
        {
            $(
                assert_eq!(
                    <$left as TStrOrd<$right>>::CMP,
                    <$left as StrValue>::STR.cmp(<$right as StrValue>::STR)
                );
            )*
        }
    };
}

type Len0 = TS!("");

type Len1A = TS!("a");
type Len1B = TS!("b");

type Len2A = TS!("aa");
type Len2B = TS!("ab");

type Len3A = TS!("aaa");
type Len3B = TS!("aab");

type Len4A = TS!("aaaa");
type Len4B = TS!("aaab");

type Len5A = TS!("aaaaa");
type Len5B = TS!("aaaab");

type Len6A = TS!("aaaaaa");
type Len6B = TS!("aaabaa");

type Len7A = TS!("aaaaaaa");
type Len7B = TS!("abaaaaa");

type Len8A = TS!("aaaaaaaa");
type Len8B = TS!("aabaaaaa");

type Len9A = TS!("aaaaaaaaa");
type Len9B = TS!("aabaaaaaa");

type Len17A = TS!("-aaaaaaa-aaaaaaa-");
type Len17B = TS!("-aaaaaaa-aaaabaa-");

type Len25A = TS!("-aaaaaaa-____aaa-aaaaaaa-");
type Len25B = TS!("-aaaaaaa-aaaaaaa-aaaaaaa-");

type Len63A = TS!("aaaaaaa-aaaaaaa-aaaaaaa-_______-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa");
type Len63B = TS!("aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa");

type Len64A = TS!("-aaaaaaa-aaaaaaa-_______-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa");
type Len64B = TS!("-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa");

type Len65A = TS!("-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-");
type Len65B = TS!("-aaaaaaa-aaaaaaa-aaaaaaa-_______-aaaaaaa-aaaaaaa-aaaaaaa-aaaaaaa-");

#[test]
fn equal_strs() {
    assert_str_eq!(Len0, Len0);
    assert_str_eq!(Len1A, Len1A);
    assert_str_eq!(Len1B, Len1B);
    assert_str_eq!(Len2A, Len2A);
    assert_str_eq!(Len2B, Len2B);
    assert_str_eq!(Len3A, Len3A);
    assert_str_eq!(Len3B, Len3B);
    assert_str_eq!(Len4A, Len4A);
    assert_str_eq!(Len4B, Len4B);
    assert_str_eq!(Len5A, Len5A);
    assert_str_eq!(Len5B, Len5B);
    assert_str_eq!(Len6A, Len6A);
    assert_str_eq!(Len6B, Len6B);
    assert_str_eq!(Len7A, Len7A);
    assert_str_eq!(Len7B, Len7B);
    assert_str_eq!(Len8A, Len8A);
    assert_str_eq!(Len8B, Len8B);
    assert_str_eq!(Len9A, Len9A);
    assert_str_eq!(Len9B, Len9B);
    assert_str_eq!(Len17A, Len17A);
    assert_str_eq!(Len17B, Len17B);
    assert_str_eq!(Len25A, Len25A);
    assert_str_eq!(Len25B, Len25B);
    assert_str_eq!(Len63A, Len63A);
    assert_str_eq!(Len63B, Len63B);
    assert_str_eq!(Len64A, Len64A);
    assert_str_eq!(Len64B, Len64B);
    assert_str_eq!(Len65A, Len65A);
    assert_str_eq!(Len65B, Len65B);
}

#[test]
fn short_strs() {
    // assert_str_ne!(Len3A, [
    //     Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A,
    //     Len9A, Len17A, Len25A, Len63A, Len64A, Len65A,
    // ]);

    assert_str_ne!(
        Len0,
        [
            Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A, Len63A,
            Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len1A,
        [
            Len0, Len1B, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len2A,
        [
            Len0, Len1A, Len2B, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len3A,
        [
            Len0, Len1A, Len2A, Len3B, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len4A,
        [
            Len0, Len1A, Len2A, Len3A, Len4B, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len5A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5B, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len6A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6B, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len7A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7B, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len8A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8B, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len9A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9B, Len17A, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len17A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17B, Len25A,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len25A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25B,
            Len63A, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len63A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63B, Len64A, Len65A,
        ]
    );

    assert_str_ne!(
        Len64A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64B, Len65A,
        ]
    );

    assert_str_ne!(
        Len65A,
        [
            Len0, Len1A, Len2A, Len3A, Len4A, Len5A, Len6A, Len7A, Len8A, Len9A, Len17A, Len25A,
            Len63A, Len64A, Len65B,
        ]
    );
}
