mod numeric {
    use std::{
        fmt::Debug,
        ops::{Add, BitAnd, BitOr, Div, Mul, Rem, Shl, Shr, Sub},
    };

    pub fn is_prime<T: Integer>(n: T) -> bool {
        if n <= T::ONE {
            return false;
        }

        let mut i = T::TWO;
        while i * i <= n {
            if n % i == T::ZERO {
                return false;
            }
            i = i + T::ONE;
        }

        true
    }

    pub fn enumrate_divisors<T: Integer>(n: T) -> Vec<T> {
        let mut divisors = vec![];

        let mut i = T::ONE;
        while i * i <= n {
            if n % i == T::ZERO {
                divisors.push(i);
                if n / i != i {
                    divisors.push(n / i);
                }
            }
            i = i + T::ONE;
        }

        divisors.sort();
        divisors
    }

    pub fn factorize<T: Integer>(n: T) -> Vec<(T, usize)> {
        let mut n = n;

        let mut factors = vec![];

        let mut i = T::TWO;
        while i * i <= n {
            let mut ex = 0;
            while n % i == T::ZERO {
                ex += 1;
                n = n / i;
            }

            if ex != 0 {
                factors.push((i, ex));
            }

            i = i + T::ONE;
        }

        if n != T::ONE {
            factors.push((n, 1));
        }

        factors
    }

    pub trait Integer:
        Sized
        + Copy
        + Ord
        + PartialOrd
        + Debug
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
    {
        const ZERO: Self;
        const ONE: Self;
        const TWO: Self;
        const MIN: Self;
        const MAX: Self;
    }

    macro_rules! impl_integer {
        ($($ty:ident),*) => {$(
            impl Integer for $ty {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MIN: Self = std::$ty::MIN;
                const MAX: Self = std::$ty::MAX;
            }
        )*};
    }

    impl_integer! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize }
}

#[cfg(test)]
mod tests {
    use super::numeric::*;

    #[test]
    fn test_is_prime() {
        let correspondence = [
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
            (12, false),
        ];

        for &(n, ans) in &correspondence {
            assert_eq!(is_prime(n), ans);
        }
    }
}
