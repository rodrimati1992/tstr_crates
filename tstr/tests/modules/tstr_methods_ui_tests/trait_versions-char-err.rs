use tstr::{IsTStr, TStr};


const fn unary<T: Copy>(ts: T) {
    _ = ts.to_tstr();
    let _: T = T::from_tstr(TStr::new());
    _ = ts.len();
    _ = ts.to_bytes();
    _ = ts.to_str();
}

const fn binary<L: Copy, R: Copy>(l: L, r: R) {
    _ = l.tstr_cmp(r);
    _ = l.tstr_ne(r);
    _ = l.tstr_eq(r);
    _ = l.type_eq(r);
}



fn main() {}






