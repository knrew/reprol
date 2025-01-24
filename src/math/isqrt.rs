use crate::bisect::Bisect;

pub trait Isqrt {
    /// 二分探索によって整数の平方根を計算する
    /// 非負整数xに対して，isqrt(x)は x^2<=nを満たす最大の整数nを返す
    /// 平方数x^2に対してはxを返すことが保証される
    fn isqrt(self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Isqrt for $ty {
            #[allow(unused_comparisons)]
            fn isqrt(self) -> Self {
                assert!(self >= 0);
                if self == 0 {
                    0
                } else {
                    (1..self + 1).bisect(|&x| match x.checked_mul(x) {
                        Some(xx) if xx <= self => true,
                        _ => false,
                    }) - 1
                }
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::Isqrt;

    #[test]
    fn test_isqrt() {
        let test_cases: Vec<(u64, u64)> = vec![
            (0, 0),
            (1, 1),
            (4, 2),
            (9, 3),
            (11, 3),
            (14, 3),
            (16, 4),
            (19, 4),
            (20, 4),
            (21, 4),
            (22, 4),
            (25, 5),
            (28, 5),
            (31, 5),
            (33, 5),
            (36, 6),
            (37, 6),
            (43, 6),
            (45, 6),
            (47, 6),
            (49, 7),
            (100, 10),
            (9000000000000000000, 3000000000),
            (9000000000000000001, 3000000000),
        ];

        for (x, ans) in test_cases {
            assert_eq!(x.isqrt(), ans);
        }
    }
}
