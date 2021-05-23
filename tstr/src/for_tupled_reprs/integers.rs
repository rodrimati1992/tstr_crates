use super::{Equal, False, Greater, Less, True};

/// A 2 digit, base 5 number
pub struct Number2_5<D1, D0>(D1, D0);

// Digit for 0 in base 5
pub struct D0;

// Digit for 1 in base 5
pub struct D1;

// Digit for 2 in base 5
pub struct D2;

// Digit for 3 in base 5
pub struct D3;

// Digit for 4 in base 5
pub struct D4;

/// For comparing digits, while taking into account the previous digit.
pub trait CmpCarry<PrevOrd, R> {
    /// Whether Self equals R. Either True or False.
    type Eq;
    /// The ordering of Self compared to R.
    ///
    /// Represented using the Less, Equal, and Greater unit structs
    /// in the for_tupled_reprs module.
    type Ord;
}

impl<L, R> CmpCarry<Less, R> for L {
    type Eq = False;
    type Ord = Less;
}

impl<L, R> CmpCarry<Greater, R> for L {
    type Eq = False;
    type Ord = Greater;
}

/*
use std::cmp::{Ord, Ordering};

use itertools::iproduct;

fn main() {
    let prod = iproduct!(0..5, 0..5);
    {
        let mut prod = prod.clone();
        for (i, (a, b)) in prod.by_ref().take(8).enumerate() {
            println!("pub(crate) type Chars{} = Number2_5<D{}, D{};", i + 1, a, b);
        }
        for (i, (a, b)) in prod.by_ref().enumerate() {
            println!("pub(crate) type Tuple{} = Number2_5<D{}, D{}>;", i, a, b);
        }
    }
    println!();
    println!();
    {
        for (a, b) in prod.clone() {
            let (eq, ord) = match a.cmp(&b) {
                Ordering::Less => ("False", "Less"),
                Ordering::Equal => ("True", "Equal"),
                Ordering::Greater => ("False", "Greater"),
            };

            println!("impl CmpCarry<Equal, D{b}> for D{a} {{", a = a, b = b);
            println!("    type Eq = {};", eq);
            println!("    type Ord = {};", ord);
            println!("}}");
        }
    }
}
*/

impl CmpCarry<Equal, D0> for D0 {
    type Eq = True;
    type Ord = Equal;
}
impl CmpCarry<Equal, D1> for D0 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D2> for D0 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D3> for D0 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D4> for D0 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D0> for D1 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D1> for D1 {
    type Eq = True;
    type Ord = Equal;
}
impl CmpCarry<Equal, D2> for D1 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D3> for D1 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D4> for D1 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D0> for D2 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D1> for D2 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D2> for D2 {
    type Eq = True;
    type Ord = Equal;
}
impl CmpCarry<Equal, D3> for D2 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D4> for D2 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D0> for D3 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D1> for D3 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D2> for D3 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D3> for D3 {
    type Eq = True;
    type Ord = Equal;
}
impl CmpCarry<Equal, D4> for D3 {
    type Eq = False;
    type Ord = Less;
}
impl CmpCarry<Equal, D0> for D4 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D1> for D4 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D2> for D4 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D3> for D4 {
    type Eq = False;
    type Ord = Greater;
}
impl CmpCarry<Equal, D4> for D4 {
    type Eq = True;
    type Ord = Equal;
}
