//! Radix
//!
//! 非負整数の 10 進法と任意の n 進法との相互変換を行う機能を提供する．
//!
//! - [`RadixDecomposer`] : 10進数の非負整数を指定した基数で分解して桁ごとに分解する．
//! - [`RadixComposer`] : 指定された基数で分解された配列から10進数の値を再構成する．

pub trait RadixDecomposer {
    /// 非負整数を基数`base`で桁に分解し，上位桁から順に並べた配列を返す．
    ///
    /// # 例
    /// ```
    /// use crate::reprol::math::radix::RadixDecomposer;
    ///
    /// let decomposed = 123u64.radix_decompose(10);
    /// assert_eq!(decomposed, vec![1, 2, 3]);
    ///
    /// let decomposed = 5u64.radix_decompose(2);
    /// assert_eq!(decomposed, vec![1, 0, 1]);
    /// ```
    fn radix_decompose(self, base: u32) -> Vec<u32>;
}

impl RadixDecomposer for u64 {
    fn radix_decompose(self, base: u32) -> Vec<u32> {
        assert!(base >= 2);
        let mut n = self;

        if n == 0 {
            return vec![0];
        }

        let base = base as u64;
        let mut res = vec![];
        while n > 0 {
            res.push((n % base) as u32);
            n /= base;
        }
        res.reverse();
        res
    }
}

pub trait RadixComposer {
    /// 基数`base`で桁ごとに分解された配列を10進数として再構成する．
    /// 上位桁から順に並んでいる必要がある．
    ///
    /// # 例
    /// ```
    /// use crate::reprol::math::radix::RadixComposer;
    ///
    /// let value = [1, 2, 3].radix_compose(10);
    /// assert_eq!(value, 123);
    ///
    /// let value = vec![1, 0, 1].radix_compose(2);
    /// assert_eq!(value, 5);
    /// ```
    fn radix_compose(&self, base: u32) -> u64;

    fn checked_radix_compose(&self, base: u32) -> Option<u64>;
}

impl RadixComposer for [u32] {
    fn radix_compose(&self, base: u32) -> u64 {
        assert!(base >= 2);
        let base = base as u64;
        let mut res = 0;
        for &e in self {
            res *= base;
            res += e as u64;
        }
        res
    }

    fn checked_radix_compose(&self, base: u32) -> Option<u64> {
        assert!(base >= 2);
        let base = base as u64;
        let mut res = 0u64;
        for &e in self {
            res = res.checked_mul(base)?.checked_add(e as u64)?;
        }
        Some(res)
    }
}

impl RadixComposer for &str {
    fn radix_compose(&self, base: u32) -> u64 {
        u64::from_str_radix(self, base).unwrap()
    }

    fn checked_radix_compose(&self, base: u32) -> Option<u64> {
        u64::from_str_radix(self, base).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_decompose() {
        let x = 697;
        assert_eq!(x.radix_decompose(10), vec![6, 9, 7]);
        assert_eq!(x.radix_decompose(2), vec![1, 0, 1, 0, 1, 1, 1, 0, 0, 1]);
        assert_eq!(x.radix_decompose(8), vec![1, 2, 7, 1]);
        assert_eq!(x.radix_decompose(16), vec![2, 11, 9]);
        assert_eq!(x.radix_decompose(5), vec![1, 0, 2, 4, 2]);
        let x = 1234;
        assert_eq!(x.radix_decompose(10), vec![1, 2, 3, 4]);
        assert_eq!(x.radix_decompose(2), vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0]);
        assert_eq!(x.radix_decompose(8), vec![2, 3, 2, 2]);
        assert_eq!(x.radix_decompose(16), vec![4, 13, 2]);
        assert_eq!(x.radix_decompose(5), vec![1, 4, 4, 1, 4]);
    }

    #[test]
    fn test_radix_compose() {
        assert_eq!(vec![6, 9, 7].radix_compose(10), 697);
        assert_eq!(vec![1, 0, 1, 0, 1, 1, 1, 0, 0, 1].radix_compose(2), 697);
        assert_eq!(vec![1, 2, 7, 1].radix_compose(8), 697);
        assert_eq!(vec![2, 11, 9].radix_compose(16), 697);
        assert_eq!(vec![1, 0, 2, 4, 2].radix_compose(5), 697);
        assert_eq!(vec![1, 2, 3, 4].radix_compose(10), 1234);
        assert_eq!(vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0].radix_compose(2), 1234);
        assert_eq!(vec![2, 3, 2, 2].radix_compose(8), 1234);
        assert_eq!(vec![4, 13, 2].radix_compose(16), 1234);
        assert_eq!(vec![1, 4, 4, 1, 4].radix_compose(5), 1234);

        assert_eq!("1271".radix_compose(8), 697);
        assert_eq!("4d2".radix_compose(16), 1234);

        assert_eq!(String::from("1271").as_str().radix_compose(8), 697);
        assert_eq!(String::from("4d2").as_str().radix_compose(16), 1234);
    }
}
