use crate::integer::Integer;

/// x/yをする 小数点以下切り上げ
pub fn ceil_div<T: Integer>(x: T, y: T) -> T {
    (x + y - T::ONE) / y
}

pub fn gcd<T: Integer>(m: T, n: T) -> T {
    if n == T::ZERO {
        abs(m)
    } else {
        gcd(n, m % n)
    }
}

pub fn lcm<T: Integer>(m: T, n: T) -> T {
    abs(m) / gcd(m, n) * abs(n)
}

#[inline]
fn abs<T: Integer>(n: T) -> T {
    if n < T::ZERO {
        T::ZERO - n
    } else {
        n
    }
}

/// x^e
pub fn pow<T: Integer>(mut x: T, mut e: T) -> T {
    if e == T::ZERO {
        return T::ONE;
    }

    let mut res = T::ONE;

    while e > T::ONE {
        if (e & T::ONE) == T::ONE {
            res = res * x;
        }
        e /= T::TWO;
        x = x * x;
    }
    res *= x;

    res
}

#[cfg(test)]
mod tests {
    use crate::math::{gcd, lcm};

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(101, 103), 1);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(48u32, 18u32), 6);
        assert_eq!(gcd(54u64, 24u64), 6);
        assert_eq!(gcd(-48, -18), 6);
        assert_eq!(gcd(-54, 24), 6);
        assert_eq!(gcd(-101, -103), 1);
        assert_eq!(
            gcd(1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            500_000_000_000_000_000u128
        );
        assert_eq!(gcd(42, 42), 42);
        assert_eq!(gcd(-42, -42), 42);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(7, 3), 21);
        assert_eq!(lcm(10, 15), 30);
        assert_eq!(lcm(7u32, 3u32), 21);
        assert_eq!(lcm(9u64, 6u64), 18);
        assert_eq!(lcm(-4, 5), 20);
        assert_eq!(lcm(-6, -8), 24);
        assert_eq!(lcm(-7, 3), 21);
        assert_eq!(
            lcm(1_000_000_000_000_000_000u128, 500_000_000_000_000_000u128),
            1_000_000_000_000_000_000u128
        );
        assert_eq!(lcm(42, 42), 42);
        assert_eq!(lcm(-42, -42), 42);
    }
}
