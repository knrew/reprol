use std::ops::{Range, RangeBounds};

use crate::{math::modint::ModInt, range::to_open_range};

pub struct RollingHash<const P: u64> {
    hash: Vec<ModInt<P>>,
    pow: Vec<ModInt<P>>,
}

impl<const P: u64> RollingHash<P> {
    pub fn new(s: &[u8], base: u64) -> Self {
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
        let testcases = vec![
            "abcabc",
            "xJ7fQm2ZaL",
            "pKd8sYwBvX",
            "ZqM3Lt9NcR",
            "VwP6YmBdT4",
            "gXJ2KsN7Qm",
            "aX7JqM2Lt9NcRVwP6YmB",
            "ZqM3Lt9NcRVwP6YmBdT4",
            "pKd8sYwBvXZqM3Lt9NcR",
            "gXJ2KsN7QmaX7JqM2Lt9N",
            "VwP6YmBdT4gXJ2KsN7QmL",
        ];

        const MOD: u64 = 1000000007;
        let base = 100;

        for s in testcases {
            let rh = RollingHash::<MOD>::new(&s.as_bytes(), base);
            for i in 0..s.len() {
                for j in i..s.len() {
                    for k in 0..s.len() {
                        for l in k..s.len() {
                            assert_eq!(rh.get(i..j) == rh.get(k..l), s[i..j] == s[k..l]);
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_rolling_hash_long() {
        debug_assert!(false, "run with release!");

        let testcases = [
            "aX7JqM2Lt9NcRVwP6YmBgXJ2KsN7QmaX7JqM2Lt9NcRVwP6YmBdT4pKd8sYwBvXZqM3Lt9NcRVwP6YmBdT4gXJ2KsN7QmaX7JqM2Lt9NcRVwP6YmBdT4pKd8sYwBvXZqM3Lt9NcRVwP6YmBdT4gXJ2KsN7QmaX7JqM2Lt9NcRVwP6YmBdT4",
            "x1XYQhseITkwqbkzTnygM872WNAbUqAmQ2iRGT8uuzJYwm64XnlPtglfknKhgrbpiB7kaobpts2BXNrPZtoJFgmiW0arFxHm1nhrAdJ2CJwyVFaTsFCh93ijVCJmMGkn77ZmQ4ynd759mET3Q8Rp4UQrpgwCMSeXkK4dQnbOTMbNM8pCLT8vsJSFNfPMT1himdztczjz",
            "GTEgHfq7X7R1hC5PvGHwIibQlYKxI40Dnb3vCyX5nVFhwdwM38UZHSEa2dQGSSOg0sKNfQ439HbxNavMNzaiM4EawfrFLUfS9OBvcMfTRgU5EgmMYhbqGhXRJBO2eSgalgYhlgbrJba91tcoPpvpRXJddyP2XXxGLMrNj6roGbJ5frl42PPa6YJEO6lK0SnqN7yrq26N",
            "7uvlQUYfl6DjNhGKM4s3S1xFNY37gY05RYYq2ZAP4hl07MwHQSxBcqgXeCZ3Frz4QWG0WmkvgOq76gu8g408IUGL4ay9nvqSeAqLzPExVrqMQeBu8ZB7vIEdn6tplyYLXt0eg1ozaPR1WCYqJguy688JUNlIaByZTOvQVHPa8LWjb0CUrv37qLlwH1f9rP9ZPu5cAY", 
    ];

        const MOD: u64 = 1000000007;
        let base = 100;

        for s in testcases {
            let rh = RollingHash::<MOD>::new(&s.as_bytes(), base);
            for i in 0..s.len() {
                for j in i..s.len() {
                    for k in 0..s.len() {
                        for l in k..s.len() {
                            assert_eq!(rh.get(i..j) == rh.get(k..l), s[i..j] == s[k..l]);
                        }
                    }
                }
            }
        }
    }
}
