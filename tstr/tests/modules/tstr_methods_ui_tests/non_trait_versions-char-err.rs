use tstr::TStr;


const fn unary<T: Copy>(ts: T) {
    _ = TStr::from_gen(ts);
    let _: T = TStr::new().to_gen();
    _ = tstr::len(ts);
    _ = tstr::to_bytes(ts);
    _ = tstr::to_str(ts);
}

const fn binary<L: Copy, R: Copy>(l: L, r: R) {
    _ = tstr::cmp(l, r);
    _ = tstr::ne(l, r);
    _ = tstr::eq(l, r);
    _ = tstr::type_eq(l, r);
}



fn main() {}






