//! NonNanFloat
//!
//! Nanでないf64について扱うモジュール．
//! fOrdやHashが実装されたf64．
//!
//! ```
//! use reprol::nonnan_float::NonNanFloat;
//! let x = NonNanFloat::new(6.0);
//! let y = NonNanFloat::new(3.0);
//! assert_eq!((x + y).inner(), 9.0);
//! assert_eq!((x - y).inner(), 3.0);
//! assert_eq!((x * y).inner(), 18.0);
//! ```

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct NonNanFloat {
    inner: f64,
}

impl NonNanFloat {
    #[inline]
    pub fn new(value: f64) -> Self {
        assert!(!value.is_nan());
        Self { inner: value }
    }

    #[inline]
    pub fn inner(&self) -> f64 {
        self.inner
    }
}

impl Eq for NonNanFloat {}

impl PartialOrd for NonNanFloat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonNanFloat {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.partial_cmp(&other.inner).unwrap()
    }
}

impl Hash for NonNanFloat {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut bits = self.inner.to_bits();
        if bits == (-0.0f64).to_bits() {
            bits = 0.0f64.to_bits();
        }
        bits.hash(state);
    }
}

impl Add for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let res = self.inner + rhs.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl AddAssign for NonNanFloat {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let res = self.inner - rhs.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl SubAssign for NonNanFloat {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl Mul for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let res = self.inner * rhs.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl MulAssign for NonNanFloat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl Div for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let res = self.inner / rhs.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl DivAssign for NonNanFloat {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl Rem for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        let res = self.inner % rhs.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl RemAssign for NonNanFloat {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl Neg for NonNanFloat {
    type Output = NonNanFloat;
    #[inline]
    fn neg(self) -> Self::Output {
        let res = -self.inner;
        assert!(!res.is_nan());
        NonNanFloat { inner: res }
    }
}

impl Sum for NonNanFloat {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(NonNanFloat { inner: 0.0 }, |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Self> for NonNanFloat {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for NonNanFloat {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(NonNanFloat { inner: 1.0 }, |acc, x| acc * x)
    }
}

impl<'a> Product<&'a Self> for NonNanFloat {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl From<f64> for NonNanFloat {
    #[inline]
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<NonNanFloat> for f64 {
    #[inline]
    fn from(value: NonNanFloat) -> Self {
        value.inner()
    }
}

impl Debug for NonNanFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Display for NonNanFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::NonNanFloat;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::{
        cmp::Ordering,
        collections::{hash_map::DefaultHasher, HashSet},
        hash::{Hash, Hasher},
    };

    #[test]
    fn test_new() {
        let value = NonNanFloat::new(1.5);
        assert_eq!(value.inner(), 1.5);

        let infinite = NonNanFloat::new(f64::INFINITY);
        assert!(f64::is_infinite(infinite.inner()));
    }

    #[test]
    #[should_panic]
    fn test_new_nan() {
        let _ = NonNanFloat::new(f64::NAN);
    }

    #[test]
    fn test_op() {
        let a = NonNanFloat::new(6.0);
        let b = NonNanFloat::new(2.0);

        assert_eq!((a + b).inner(), 8.0);
        assert_eq!((a - b).inner(), 4.0);
        assert_eq!((a * b).inner(), 12.0);
        assert_eq!((a / b).inner(), 3.0);
        assert_eq!((a % b).inner(), 0.0);
        assert_eq!((-a).inner(), -6.0);

        let mut acc = a;
        acc += b;
        assert_eq!(acc.inner(), 8.0);
        acc -= b;
        assert_eq!(acc.inner(), 6.0);
        acc *= b;
        assert_eq!(acc.inner(), 12.0);
        acc /= b;
        assert_eq!(acc.inner(), 6.0);
        acc %= b;
        assert_eq!(acc.inner(), 0.0);

        let c = NonNanFloat::new(-4.5);
        let d = NonNanFloat::new(1.5);
        assert_eq!((c + d).inner(), -3.0);
        assert_eq!((c - d).inner(), -6.0);
        assert_eq!((c * d).inner(), -6.75);
        assert_eq!((c / d).inner(), -3.0);

        let mut mixed = d;
        mixed += c;
        assert_eq!(mixed.inner(), -3.0);
        mixed -= c;
        assert_eq!(mixed.inner(), 1.5);
        mixed *= NonNanFloat::new(2.0);
        assert_eq!(mixed.inner(), 3.0);
        mixed /= NonNanFloat::new(-3.0);
        assert_eq!(mixed.inner(), -1.0);
        mixed %= NonNanFloat::new(0.4);
        assert_eq!(mixed.inner(), (-1.0f64) % 0.4);

        let neg_zero = -NonNanFloat::new(0.0);
        assert_eq!(neg_zero.inner().to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn test_sum_product() {
        let values = [
            NonNanFloat::new(1.0),
            NonNanFloat::new(2.0),
            NonNanFloat::new(3.5),
        ];

        assert_eq!(values.into_iter().sum::<NonNanFloat>().inner(), 6.5);

        let values = [
            NonNanFloat::new(2.0),
            NonNanFloat::new(3.0),
            NonNanFloat::new(4.0),
        ];
        assert_eq!(values.iter().sum::<NonNanFloat>().inner(), 9.0);
        assert_eq!(values.iter().product::<NonNanFloat>().inner(), 24.0);
    }

    #[test]
    fn test_from() {
        let x = NonNanFloat::new(0.25);
        let y: NonNanFloat = 0.25f64.into();
        assert_eq!(x, y);

        let x = 0.4f64;
        let y: f64 = NonNanFloat::new(0.4).into();
        assert_eq!(x, y);
    }

    #[test]
    fn test_ord() {
        let mut values = vec![
            NonNanFloat::new(3.0),
            NonNanFloat::new(-0.0),
            NonNanFloat::new(1.0),
        ];
        values.sort();
        assert_eq!(
            values,
            vec![
                NonNanFloat::new(-0.0),
                NonNanFloat::new(1.0),
                NonNanFloat::new(3.0)
            ]
        );

        assert_eq!(
            NonNanFloat::new(-0.0).cmp(&NonNanFloat::new(0.0)),
            Ordering::Equal
        );
        assert!(NonNanFloat::new(-5.0) < NonNanFloat::new(-1.0));
        assert!(NonNanFloat::new(1.0) < NonNanFloat::new(f64::INFINITY));

        let mut extended = vec![
            NonNanFloat::new(f64::INFINITY),
            NonNanFloat::new(-42.0),
            NonNanFloat::new(42.0),
            NonNanFloat::new(f64::NEG_INFINITY),
            NonNanFloat::new(0.0),
        ];
        extended.sort();
        assert_eq!(extended.first().unwrap().inner(), f64::NEG_INFINITY);
        assert_eq!(extended.last().unwrap().inner(), f64::INFINITY);
        assert!(extended.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_hash() {
        let mut hash_zero = DefaultHasher::new();
        NonNanFloat::new(0.0).hash(&mut hash_zero);

        let mut hash_neg_zero = DefaultHasher::new();
        NonNanFloat::new(-0.0).hash(&mut hash_neg_zero);

        assert_eq!(hash_zero.finish(), hash_neg_zero.finish());

        let mut hash_pos = DefaultHasher::new();
        NonNanFloat::new(1.234).hash(&mut hash_pos);
        let mut hash_pos_again = DefaultHasher::new();
        NonNanFloat::new(1.234).hash(&mut hash_pos_again);
        assert_eq!(hash_pos.finish(), hash_pos_again.finish());

        let mut set = HashSet::new();
        set.insert(NonNanFloat::new(0.0));
        set.insert(NonNanFloat::new(-0.0));
        assert_eq!(set.len(), 1);

        set.insert(NonNanFloat::new(1.5));
        set.insert(NonNanFloat::new(-1.5));
        assert!(set.contains(&NonNanFloat::new(1.5)));
        assert!(set.contains(&NonNanFloat::new(-1.5)));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_random_op() {
        const T: usize = 1000000;

        let mut rng = StdRng::seed_from_u64(30);

        for _ in 0..T {
            let lhs_raw: f64 = rng.gen();
            let rhs_raw: f64 = rng.gen();

            let lhs = NonNanFloat::new(lhs_raw);
            let rhs = NonNanFloat::new(rhs_raw);

            // add
            {
                let expected = lhs_raw + rhs_raw;
                if !expected.is_nan() {
                    assert_eq!((lhs + rhs).inner(), expected);
                }
            }

            // sub
            {
                let expected = lhs_raw - rhs_raw;
                if !expected.is_nan() {
                    assert_eq!((lhs - rhs).inner(), expected);
                }
            }

            // mul
            {
                let expected = lhs_raw * rhs_raw;
                if !expected.is_nan() {
                    assert_eq!((lhs * rhs).inner(), expected);
                }
            }

            // div
            {
                let expected = lhs_raw / rhs_raw;
                if !expected.is_nan() {
                    assert_eq!((lhs / rhs).inner(), expected);
                }
            }

            // rem
            {
                let expected = lhs_raw % rhs_raw;
                if !expected.is_nan() {
                    assert_eq!((lhs % rhs).inner(), expected);
                }
            }

            // ord
            {
                if let Some(expected) = lhs_raw.partial_cmp(&rhs_raw) {
                    assert_eq!(lhs.cmp(&rhs), expected);
                }
            }
        }
    }

    #[test]
    fn test_random_ord() {
        const T: usize = 100;
        const N: usize = 100000;

        let mut rng = StdRng::seed_from_u64(31);

        for _ in 0..T {
            let mut v_raw = (0..N).map(|_| rng.gen()).collect::<Vec<f64>>();
            let mut v = v_raw
                .iter()
                .map(|&e| NonNanFloat::new(e))
                .collect::<Vec<NonNanFloat>>();

            // min
            {
                let expected = v_raw
                    .iter()
                    .min_by(|l, r| l.partial_cmp(r).unwrap())
                    .copied();
                let res = v.iter().min().map(|x| x.inner());
                assert_eq!(res, expected);
            }

            // max
            {
                let expected = v_raw
                    .iter()
                    .max_by(|l, r| l.partial_cmp(r).unwrap())
                    .copied();
                let res = v.iter().max().map(|x| x.inner());
                assert_eq!(res, expected);
            }

            // sort
            {
                v_raw.sort_by(|l, r| l.partial_cmp(r).unwrap());
                v.sort();
                assert_eq!(v.iter().map(|e| e.inner()).collect::<Vec<f64>>(), v_raw);
            }
        }
    }
}
