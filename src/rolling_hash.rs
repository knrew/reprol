use std::ops::{Range, RangeBounds};

use crate::{math::modint::ModInt, range::to_open_range};

pub struct RollingHash<const P: u64> {
    hash: Vec<ModInt<P>>,
    pow: Vec<ModInt<P>>,
}

impl<const P: u64> RollingHash<P> {
    pub fn new(s: &[u8], base: u64) -> Self {
        assert!(base < P);
        let n = s.len();
        let base = base.into();
        let mut hash = vec![ModInt::new(0); n + 1];
        let mut pow = vec![ModInt::new(1); n + 1];
        for i in 0..n {
            hash[i + 1] = hash[i] * base + s[i].into();
            pow[i + 1] = pow[i] * base.into();
        }
        Self { hash, pow }
    }

    pub fn get(&self, range: impl RangeBounds<usize>) -> u64 {
        let Range { start: l, end: r } = to_open_range(range, self.hash.len() - 1);
        assert!(l <= r);
        (self.hash[r] - self.hash[l] * self.pow[r - l]).value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        use rand::{thread_rng, Rng};

        let testcases = vec![
            "abcabc",
            "aabcaabxaaaz",
            "aaaaaaa",
            "abcababcababcab",
            "abacabadabacaba",
            "abracadabra",
            "abcabcabcabc",
            "abcdeedcba",
            "abababababab",
            "abcdefg",
            "xyzxyzxyzxyzxyz",
            "banana",
            "jxqweorvzmxnalskdjqpwoeiruty",
            "ghfjdkalsuznvmbqowieury",
            "thequickbrownfoxjumpsoveralazydog",
        ];

        const MOD1: u64 = 1000000007;
        const MOD2: u64 = 2147483647;

        let mut rng = thread_rng();

        for s in testcases {
            let rh1 = RollingHash::<MOD1>::new(s.as_bytes(), rng.gen_range(0..MOD1));
            let rh2 = RollingHash::<MOD2>::new(s.as_bytes(), rng.gen_range(0..MOD2));
            for i in 0..s.len() {
                for j in i..s.len() {
                    for k in 0..s.len() {
                        for l in k..s.len() {
                            assert_eq!(rh1.get(i..j) == rh1.get(k..l), s[i..j] == s[k..l]);
                            assert_eq!(rh2.get(i..j) == rh2.get(k..l), s[i..j] == s[k..l]);
                        }
                    }
                }
            }
        }
    }
}
