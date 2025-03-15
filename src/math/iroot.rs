use crate::bisect::Bisect;

pub trait Isqrt: Sized {
    /// $\lfloor \sqrt{x} \rfloor$
    fn isqrt(self) -> Self {
        self.iroot_nth(2)
    }

    /// $\lfloor \sqrt[3]{x} \rfloor$
    fn icbrt(self) -> Self {
        self.iroot_nth(3)
    }

    /// 非負整数$x$に対して$n$乗根の整数部分$\lfloor \sqrt[n]{x} \rfloor$を計算する
    /// $n$乗数$x^n$に対しては$x$を返すことが保証される
    fn iroot_nth(self, n: u32) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Isqrt for $ty {
            #[allow(unused_comparisons)]
            fn iroot_nth(self, n: u32) -> Self {
                assert!(self >= 0);
                if self == 0 {
                    0
                } else {
                    (1..self + 1).bisect(|&x| match x.checked_pow(n) {
                        Some(xn) if xn <= self => true,
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
        let testcases = vec![
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
            (2147395600, 46340),
            (2147483646, 46340),
        ];

        for (x, expected) in testcases {
            assert_eq!(x.isqrt(), expected);
        }

        let testcases = vec![
            (9000000000000000000u64, 3000000000),
            (9000000000000000001, 3000000000),
            (18446744065119617024, 4294967294),
            (18446744065119617025, 4294967295),
            (18446744073709551614, 4294967295),
        ];
        for (x, expected) in testcases {
            assert_eq!(x.isqrt(), expected);
        }
    }

    #[test]
    fn test_icbrt() {
        let testcases = vec![
            (0, 0),
            (1, 1),
            (3, 1),
            (6, 1),
            (8, 2),
            (9, 2),
            (16, 2),
            (27, 3),
            (39, 3),
            (50, 3),
            (51, 3),
            (54, 3),
            (57, 3),
            (62, 3),
            (64, 4),
            (68, 4),
            (69, 4),
            (70, 4),
            (75, 4),
            (77, 4),
            (81, 4),
            (83, 4),
            (84, 4),
            (92, 4),
            (94, 4),
            (100, 4),
            (124, 4),
            (125, 5),
            (126, 5),
            (215, 5),
            (216, 6),
            (217, 6),
            (2146689000, 1290),
            (2147483646, 1290),
        ];

        for (x, expected) in testcases {
            assert_eq!(x.icbrt(), expected);
        }

        let testcases = vec![
            (18446724184312856124u64, 2642244),
            (18446724184312856125, 2642245),
            (18446744073709551614, 2642245),
        ];
        for (x, expected) in testcases {
            assert_eq!(x.icbrt(), expected);
        }
    }
}
