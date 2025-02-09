use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::math::gcd::Gcd;

// 有理数(分数)を扱うライブラリ
// NOTE: 整備不十分
#[derive(Clone, Copy)]
pub struct Ratio {
    num: i64,
    den: i64,
}

impl Ratio {
    pub fn new(num: i64, den: i64) -> Self {
        assert_ne!(den, 0);
        let g = num.gcd(den);
        let (num, den) = (num.signum() * den.signum() * num.abs() / g, den.abs() / g);
        Self { num, den }
    }

    /// 分子
    pub fn num(&self) -> i64 {
        self.num
    }

    /// 分母
    pub fn den(&self) -> i64 {
        self.den
    }

    /// 浮動小数点数に変換する
    pub fn get(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn is_positive(&self) -> bool {
        self.num > 0
    }

    pub fn is_negative(&self) -> bool {
        self.num < 0
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == other.num * self.den
    }
}

impl Eq for Ratio {}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}

impl Add for Ratio {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let den = self.den * rhs.den;
        let num = self.num * rhs.den + rhs.num * self.den;
        Self::new(num, den)
    }
}

impl AddAssign for Ratio {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Ratio {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let den = self.den * rhs.den;
        let num = self.num * rhs.den - rhs.num * self.den;
        Self::new(num, den)
    }
}

impl SubAssign for Ratio {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Ratio {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl MulAssign for Ratio {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Ratio {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

impl DivAssign for Ratio {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Sum for Ratio {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0, 1), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Self> for Ratio {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for Ratio {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1, 1), |acc, x| acc * x)
    }
}

impl<'a> Product<&'a Self> for Ratio {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl Debug for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl From<$ty> for Ratio {
            fn from(x: $ty) -> Ratio {
                Ratio::new(x as i64, 1)
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::Ratio;

    #[test]
    fn test_ratio_add() {
        // ((a, b), (c, d), (x, y)): a/b+c/d=x/y
        let test_cases = vec![
            //
            ((2, 12), (6, 16), (13, 24)),
        ];

        for ((a, b), (c, d), (expected_num, expected_den)) in test_cases {
            let x = Ratio::new(a, b);
            let y = Ratio::new(c, d);
            let z = x + y;
            assert_eq!(z.num, expected_num);
            assert_eq!(z.den, expected_den);
        }
    }
}
