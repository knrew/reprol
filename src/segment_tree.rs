use std::ops::{Bound, Range, RangeBounds};

use crate::monoid::Monoid;

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
    pub fn new(len: usize, monoid: M) -> Self {
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

    pub fn set(&mut self, index: usize, value: &M::Value) {
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

    pub fn product<R: RangeBounds<usize>>(&self, range: R) -> M::Value {
        let Range { start: l, end: r } = to_open(range, self.len);

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

/// ranageを区間[l, r)に変換する
fn to_open<R: RangeBounds<usize>>(range: R, n: usize) -> Range<usize> {
    let l = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };

    let r = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };

    l..r
}

impl<M> SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    pub fn with_len(len: usize) -> Self {
        Self::new(len, M::default())
    }
}

impl<M> From<&[M::Value]> for SegmentTree<M>
where
    M: Monoid + Default,
    M::Value: Clone,
{
    fn from(v: &[M::Value]) -> Self {
        let mut res = Self::new(v.len(), M::default());

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
