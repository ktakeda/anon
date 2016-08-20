#[macro_export]
macro_rules! anon {
    ( $($n:ident: $v:expr),+ ) => {
        {
            struct Anon<$($n,)*> {
                $($n: $n,)*
            };
            Anon { $($n: $v,)* }
        }
    };
}
