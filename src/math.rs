mod math {
    use std::{
        fmt::Debug,
        ops::{Add, BitAnd, BitOr, Div, Mul, Rem, Shl, Shr, Sub},
    };

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

    fn abs<T: Integer>(n: T) -> T {
        if n < T::ZERO {
            T::ZERO - n
        } else {
            n
        }
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
