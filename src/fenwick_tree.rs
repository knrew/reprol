use std::ops::{Range, RangeBounds};

use crate::{group::Group, monoid::Monoid, range::to_open_range};

/// 要素の1点更新と区間積(和)の取得が行えるデータ構造
pub struct FenwickTree<O: Monoid> {
    len: usize,
    nodes: Vec<O::Value>,
    op: O,
}

impl<O> FenwickTree<O>
where
    O: Monoid,
{
    pub fn new(n: usize) -> Self
    where
        O: Default,
        O::Value: Clone,
    {
        Self::with_op(n, O::default())
    }

    /// 演算を引数で指定
    pub fn with_op(n: usize, op: O) -> Self
    where
        O::Value: Clone,
    {
        Self {
            len: n,
            nodes: vec![op.identity(); n],
            op,
        }
    }

    /// v[index]<-v[index]+value
    pub fn add(&mut self, mut index: usize, value: O::Value) {
        assert!(index < self.len);
        index += 1;
        while index <= self.len {
            self.nodes[index - 1] = self.op.op(&self.nodes[index - 1], &value);
            index += index & index.wrapping_neg();
        }
    }

    // [0, r)の累積
    pub fn get(&self, mut r: usize) -> O::Value {
        let mut res = self.op.identity();
        while r > 0 {
            res = self.op.op(&res, &self.nodes[r - 1]);
            r -= r & r.wrapping_neg();
        }
        res
    }

    /// [l, r)の区間積(和)を取得する
    pub fn product(&self, range: impl RangeBounds<usize>) -> O::Value
    where
        O: Group,
    {
        let Range { start: l, end: r } = to_open_range(range, self.len);
        assert!(l <= r);
        let cl = self.get(l);
        let cr = self.get(r);
        self.op.op(&cr, &self.op.inv(&cl))
    }
}

#[cfg(test)]
mod tests {
    use crate::ops::op_add::OpAdd;

    use super::FenwickTree;

    #[test]
    fn test_fenwick_tree() {
        let mut ft = FenwickTree::<OpAdd<i64>>::new(10);
        ft.add(0, 5);
        ft.add(2, 10);
        ft.add(6, 20);
        assert_eq!(ft.product(..1), 5);
        assert_eq!(ft.product(..3), 15);
        assert_eq!(ft.product(..7), 35);
        assert_eq!(ft.product(..), 35);
        assert_eq!(ft.product(0..3), 15);
        assert_eq!(ft.product(3..=6), 20);
        ft.add(9, 10);
        assert_eq!(ft.product(0..10), 45);
    }
}
