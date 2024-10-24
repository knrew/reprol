pub trait PowMod {
    /// 法pのもとで冪乗を計算する
    fn pow_mod(self, exp: Self, p: Self) -> Self;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl PowMod for $ty {
            #[allow(unused_comparisons)]
            fn pow_mod(self, mut exp: Self, p: Self) -> Self {
                debug_assert!(self >= 0);
                debug_assert!(exp >= 0);
                debug_assert!(p >= 0);

                if p == 1 {
                    return 0;
                }

                let mut res = 1;
                let mut base = self % p;

                while exp > 0 {
                    if exp & 1 == 1 {
                        res = res * base % p;
                    }
                    base = base * base % p;
                    exp >>= 1;
                }

                res
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::PowMod;

    #[test]
    fn test_pow_mod() {
        const P1: u64 = 998244353;
        const P2: u64 = 1000000007;

        // (base, exp, ans1, ans2)の順で並んでいる
        // base^expを計算する
        // ans1, ans2はそれぞれ法P1, P2での答え
        let test_cases = vec![
            (0, 0, 1, 1),
            (10, 0, 1, 1),
            (0, 12, 0, 0),
            (2, 10, 1024, 1024),
            (3, 5, 243, 243),
            (5, 3, 125, 125),
            (7, 4, 2401, 2401),
            (123, 456, 500543741, 565291922),
            (987654321, 2, 17678886, 961743691),
            (1000000006, 100, 308114436, 1),
            (999, 999, 117436213, 760074701),
            (500, 500, 650576768, 742761597),
            (2, 998244352, 1, 106733835),
            (2, 1000000006, 565485962, 1),
            (35159992, 853659348, 171826619, 73025258),
            (173744080, 972168833, 562413643, 338142216),
            (258912740, 518302010, 763696358, 868359857),
            (561083107, 110854587, 592288248, 136419826),
            (578612337, 331137309, 165640937, 170496686),
            (595763466, 176515871, 635087261, 802111797),
            (633335045, 18529847, 929415341, 539935827),
            (723091847, 451729607, 531431947, 242080099),
            (775348050, 914965051, 833671373, 960043753),
            (947772619, 548149867, 577212826, 184934494),
            (930769844, 4294967295, 517902255, 190677013),
        ];

        for &(base, exp, ans1, ans2) in &test_cases {
            assert_eq!(base.pow_mod(exp, P1), ans1);
            assert_eq!(base.pow_mod(exp, P2), ans2);
        }
    }
}
