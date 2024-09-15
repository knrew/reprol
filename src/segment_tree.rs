use std::ops::Range;

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

    fn update(&mut self, x: usize, value: M::Value) {
        let mut x = x + self.offset;
        self.nodes[x] = value;

        x >>= 1;
        while x > 0 {
            self.nodes[x] = self.monoid.op(&self.nodes[2 * x], &self.nodes[2 * x + 1]);
            x >>= 1;
        }
    }

    pub fn set(&mut self, x: usize, value: M::Value) {
        self.update(x, value)
    }

    pub fn get(&self, x: usize) -> &M::Value {
        &self.nodes[x + self.offset]
    }

    /// 区間[l..r)の演算結果を求める
    pub fn product(&self, range: Range<usize>) -> M::Value {
        let mut vl = self.monoid.identity();
        let mut vr = self.monoid.identity();

        let mut left = range.start + self.offset;
        let mut right = range.end + self.offset;
        while left < right {
            if left & 1 != 0 {
                vl = self.monoid.op(&vl, &self.nodes[left]);
                left += 1;
            }

            if right & 1 != 0 {
                right -= 1;
                vr = self.monoid.op(&self.nodes[right], &vr);
            }

            left >>= 1;
            right >>= 1;
        }

        self.monoid.op(&vl, &vr)
    }

    /// 左端をlで固定したときの最大のrを求める
    pub fn max_right(&self, l: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        let mut r = l + self.offset;
        let mut v = self.monoid.identity();

        loop {
            let p = r.trailing_zeros();
            let rr = r + (1 << p);
            if self.offset + self.len < rr {
                break;
            }
            let vv = self.monoid.op(&v, &self.nodes[r >> p]);
            if !f(&vv) {
                break;
            }
            r = rr;
            v = vv;
        }

        for p in (0..r.trailing_zeros()).rev() {
            let rr = r + (1 << p);
            if self.offset + self.len < rr {
                continue;
            }
            let vv = self.monoid.op(&v, &self.nodes[r >> p]);
            if !f(&vv) {
                continue;
            }
            r = rr;
            v = vv;
        }

        r - self.offset
    }

    /// 右端をrで固定したときの最小のlを求める
    pub fn min_left(&self, r: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        if r == 0 {
            return 0;
        }

        let mut l = r + self.offset;
        let mut v = self.monoid.identity();

        loop {
            let p = (l | self.offset).trailing_zeros();
            let ll = l - (1 << p);
            let vv = self.monoid.op(&v, &self.nodes[ll >> p]);
            if !f(&vv) {
                break;
            }
            l = ll;
            v = vv;
            if l == self.offset {
                return 0;
            }
        }

        for p in (0..(l | self.offset).trailing_zeros()).rev() {
            let ll = l - (1 << p);
            let vv = self.monoid.op(&v, &self.nodes[ll >> p]);
            if !f(&vv) {
                continue;
            }
            l = ll;
            v = vv;
        }

        l - self.offset
    }
}

//  pub struct SegmentTree<M>
//     where
//         M: Monoid,
//     {
//         len: usize,
//         offset: usize,
//         nodes: Vec<M::Value>,
//         monoid: M,
//     }

//     impl<M> SegmentTree<M>
//     where
//         M: Monoid,
//         M::Value: Clone,
//     {
//         pub fn new(len: usize, monoid: M) -> Self {
//             let offset = len.next_power_of_two();
//             Self {
//                 len,
//                 offset,
//                 nodes: vec![monoid.identity(); 2 * offset],
//                 monoid,
//             }
//         }

//         fn update(&mut self, x: usize, value: M::Value) {
//             let mut x = x + self.offset;
//             self.nodes[x] = value;

//             x >>= 1;
//             while x > 0 {
//                 self.nodes[x] = self.monoid.op(&self.nodes[2 * x], &self.nodes[2 * x + 1]);
//                 x >>= 1;
//             }
//         }

//         pub fn set(&mut self, x: usize, value: M::Value) {
//             self.update(x, value)
//         }

//         pub fn get(&self, x: usize) -> &M::Value {
//             &self.nodes[x + self.offset]
//         }

//         /// 区間[l..r)の演算結果を求める
//         pub fn product(&self, range: Range<usize>) -> M::Value {
//             let mut vl = self.monoid.identity();
//             let mut vr = self.monoid.identity();

//             let mut left = range.start + self.offset;
//             let mut right = range.end + self.offset;
//             while left < right {
//                 if left & 1 != 0 {
//                     vl = self.monoid.op(&vl, &self.nodes[left]);
//                     left += 1;
//                 }

//                 if right & 1 != 0 {
//                     right -= 1;
//                     vr = self.monoid.op(&self.nodes[right], &vr);
//                 }

//                 left >>= 1;
//                 right >>= 1;
//             }

//             self.monoid.op(&vl, &vr)
//         }

//         /// 左端をlで固定したときの最大のrを求める
//         pub fn max_right(&self, l: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
//             debug_assert!(f(&self.monoid.identity()));
//             assert!(l <= self.len);
//             if l == self.len {
//                 return self.len;
//             }

//             let mut l = l + self.offset;
//             let mut value = self.monoid.identity();

//             loop {
//                 while l % 2 == 0 {
//                     l >>= 1;
//                 }
//                 if !f(&self.monoid.op(&value, &self.nodes[l])) {
//                     while l < self.offset {
//                         l *= 2;
//                         let tmp = self.monoid.op(&value, &self.nodes[l]);
//                         if f(&tmp) {
//                             value = tmp;
//                             l += 1;
//                         }
//                     }
//                     return l - self.offset;
//                 }
//                 value = self.monoid.op(&value, &self.nodes[l]);
//                 l += 1;
//                 if (l as i64 & -(l as i64)) == l as i64 {
//                     break;
//                 }
//             }

//             self.len
//         }
//     }
// }
