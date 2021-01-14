use tstr::{alias, ts, TS};

alias! {
    A = aaa;
    B = bbb;
    C = ccc;
    D = ddd;

    Tup2 = (aaa, bbb);
    Tup3 = (bbb, ccc, ddd);
}

#[test]
fn alias_and_tups() {
    let aa: TS!(aaa) = A;
    let bb: TS!(bbb) = B;
    let cc: TS!(ccc) = C;
    let dd: TS!(ddd) = D;

    let _: TS!(aaa, bbb) = Tup2;
    let _: TS!(bbb, ccc, ddd) = Tup3;

    let _: Tup2 = ts!(aaa, bbb);
    let _: Tup3 = ts!(bbb, ccc, ddd);

    let _: A = aa;
    let _: B = bb;
    let _: C = cc;
    let _: D = dd;

    let _: (A, B) = Tup2;
    let _: (B, C, D) = Tup3;
}
