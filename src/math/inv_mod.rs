pub trait InvMod {
    /// 法pにおける逆元を計算する
    fn inv_mod(self, p: Self) -> Self;
}

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl InvMod for $ty {
            fn inv_mod(self, p: Self) -> Self {
                debug_assert!(self > 0);
                debug_assert!(p > 0);
                if self == 1 {
                    return 1;
                }
                p + (1 - p * (p % self).inv_mod(self)) / self
            }
        }
    )*};
}

impl_signed! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl InvMod for $ty {
            fn inv_mod(self, p: Self) -> Self {
                (self as i64).inv_mod(p as i64) as $ty
            }
        }
    )*};
}

impl_unsigned! { u8, u16, u32, u64, usize }

impl InvMod for u128 {
    fn inv_mod(self, p: Self) -> Self {
        (self as i128).inv_mod(p as i128) as u128
    }
}

#[cfg(test)]
mod tests {
    use super::InvMod;

    #[test]
    fn test_inv_mod() {
        const P1: u64 = 998244353;
        const P2: u64 = 1000000007;

        let test_cases = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 123456789, 987654321, 500000000, 400000000, 99999999,
            876543210, 998244352, 1000000010, 5420274012, 51307341, 177174101, 272154126,
            285120554, 310046136, 512696315, 537364739, 606810056, 703996446, 808398679, 93762712,
            126607016, 126882966, 169157861, 431575151, 489724038, 667652900, 735396744, 931229540,
            966373973, 1000000006,
        ];

        for &x in &test_cases {
            let x_inv = x.inv_mod(P1);
            assert_eq!((x % P1 * x_inv % P1), 1);

            let x_inv = x.inv_mod(P2);
            assert_eq!((x % P2 * x_inv % P2), 1);
        }
    }
}
