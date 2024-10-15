use crate::integer::Integer;

/// x/yをする 小数点以下切り上げ
/// NOTE:負のときは未定義
pub fn ceil_div<T: Integer>(x: T, y: T) -> T {
    debug_assert!(x >= T::ZERO);
    debug_assert!(y >= T::ONE);
    (x + y - T::ONE) / y
}

/// 繰り返し二乗法による冪乗の計算
pub fn pow<T: Integer>(mut base: T, mut exp: T) -> T {
    debug_assert!(base >= T::ZERO);
    debug_assert!(exp >= T::ZERO);

    if exp == T::ZERO {
        return T::ONE;
    }

    let mut res = T::ONE;

    while exp > T::ONE {
        if (exp & T::ONE) == T::ONE {
            res = res * base;
        }
        exp /= T::TWO;
        base = base * base;
    }
    res *= base;

    res
}

#[cfg(test)]
mod tests {
    use super::{ceil_div, pow};

    #[test]
    fn test_ceil_div() {
        assert_eq!(ceil_div(10, 2), 5);
        assert_eq!(ceil_div(100, 5), 20);
        assert_eq!(ceil_div(10, 3), 4);
        assert_eq!(ceil_div(7, 2), 4);
        assert_eq!(ceil_div(15, 1), 15);
        assert_eq!(ceil_div(0, 1), 0);
        assert_eq!(ceil_div(0, 5), 0);
        assert_eq!(ceil_div(0, 100), 0);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 3), 8);
        assert_eq!(pow(5, 0), 1);
        assert_eq!(pow(7, 1), 7);
        assert_eq!(pow(3, 4), 81);
        assert_eq!(pow(0, 5), 0);
        assert_eq!(pow(0, 0), 1);
        assert_eq!(pow(2, 30), 1073741824);
        assert_eq!(pow(10, 9), 1000000000);
    }
}
