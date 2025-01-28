pub trait DivCeil {
    /// $\lceil \frac{x}{y} \rceil$を計算する 小数点以下切り上げ
    /// NOTE: stdのdiv_ceilはrustc1.73から
    fn div_ceil_(self, rhs: Self) -> Self;
}

macro_rules! impl_signed {
    ($($ty:ident),*) => {$(
        impl DivCeil for $ty {
            fn div_ceil_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                if (self >= 0) == (rhs > 0) {
                    let lhs = self.abs();
                    let rhs = rhs.abs();
                    let d = lhs / rhs;
                    let r = lhs % rhs;
                    if r > 0 {
                        d + 1
                    } else {
                        d
                    }
                } else {
                    self / rhs
                }
            }
        }
    )*};
}

impl_signed! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_unsigned {
    ($($ty:ident),*) => {$(
        impl DivCeil for $ty {
            fn div_ceil_(self, rhs: Self) -> Self {
                assert!(rhs != 0);
                let d = self / rhs;
                let r = self % rhs;
                if r > 0 {
                    d + 1
                } else {
                    d
                }
            }
        }
    )*};
}

impl_unsigned! { u8, u16, u32, u64, u128, usize }

#[cfg(test)]
mod tests {
    use super::DivCeil;

    #[test]
    fn test_div_ceil_i32() {
        // (lhs, rhs, ans)
        let test_cases: Vec<(i32, i32, i32)> = vec![
            (0, 6, 0),
            (196816367, 408936581, 1),
            (1469330793, 211923681, 7),
            (524989232, 1521503399, 1),
            (1980481447, 743305495, 3),
            (994578483, 91868066, 11),
            (164452122, 233341757, 1),
            (892769521, 750125500, 2),
            (535341248, 1863750977, 1),
            (436725017, 10409309, 42),
            (2137006360, 1895544918, 2),
            (i32::MAX, i32::MAX, 1),
            (i32::MAX, 1, i32::MAX),
            (-7, 3, -2),
            (-9, 3, -3),
            (7, -3, -2),
            (9, -3, -3),
            (-7, -3, 3),
            (-9, -3, 3),
            (0, 5, 0),
            (0, -5, 0),
        ];
        for &(x, y, ans) in &test_cases {
            assert_eq!(x.div_ceil_(y), ans);
        }
    }

    #[test]
    fn test_div_ceil_u32() {
        // (lhs, rhs, ans)
        let test_cases: Vec<(u32, u32, u32)> = vec![
            (3242167662, 4067889619, 1),
            (3228637172, 2099291377, 2),
            (1627038402, 2683980054, 1),
            (3754350367, 1415834133, 3),
            (3429753677, 720984332, 5),
            (2829652990, 539026088, 6),
            (4003110728, 1367184956, 3),
            (2809049968, 2899476733, 1),
            (1752283686, 963336685, 2),
            (2791639793, 496148063, 6),
            (u32::MAX, u32::MAX, 1),
            (u32::MAX, 1, u32::MAX),
        ];
        for &(x, y, ans) in &test_cases {
            assert_eq!(x.div_ceil_(y), ans);
        }
    }

    #[test]
    fn test_div_ceil_i64() {
        // (lhs, rhs, ans)
        let test_cases: Vec<(i64, i64, i64)> = vec![
            (3743961514775493914, 7847710520604452858, 1),
            (3658785061199668031, 4941439423322782315, 1),
            (189632417364924173, 8997957903449639156, 1),
            (2791055268025099259, 2619683137426655400, 2),
            (1734403341746549421, 6923822879703219680, 1),
            (5077729266857648076, 5157158773342006445, 1),
            (4994375334885718818, 1657802092686856189, 4),
            (2750481411247349115, 8920783884410743561, 1),
            (6843860248342457318, 1641229231135444988, 5),
            (5622981829400302400, 4534692062369137460, 2),
            (i64::MAX, i64::MAX, 1),
            (i64::MAX, 1, i64::MAX),
        ];
        for &(x, y, ans) in &test_cases {
            assert_eq!(x.div_ceil_(y), ans);
        }
    }

    #[test]
    fn test_div_ceil_u64() {
        // (lhs, rhs, ans)
        let test_cases: Vec<(u64, u64, u64)> = vec![
            (2430709731614043528, 3632250489442770625, 1),
            (570109040799770453, 6702605256062026021, 1),
            (965384831889102938, 8268487403814979940, 1),
            (9206659733475386584, 3804307026053483427, 3),
            (17152143308950541775, 15576234526326580843, 2),
            (15332043456037348367, 9273459586063261272, 2),
            (1420003914580467124, 3059658280927341758, 1),
            (11700379081278641973, 12753136605423676479, 1),
            (13665743024848294134, 9100183548711498319, 2),
            (8436937732911287877, 11407520177484861707, 1),
            (u64::MAX, u64::MAX, 1),
            (u64::MAX, 1, u64::MAX),
        ];
        for &(x, y, ans) in &test_cases {
            assert_eq!(x.div_ceil_(y), ans);
        }
    }
}
