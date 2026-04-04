use core::marker::PhantomData;

use crate::{IsTStr, TStr, TStrArg};

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "serde")))]
impl<'de, SA: TStrArg> serde::Deserialize<'de> for TStr<SA> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitTStr<T>(PhantomData<T>);

        impl<'de, S: IsTStr> serde::de::Visitor<'de> for VisitTStr<S> {
            type Value = S;

            fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(fmt, "the value `{:?}`", S::STR)
            }

            fn visit_str<E>(self, found: &str) -> Result<S, E>
            where
                E: serde::de::Error,
            {
                if S::STR == found {
                    Ok(S::VAL)
                } else {
                    Err(serde::de::Error::custom(format_args!(
                        "expected `{:?}` found `{:?}`",
                        S::STR,
                        found,
                    )))
                }
            }
        }

        deserializer.deserialize_str(VisitTStr(PhantomData::<Self>))
    }
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "serde")))]
impl<SA: TStrArg> serde::Serialize for TStr<SA> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&Self::STR, serializer)
    }
}
