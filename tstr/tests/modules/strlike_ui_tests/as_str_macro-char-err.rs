use tstr::strlike::StrLike;

const fn unsized_type_param<T: ?Sized>(x: &T) -> &str {
    tstr::strlike::as_str!(x)
}

const fn sized_type_param<U>(x: &U) -> &str {
    let _ = tstr::strlike::as_str!(x);
    tstr::strlike::as_str!(&x)
}

const fn unsized_type_param_works<V: ?Sized + StrLike>(x: &V) -> &str {
    tstr::strlike::as_str!(x)
}

fn main() {

}
