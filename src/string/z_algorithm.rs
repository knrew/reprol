//! Z algorithm
//!
//! 文字列`s`に対して，配列`z`(長さ`|S|`)を構築する．
//! `z[i]`: `s`と`s[i..n)`の最長共通接頭辞(LCP)の長さ．
//!
//! # 使用例
//! ```
//! use reprol::string::z_algorithm::ZAlgorithm;
//! let z = ZAlgorithm::new(b"abacaba");
//! assert_eq!(z[0], 7);
//! assert_eq!(z[1], 0);
//! assert_eq!(z[4], 3);
//! ```

use std::{fmt::Debug, ops::Index};

#[derive(Clone)]
pub struct ZAlgorithm {
    z: Vec<usize>,
}

impl ZAlgorithm {
    /// 文字列`s`に対して，配列`z`(長さ`|S|`)を構築する．
    /// `z[i]`: `s`と`s[i..n)`の最長共通接頭辞(LCP)の長さ．
    pub fn new<T: PartialEq>(s: &[T]) -> Self {
        if s.is_empty() {
            return Self { z: vec![] };
        }

        let n = s.len();

        let mut z = vec![0; n];
        z[0] = s.len();

        let mut i = 1;
        let mut j = 0;
        while i < n {
            while i + j < n && s[j] == s[i + j] {
                j += 1;
            }
            z[i] = j;
            if j == 0 {
                i += 1;
                continue;
            }
            let mut k = 1;
            while i + k < n && k + z[k] < j {
                z[i + k] = z[k];
                k += 1;
            }
            i += k;
            j -= k;
        }

        Self { z }
    }

    pub fn get(&self, index: usize) -> Option<&usize> {
        self.z.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.z.iter()
    }
}

impl Index<usize> for ZAlgorithm {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.z[index]
    }
}

impl IntoIterator for ZAlgorithm {
    type IntoIter = std::vec::IntoIter<usize>;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        self.z.into_iter()
    }
}

impl<'a> IntoIterator for &'a ZAlgorithm {
    type IntoIter = std::slice::Iter<'a, usize>;
    type Item = &'a usize;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Debug for ZAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.z.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_algorithm() {
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
        ];

        for s in testcases {
            let z = ZAlgorithm::new(s.as_bytes());
            let n = s.len();
            for i in 0..n {
                let l = z[i];
                assert_eq!(&s[0..l], &s[i..i + l]);
                if i + l < s.len() {
                    assert!(s[l..l + 1] != s[i + l..i + l + 1]);
                }
            }
        }
    }
}
