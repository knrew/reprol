/// 整数の平方根を計算する
/// 非負整数xに対して，isqrt(x)は x^2<=nを満たす最大の整数nを返す
/// 平方数x^2に対してはxを返すことが保証される
pub trait Isqrt {
    fn isqrt(self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Isqrt for $ty {
            #[allow(unused_comparisons)]
            fn isqrt(self) -> Self {
                debug_assert!(self >= 0);
                let mut l = 0;
                let mut r = self;
                while l <= r {
                    let mid = l + (r - l) / 2;
                    let mid2 = mid * mid;
                    if mid2 == self {
                        return mid;
                    } else if mid2 < self {
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                r
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
        let test_cases: Vec<(u32, u32)> = vec![
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
        ];

        for (x, ans) in test_cases {
            assert_eq!(x.isqrt(), ans);
        }
    }
}
