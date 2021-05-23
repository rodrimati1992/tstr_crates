use super::{
    integers::{CmpCarry, Number2_5, D0, D1, D2, D3, D4},
    Equal,
};

/// Information about what kind of type-level value this is
#[doc(hidden)]
pub trait Classify {
    /// Whether this is a unit struct which represents a byte.
    type IsByte;

    /// A number used to encode the kind of thing this is
    ///
    /// It can be any of the `Chars*` or `Tuple*` type aliases.
    ///
    /// `Chars1` is used for the unit structs which represent bytes.
    ///
    /// `Chars*` is for the structs parameterized by char const parameters, to emulate char arrays.
    ///
    /// `Tuple*` is for tuples
    type KindNumber;
}

/// Helper trait which computes whether Self::KindNumber equals Rhs::KindNumber
#[doc(hidden)]
pub trait HasSameKindNumber<Rhs> {
    /// This can be either True or False
    type DoesIt;
}

impl<L, R, LD0, LD1, RD0, RD1> HasSameKindNumber<L> for R
where
    L: Classify<KindNumber = Number2_5<LD1, LD0>>,
    R: Classify<KindNumber = Number2_5<RD1, RD0>>,
    LD1: CmpCarry<Equal, RD1>,
    LD0: CmpCarry<LD1::Ord, RD0>,
{
    type DoesIt = LD0::Eq;
}

pub(crate) type Chars1 = Number2_5<D0, D0>;

#[allow(dead_code)]
pub(crate) type Chars2 = Number2_5<D0, D1>;

#[allow(dead_code)]
pub(crate) type Chars3 = Number2_5<D0, D2>;

#[allow(dead_code)]
pub(crate) type Chars4 = Number2_5<D0, D3>;

#[allow(dead_code)]
pub(crate) type Chars5 = Number2_5<D0, D4>;

#[allow(dead_code)]
pub(crate) type Chars6 = Number2_5<D1, D0>;

#[allow(dead_code)]
pub(crate) type Chars7 = Number2_5<D1, D1>;

#[allow(dead_code)]
pub(crate) type Chars8 = Number2_5<D1, D2>;

pub(crate) type Tuple0 = Number2_5<D1, D3>;
pub(crate) type Tuple1 = Number2_5<D1, D4>;
pub(crate) type Tuple2 = Number2_5<D2, D0>;
pub(crate) type Tuple3 = Number2_5<D2, D1>;
pub(crate) type Tuple4 = Number2_5<D2, D2>;
pub(crate) type Tuple5 = Number2_5<D2, D3>;
pub(crate) type Tuple6 = Number2_5<D2, D4>;
pub(crate) type Tuple7 = Number2_5<D3, D0>;
pub(crate) type Tuple8 = Number2_5<D3, D1>;
