use tstr::{TS, alias};

alias! {
    A = aaa;
    B = bbb;
    C = ccc;
    D = ddd;
    Concatenated = concat!(foo, bar);
    Stringified = stringify!(==);
}

#[test]
fn aliases() {
    let aa: TS!(aaa) = A;
    let bb: TS!(bbb) = B;
    let cc: TS!(ccc) = C;
    let dd: TS!(ddd) = D;
    let concatenated: TS!(foobar) = Concatenated;
    let stringified: TS!(stringify!(==)) = Stringified;

    let _: A = aa;
    let _: B = bb;
    let _: C = cc;
    let _: D = dd;
    let _: Concatenated = concatenated;
    let _: Stringified = stringified;

    assert_eq!(tstr::to_str(A), "aaa");
    assert_eq!(tstr::to_str(B), "bbb");
    assert_eq!(tstr::to_str(C), "ccc");
    assert_eq!(tstr::to_str(D), "ddd");
    assert_eq!(tstr::to_str(Concatenated), "foobar");
    assert_eq!(tstr::to_str(Stringified), "==");
}
