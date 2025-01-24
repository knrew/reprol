use std::ops::{Range, RangeBounds};

use crate::{group::Group, ops::op_add::OpAdd, range::to_open_range};

#[derive(Clone)]
pub struct CumulativeProduct<G: Group> {
    len: usize,
    data: Vec<G::Value>,
    group: G,
}

impl<G> CumulativeProduct<G>
where
    G: Group,
    G::Value: Clone,
{
    pub fn new(v: Vec<G::Value>, group: G) -> Self {
        assert!(!v.is_empty());
        Self::new_by(v.len(), group, |i| v[i].clone())
    }

    pub fn new_by(len: usize, group: G, mut f: impl FnMut(usize) -> G::Value) -> Self {
        let mut cum = vec![group.identity(); len + 1];
        for i in 0..len {
            cum[i + 1] = group.op(&cum[i], &f(i));
        }
        Self {
            len,
            data: cum,
            group,
        }
    }

    /// 区間積を計算する
    /// a[l]+ ... + a[r-1]
    pub fn product(&self, range: impl RangeBounds<usize>) -> G::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);
        assert!(l <= r);
        self.group.op(&self.data[r], &self.group.inv(&self.data[l]))
    }
}

impl<G> From<Vec<G::Value>> for CumulativeProduct<G>
where
    G: Group + Default,
    G::Value: Clone,
{
    fn from(v: Vec<G::Value>) -> Self {
        CumulativeProduct::new(v, G::default())
    }
}

impl<G> From<&Vec<G::Value>> for CumulativeProduct<G>
where
    G: Group + Default,
    G::Value: Clone,
{
    fn from(v: &Vec<G::Value>) -> Self {
        CumulativeProduct::new(v.clone(), G::default())
    }
}

impl<G> From<&[G::Value]> for CumulativeProduct<G>
where
    G: Group + Default,
    G::Value: Clone,
{
    fn from(v: &[G::Value]) -> Self {
        CumulativeProduct::new(v.to_vec(), G::default())
    }
}

impl<G> FromIterator<G::Value> for CumulativeProduct<G>
where
    G: Group + Default,
    G::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::Value>>(iter: T) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

pub type CumulativeSum<T> = CumulativeProduct<OpAdd<T>>;

#[cfg(test)]
mod tests {
    use crate::{cumulative_product::CumulativeSum, ops::op_add::OpAdd};

    #[test]
    fn test_cumulative_sum_1d() {
        let v = vec![1, 2, 3, 4, 5];
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];
        let cum = CumulativeSum::<i64>::from(v);
        assert_eq!(cum.product(..), 15);
        for ((l, r), expected) in test_cases {
            assert_eq!(cum.product(l..r), expected);
        }

        let cum = CumulativeSum::new_by(5, OpAdd::<i64>::default(), |i| i as i64 + 1);
        let test_cases = vec![
            ((0, 5), 15),
            ((0, 1), 1),
            ((1, 3), 5),
            ((3, 5), 9),
            ((2, 4), 7),
            ((2, 2), 0),
            ((4, 5), 5),
            ((0, 4), 10),
        ];

        for ((l, r), expected) in test_cases {
            assert_eq!(cum.product(l..r), expected);
        }
    }
}
