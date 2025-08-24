use tstr::{TS, alias};

alias! {
    A = aaa;
    B = bbb;
    C = ccc;
    D = ddd;
}

#[test]
fn alias_and_tups() {
    let aa: TS!(aaa) = A;
    let bb: TS!(bbb) = B;
    let cc: TS!(ccc) = C;
    let dd: TS!(ddd) = D;

    let _: A = aa;
    let _: B = bb;
    let _: C = cc;
    let _: D = dd;
}
