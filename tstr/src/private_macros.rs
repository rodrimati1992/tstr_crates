macro_rules! with_elem_count_idents {
    ($($macro:ident)::* !{$($prev_args:tt)*}) => (
        $($macro)::* !{
            $($prev_args)*

            (_0 _1 _2 _3 _4 _5 _6 _7)
        }
    )
}
pub(crate) use with_elem_count_idents;

macro_rules! with_elem_count_idents2 {
    ($($macro:ident)::* !{$($prev_args:tt)*}) => (
        $($macro)::* !{
            $($prev_args)*

            (_0 _1 _2 _3 _4 _5 _6 _7)
            (P0 P1 P2 P3 P4 P5 P6 P7)
        }
    )
}
pub(crate) use with_elem_count_idents2;
