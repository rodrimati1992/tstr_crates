use tstr::TStr;

const fn assert_traits<T>(_: T)
where
    T: 'static
        + tstr::strlike::StrLike
        + Copy
        + Clone
        + std::fmt::Debug
        + std::fmt::Display
        + Default
        + std::hash::Hash
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + Send
        + Sized
        + Sync
        + core::marker::Unpin
{}

const fn generic<T>(ts: TStr<T>) {
    assert_traits(ts);
}

const fn generic_bounded<T: tstr::TStrArg>(ts: TStr<T>) {
    assert_traits(ts);
}


fn main() {}








