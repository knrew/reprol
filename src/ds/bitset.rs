//! BitSet
//!
//! 固定長のビット集合を`u64`配列で管理するデータ構造．
//! 単一ビットの取得・設定・リセットと，辞書順(数値的)比較を提供する．
//!
//! # 使用例
//! ```
//! use reprol::{bitset, ds::bitset::BitSet};
//!
//! let mut bs = bitset!(130);
//! bs.set(5);
//! assert!(bs.get(5));
//!
//! bs.reset(5);
//! assert!(!bs.get(5));
//!
//! let high: BitSet<3> = BitSet::new();
//! assert_eq!(high, BitSet::<3>::ZERO);
//! ```

use std::{cmp::Ordering, fmt::Debug};

/// 固定長のビット集合を`u64`の配列で保持するデータ構造．
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitSet<const WORDS: usize> {
    bit: [u64; WORDS],
}

impl<const WORDS: usize> BitSet<WORDS> {
    /// すべてのビットが0の定数インスタンス．
    pub const ZERO: Self = Self { bit: [0; WORDS] };

    /// すべてのビットが0の`BitSet`を生成する．
    pub const fn new() -> Self {
        Self { bit: [0; WORDS] }
    }

    #[inline]
    const fn index(i: usize) -> (usize, u64) {
        let word = i >> 6;
        let mask = 1u64 << (i & 63);
        (word, mask)
    }

    /// 指定したビットが立っているかを返す．
    ///
    /// # パニック
    /// - `i / 64 >= WORDS` の場合
    pub const fn get(&self, i: usize) -> bool {
        let (w, m) = Self::index(i);
        assert!(w < WORDS);
        (self.bit[w] & m) != 0
    }

    /// 指定したビットを1に更新する．
    ///
    /// # パニック
    /// - `i / 64 >= WORDS` の場合
    pub fn set(&mut self, i: usize) {
        let (w, m) = Self::index(i);
        assert!(w < WORDS);
        self.bit[w] |= m;
    }

    /// 指定したビットを0に更新する．
    ///
    /// # パニック
    /// - `i / 64 >= WORDS` の場合
    pub fn reset(&mut self, i: usize) {
        let (w, m) = Self::index(i);
        assert!(w < WORDS);
        self.bit[w] &= !m;
    }
}

impl<const WORDS: usize> Default for BitSet<WORDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WORDS: usize> Ord for BitSet<WORDS> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bit.iter().rev().cmp(other.bit.iter().rev())
    }
}

impl<const WORDS: usize> PartialOrd for BitSet<WORDS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const WORDS: usize> Debug for BitSet<WORDS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut started = false;

        for &w in self.bit.iter().rev() {
            if !started {
                if w == 0 {
                    continue;
                }
                started = true;
                write!(f, "{:b}", w)?;
            } else {
                write!(f, "{:064b}", w)?;
            }
        }

        if !started {
            write!(f, "0")?;
        }

        Ok(())
    }
}

/// 指定したビット長に対応する`BitSet`を生成する．
#[macro_export]
macro_rules! bitset {
    ($len: expr) => {
        BitSet::<{ (($len) + 63) / 64 }>::new()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::cmp::Ordering;

    #[test]
    fn test_new_default_zero() {
        fn test<const WORDS: usize>() {
            let from_new = BitSet::<WORDS>::new();
            let from_default = BitSet::<WORDS>::default();
            let from_const = BitSet::<WORDS>::ZERO;

            assert_eq!(from_new, from_default);
            assert_eq!(from_new, from_const);
        }
        test::<1>();
        test::<2>();
    }

    #[test]
    fn test_get_set_reset_single() {
        {
            let mut bs = BitSet::<1>::new();

            assert!(!bs.get(0));
            bs.set(0);
            assert!(bs.get(0));

            assert!(!bs.get(63));
            bs.set(63);
            assert!(bs.get(63));
        }

        {
            let mut bs = BitSet::<1>::new();
            bs.set(5);
            bs.set(7);

            bs.reset(5);
            assert!(!bs.get(5));
            assert!(bs.get(7));
        }
    }

    #[test]
    fn test_get_set_reset_multiple_words() {
        let mut bs = BitSet::<2>::new();
        bs.set(70);
        assert!(bs.get(70));

        bs.reset(70);
        assert!(!bs.get(70));

        bs.set(80);
        bs.set(3);
        assert!(bs.get(80));
        assert!(bs.get(3));
    }

    #[test]
    #[should_panic]
    fn get_panics_when_index_out_of_bounds() {
        let bs = BitSet::<1>::new();
        let _ = bs.get(64);
    }

    #[test]
    #[should_panic]
    fn set_panics_when_index_out_of_bounds() {
        let mut bs = BitSet::<1>::new();
        bs.set(64);
    }

    #[test]
    #[should_panic]
    fn reset_panics_when_index_out_of_bounds() {
        let mut bs = BitSet::<1>::new();
        bs.reset(64);
    }

    #[test]
    fn test_ordering() {
        {
            let mut smaller = BitSet::<2>::new();
            let mut larger = BitSet::<2>::new();

            smaller.set(1);
            larger.set(70);

            assert_eq!(Ordering::Less, smaller.cmp(&larger));
            assert!(smaller < larger);

            let equal = larger.clone();
            assert_eq!(Ordering::Equal, larger.cmp(&equal));
            assert_eq!(larger, equal);
        }

        {
            let mut lhs = BitSet::<2>::new();
            let mut rhs = BitSet::<2>::new();

            lhs.set(5);
            rhs.set(6);
            assert_eq!(Ordering::Less, lhs.cmp(&rhs));
            assert!(lhs < rhs);

            rhs.reset(6);
            rhs.set(5);
            assert_eq!(Ordering::Equal, lhs.cmp(&rhs));
            assert_eq!(lhs, rhs);
        }

        {
            let mut low_heavy = BitSet::<2>::new();
            for i in 0..64 {
                low_heavy.set(i);
            }

            let mut high_bit = BitSet::<2>::new();
            high_bit.set(64);

            assert_eq!(Ordering::Less, low_heavy.cmp(&high_bit));
            assert!(low_heavy < high_bit);
        }
    }

    #[test]
    fn test_clone() {
        let mut original = BitSet::<2>::new();
        original.set(5);
        original.set(100);

        let mut clone = original.clone();
        assert_eq!(original, clone);

        clone.reset(5);
        assert_ne!(original, clone);
    }

    #[test]
    fn test_debug() {
        let zero = BitSet::<3>::new();
        assert_eq!("0", format!("{zero:?}"));

        let mut bs = BitSet::<2>::new();
        bs.set(70);
        bs.set(0);

        let high_word = bs.bit[1];
        let low_word = bs.bit[0];
        let expected = format!("{high_word:b}{low_word:064b}");

        assert_eq!(expected, format!("{bs:?}"));
    }

    #[test]
    fn test_macro() {
        let mut bs = bitset!(130);
        assert_eq!(3, bs.bit.len());

        bs.set(129);
        assert!(bs.get(129));
    }

    #[test]
    fn test_random() {
        fn test<const WORDS: usize>(rng: &mut impl Rng) {
            assert!(WORDS > 0);

            const T: usize = 100;
            const Q: usize = 10000;

            let n: usize = WORDS * 64;

            for _ in 0..T {
                let mut bs = BitSet::<WORDS>::new();
                let mut naive = vec![false; n];

                for _ in 0..Q {
                    let i = rng.gen_range(0..n);
                    if rng.gen_bool(0.5) {
                        bs.set(i);
                        naive[i] = true;
                    } else {
                        bs.reset(i);
                        naive[i] = false;
                    }
                    assert_eq!(bs.get(i), naive[i]);

                    let i = rng.gen_range(0..n);
                    assert_eq!(bs.get(i), naive[i]);
                }

                for (i, &expected) in naive.iter().enumerate() {
                    assert_eq!(bs.get(i), expected);
                }
            }
        }

        let mut rng = StdRng::seed_from_u64(30);
        test::<1>(&mut rng);
        test::<2>(&mut rng);
        test::<10>(&mut rng);
    }
}
