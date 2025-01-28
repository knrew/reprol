use std::ops::{Range, RangeBounds};

use crate::{monoid::Monoid, range::to_open_range};

pub struct SegmentTree<M>
where
    M: Monoid,
{
    len: usize,
    offset: usize,
    log: u32,
    nodes: Vec<M::Value>,
    monoid: M,
}

impl<M> SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    pub fn new(v: Vec<M::Value>) -> Self
    where
        M: Default,
    {
        Self::from(v)
    }

    pub fn with_len(len: usize) -> Self
    where
        M: Default,
    {
        Self::with_op(len, M::default())
    }

    /// 演算(モノイド)を引数で指定
    pub fn with_op(len: usize, monoid: M) -> Self {
        let offset = len.next_power_of_two();
        let log = offset.trailing_zeros();
        let nodes = vec![monoid.identity(); 2 * offset];
        Self {
            len,
            offset,
            log,
            nodes,
            monoid,
        }
    }

    fn update(&mut self, k: usize) {
        self.nodes[k] = self.monoid.op(&self.nodes[k * 2], &self.nodes[k * 2 + 1]);
    }

    pub fn set(&mut self, index: usize, value: M::Value) {
        assert!(index < self.len);
        let index = index + self.offset;
        self.nodes[index] = value.clone();
        for i in 1..=self.log {
            self.update(index >> i)
        }
    }

    pub fn get(&self, index: usize) -> &M::Value {
        assert!(index < self.len);
        &self.nodes[index + self.offset]
    }

    /// 区間積を取得する
    pub fn product(&self, range: impl RangeBounds<usize>) -> M::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);

        assert!(r <= self.len);
        assert!(l <= r);

        let (mut l, mut r) = (l + self.offset, r + self.offset);

        let mut vl = self.monoid.identity();
        let mut vr = self.monoid.identity();

        while l < r {
            if l % 2 == 1 {
                vl = self.monoid.op(&vl, &self.nodes[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                vr = self.monoid.op(&self.nodes[r], &vr);
            }
            l /= 2;
            r /= 2;
        }

        self.monoid.op(&vl, &vr)
    }

    pub fn max_right(&self, l: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(l <= self.len);

        #[cfg(debug_assertions)]
        assert!(f(&self.monoid.identity()));

        if l == self.len {
            return self.len;
        }

        let mut l = l + self.offset;
        let mut sum = self.monoid.identity();

        loop {
            while l % 2 == 0 {
                l /= 2;
            }

            if !f(&self.monoid.op(&sum, &self.nodes[l])) {
                while l < self.offset {
                    l *= 2;

                    let tmp = self.monoid.op(&sum, &self.nodes[l]);
                    if f(&tmp) {
                        sum = tmp;
                        l += 1;
                    }
                }

                return l - self.offset;
            }

            sum = self.monoid.op(&sum, &self.nodes[l]);
            l += 1;

            if l.is_power_of_two() {
                break;
            }
        }

        return self.len;
    }

    pub fn min_left(&self, r: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(r <= self.len);

        #[cfg(debug_assertions)]
        assert!(f(&self.monoid.identity()));

        if r == 0 {
            return 0;
        }
        let mut r = r + self.offset;
        let mut sum = self.monoid.identity();

        loop {
            r -= 1;

            while r > 1 && r % 2 == 1 {
                r /= 2;
            }

            if !f(&self.monoid.op(&self.nodes[r], &sum)) {
                while r < self.offset {
                    r = 2 * r + 1;
                    let tmp = self.monoid.op(&self.nodes[r], &sum);
                    if f(&tmp) {
                        sum = tmp;
                        r -= 1;
                    }
                }

                return r + 1 - self.offset;
            }

            sum = self.monoid.op(&self.nodes[r], &sum);

            if r.is_power_of_two() {
                break;
            }
        }

        0
    }
}

impl<M> From<&[M::Value]> for SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &[M::Value]) -> Self {
        let mut res = Self::with_op(v.len(), M::default());

        for i in 0..res.len {
            res.nodes[i + res.offset] = v[i].clone();
        }

        for i in (1..res.offset).rev() {
            res.update(i)
        }

        res
    }
}

impl<M> From<&Vec<M::Value>> for SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &Vec<M::Value>) -> Self {
        Self::from(v.as_slice())
    }
}

impl<M> From<Vec<M::Value>> for SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: Vec<M::Value>) -> Self {
        Self::from(v.as_slice())
    }
}

impl<M> FromIterator<M::Value> for SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

// TODO: min_left, max_rightのテストを書く
#[cfg(test)]
mod tests {
    mod tests_add {
        use crate::{monoid::Monoid, segment_tree::SegmentTree};

        #[derive(Default)]
        struct Op;

        impl Monoid for Op {
            type Value = i64;

            fn identity(&self) -> Self::Value {
                0
            }

            fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
                x + y
            }
        }

        #[test]
        fn test_add() {
            let v = vec![1, 3, 5, 7, 9, 11];
            let mut seg = SegmentTree::<Op>::from(&v);
            assert_eq!(seg.product(0..3), 9);
            assert_eq!(seg.product(1..=4), 24);
            assert_eq!(seg.product(..), 36);
            seg.set(2, 6);
            assert_eq!(seg.product(0..3), 10);
        }
    }

    mod tests_min {
        use crate::{monoid::Monoid, segment_tree::SegmentTree};

        #[derive(Default)]
        struct Op;

        impl Monoid for Op {
            type Value = i64;

            fn identity(&self) -> Self::Value {
                i64::MAX
            }

            fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
                *x.min(y)
            }
        }

        #[test]
        fn test_min() {
            let v = vec![5, 2, 6, 3, 7, 1];
            let mut seg = SegmentTree::<Op>::from(&v);
            assert_eq!(seg.product(0..4), 2);
            assert_eq!(seg.product(2..=5), 1);
            assert_eq!(seg.product(..), 1);
            assert_eq!(seg.product(..=4), 2);
            seg.set(3, 0);
            assert_eq!(seg.product(0..4), 0);
        }
    }
}
