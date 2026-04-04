tstr::alias! {
    Foo = "foo";
    N0 = "0";
    N34 = "34";
}

macro_rules! deserialize_err {
    ($ty:ty, $from_str:expr) => {
        serde_json::from_str::<$ty>($from_str)
            .map_err(|e| e.to_string())
            .unwrap_err()
    };
}

#[test]
fn test_deserialize_str() {
    let nonstatic = String::from(r#""foo""#);

    assert_eq!(serde_json::from_str::<Foo>(&nonstatic).ok(), Some(Foo));
    assert_eq!(serde_json::from_str::<N34>(r#""34""#).ok(), Some(N34));

    assert_eq!(serde_json::from_str::<N34>(r#""33""#).ok(), None);
    assert_eq!(serde_json::from_str::<Foo>(r#""1""#).ok(), None);

    let err = deserialize_err!(Foo, "1");
    assert!(err.contains(r#"expected the value `"foo"`"#), "{err:?}");

    assert_eq!(serde_json::from_str::<Foo>("foo").ok(), None);
    assert_eq!(serde_json::from_str::<Foo>("{}").ok(), None);
    assert_eq!(serde_json::from_str::<Foo>("[]").ok(), None);
}

#[test]
fn test_serialize_str() {
    assert_eq!(serde_json::to_string(&N0).unwrap(), r#""0""#);
    assert_eq!(serde_json::to_string(&N34).unwrap(), r#""34""#);
    assert_eq!(serde_json::to_string(&Foo).unwrap(), r#""foo""#);
}
