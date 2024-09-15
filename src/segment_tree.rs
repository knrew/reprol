use std::ops::{
    Bound,
    // Index, IndexMut,
    RangeBounds,
};

pub trait Monoid {
    type Value;
    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value;
    fn identity(&self) -> Self::Value;
}

pub struct SegmentTree<M>
where
    M: Monoid,
{
    len: usize,
    offset: usize,
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
        Self {
            len,
            offset,
            nodes: vec![monoid.identity(); 2 * offset],
            monoid,
        }
    }

    /// x番目の要素をvalueにする
    fn update(&mut self, index: usize, value: M::Value) {
        let mut x = index + self.offset;
        self.nodes[x] = value;
        while x > 0 {
            x /= 2;
            self.nodes[x] = self.monoid.op(&self.nodes[2 * x], &self.nodes[2 * x + 1]);
        }
    }

    pub fn set(&mut self, index: usize, value: M::Value) {
        self.update(index, value)
    }

    pub fn get(&self, index: usize) -> &M::Value {
        &self.nodes[index + self.offset]
    }

    /// 区間の総積を計算する
    pub fn product<R: RangeBounds<usize>>(&self, range: R) -> M::Value {
        let mut value_left = self.monoid.identity();
        let mut value_right = self.monoid.identity();

        let mut start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        } + self.offset;

        let mut end = match range.end_bound() {
            Bound::Unbounded => self.len,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        } + self.offset;

        while start < end {
            if start % 2 == 1 {
                value_left = self.monoid.op(&value_left, &self.nodes[start]);
                start += 1;
            }

            if end % 2 == 1 {
                end -= 1;
                value_right = self.monoid.op(&self.nodes[end], &value_right);
            }

            start /= 2;
            end /= 2;
        }

        self.monoid.op(&value_left, &value_right)
    }

    /// 左端をstartで固定したときの最大のrを求める
    pub fn max_right(&self, start: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        let mut end = start + self.offset;
        let mut value = self.monoid.identity();

        loop {
            let p = end.trailing_zeros();
            let end_tmp = end + (1 << p);
            if self.offset + self.len < end_tmp {
                break;
            }
            let value_tmp = self.monoid.op(&value, &self.nodes[end >> p]);
            if !f(&value_tmp) {
                break;
            }
            end = end_tmp;
            value = value_tmp;
        }

        for p in (0..end.trailing_zeros()).rev() {
            let end_tmp = end + (1 << p);
            if self.offset + self.len < end_tmp {
                continue;
            }
            let value_tmp = self.monoid.op(&value, &self.nodes[end >> p]);
            if !f(&value_tmp) {
                continue;
            }
            end = end_tmp;
            value = value_tmp;
        }

        end - self.offset
    }

    /// 右端をrで固定したときの最小のlを求める
    pub fn min_left(&self, end: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        if end == 0 {
            return 0;
        }

        let mut start = end + self.offset;
        let mut value = self.monoid.identity();

        loop {
            let p = (start | self.offset).trailing_zeros();
            let start_tmp = start - (1 << p);
            let value_tmp = self.monoid.op(&value, &self.nodes[start_tmp >> p]);
            if !f(&value_tmp) {
                break;
            }
            start = start_tmp;
            value = value_tmp;
            if start == self.offset {
                return 0;
            }
        }

        for p in (0..(start | self.offset).trailing_zeros()).rev() {
            let start_tmp = start - (1 << p);
            let value_tmp = self.monoid.op(&value, &self.nodes[start_tmp >> p]);
            if !f(&value_tmp) {
                continue;
            }
            start = start_tmp;
            value = value_tmp;
        }

        start - self.offset
    }
}

// impl<M> Index<usize> for SegmentTree<M>
// where
//     M: Monoid,
// {
//     type Output = M::Value;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.nodes[index + self.offset]
//     }
// }

// impl<M> IndexMut<usize> for SegmentTree<M>
// where
//     M: Monoid,
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.nodes[index + self.offset]
//     }
// }

impl<M> From<(&[M::Value], M)> for SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn from((v, m): (&[M::Value], M)) -> Self {
        let mut res = SegmentTree::new(v.len(), m);
        for i in 0..v.len() {
            res.set(i, v[i].clone());
        }
        res
    }
}

impl<M> From<(&Vec<M::Value>, M)> for SegmentTree<M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn from((v, m): (&Vec<M::Value>, M)) -> Self {
        SegmentTree::from((v.as_slice(), m))
    }
}
