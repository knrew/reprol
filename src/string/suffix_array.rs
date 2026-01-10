//! Suffix Array
//!
//! Suffix Array(接尾辞配列)を構築する．
//!
//! NOTE: 遅いかも

use std::{cmp::Ordering, fmt::Debug, mem::swap, ops::Index};

#[repr(transparent)]
#[derive(Clone)]
pub struct SuffixArray {
    sa: Vec<usize>,
}

impl SuffixArray {
    pub fn new(s: &[u8]) -> Self {
        let len = s.len();

        let mut sa = (0..=len).collect::<Vec<_>>();

        let mut rank = s
            .iter()
            .map(|&si| si as i32)
            .chain(Some(-1))
            .collect::<Vec<_>>();

        let mut rank_swp = vec![0; len + 1];

        let mut k = 1;
        while k <= len {
            sa.sort_unstable_by(|&i, &j| cmp_by_rank(&rank, i, j, k));

            rank_swp[sa[0]] = 0;
            for i in 1..=len {
                rank_swp[sa[i]] = rank_swp[sa[i - 1]]
                    + if cmp_by_rank(&rank, sa[i - 1], sa[i], k) == Ordering::Less {
                        1
                    } else {
                        0
                    };
            }

            swap(&mut rank, &mut rank_swp);

            k *= 2;
        }

        Self { sa }
    }

    pub fn get(&self, index: usize) -> Option<&usize> {
        self.sa.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.sa.iter()
    }
}

impl Index<usize> for SuffixArray {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.sa[index]
    }
}

impl IntoIterator for SuffixArray {
    type IntoIter = std::vec::IntoIter<usize>;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        self.sa.into_iter()
    }
}

impl<'a> IntoIterator for &'a SuffixArray {
    type IntoIter = std::slice::Iter<'a, usize>;
    type Item = &'a usize;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Debug for SuffixArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.sa.iter()).finish()
    }
}

fn cmp_by_rank(rank: &[i32], i: usize, j: usize, k: usize) -> Ordering {
    if rank[i] == rank[j] {
        let ri = rank.get(i + k).unwrap_or(&-1);
        let rj = rank.get(j + k).unwrap_or(&-1);
        ri.cmp(rj)
    } else {
        rank[i].cmp(&rank[j])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sa() {
        let s = b"banana";
        let sa = SuffixArray::new(s);
        let expected = vec![6, 5, 3, 1, 0, 4, 2];
        assert!(sa.iter().eq(&expected));

        let s = b"mississippi";
        let sa = SuffixArray::new(s);
        let expected = vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
        assert!(sa.iter().eq(&expected));
    }
}
