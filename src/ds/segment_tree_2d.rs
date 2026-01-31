use std::ops::{Deref, DerefMut, Index, Range, RangeBounds};

use crate::{ops::monoid::Monoid, utils::range::to_half_open_index_range};

pub struct SegmentTree2d<O: Monoid> {
    len_rows: usize,
    len_cols: usize,
    offset_row: usize,
    offset_col: usize,
    nodes: Vec<O::Value>,
    nodes_len_cols: usize,
    op: O,
}

impl<O: Monoid> SegmentTree2d<O> {
    pub fn new(h: usize, w: usize) -> Self
    where
        O: Default,
    {
        Self::with_op(h, w, O::default())
    }

    pub fn with_op(h: usize, w: usize, op: O) -> Self {
        assert!(h > 0 && w > 0);

        let offset_row = h.next_power_of_two();
        let offset_col = w.next_power_of_two();

        let nodes_len_rows = 2 * offset_row;
        let nodes_len_cols = 2 * offset_col;
        let nodes_len = nodes_len_rows * nodes_len_cols;

        let nodes = (0..nodes_len).map(|_| op.identity()).collect();

        Self {
            len_rows: h,
            len_cols: w,
            offset_row,
            offset_col,
            nodes,
            nodes_len_cols,
            op,
        }
    }

    #[inline(always)]
    fn idx(&self, i: usize, j: usize) -> usize {
        i * self.nodes_len_cols + j
    }

    pub fn get(&self, i: usize, j: usize) -> &O::Value {
        assert!(i < self.len_rows && j < self.len_cols);
        &self.nodes[self.idx(i + self.offset_row, j + self.offset_col)]
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, j: usize, value: O::Value) {
        *self.entry_mut(i, j) = value;
    }

    pub fn entry_mut(&mut self, i: usize, j: usize) -> EntryMut<'_, O> {
        assert!(i < self.len_rows && j < self.len_cols);
        EntryMut {
            seg: self,
            index_row: i,
            index_col: j,
        }
    }

    pub fn fold(
        &self,
        row_range: impl RangeBounds<usize>,
        col_range: impl RangeBounds<usize>,
    ) -> O::Value {
        let Range { start: il, end: ir } = to_half_open_index_range(row_range, self.len_rows);
        let Range { start: jl, end: jr } = to_half_open_index_range(col_range, self.len_cols);
        assert!(il <= ir && ir <= self.len_rows);
        assert!(jl <= jr && jr <= self.len_cols);

        let mut l = il + self.offset_row;
        let mut r = ir + self.offset_row;

        let mut prod_l = self.op.identity();
        let mut prod_r = self.op.identity();

        while l < r {
            if l % 2 == 1 {
                let tmp = self.fold_col(l, jl..jr);
                prod_l = self.op.op(&prod_l, &tmp);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                let tmp = self.fold_col(r, jl..jr);
                prod_r = self.op.op(&tmp, &prod_r);
            }

            l /= 2;
            r /= 2;
        }

        self.op.op(&prod_l, &prod_r)
    }

    fn rebuild_col(&mut self, node_index_i: usize, j: usize) {
        let mut node_index_j = j + self.offset_col;
        while node_index_j > 1 {
            node_index_j /= 2;
            let index = self.idx(node_index_i, node_index_j);
            let index_l = self.idx(node_index_i, 2 * node_index_j);
            let index_r = self.idx(node_index_i, 2 * node_index_j + 1);
            self.nodes[index] = self.op.op(&self.nodes[index_l], &self.nodes[index_r]);
        }
    }

    fn fold_col(&self, node_index_i: usize, col_range: Range<usize>) -> O::Value {
        let Range { start: jl, end: jr } = col_range;

        let mut l = jl + self.offset_col;
        let mut r = jr + self.offset_col;

        let mut prod_l = self.op.identity();
        let mut prod_r = self.op.identity();

        while l < r {
            if l % 2 == 1 {
                prod_l = self.op.op(&prod_l, &self.nodes[self.idx(node_index_i, l)]);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                prod_r = self.op.op(&self.nodes[self.idx(node_index_i, r)], &prod_r);
            }

            l /= 2;
            r /= 2;
        }

        self.op.op(&prod_l, &prod_r)
    }
}

impl<O: Monoid> From<(Vec<Vec<O::Value>>, O)> for SegmentTree2d<O> {
    fn from((v, op): (Vec<Vec<O::Value>>, O)) -> Self {
        assert!(!v.is_empty());
        assert!(!v[0].is_empty());
        debug_assert!(v.iter().all(|vi| vi.len() == v[0].len()));

        let h = v.len();
        let w = v[0].len();

        let mut seg = Self::with_op(h, w, op);

        for (i, vi) in v.into_iter().enumerate() {
            let node_index_i = i + seg.offset_row;

            for (j, vij) in vi.into_iter().enumerate() {
                let index = seg.idx(node_index_i, j + seg.offset_col);
                seg.nodes[index] = vij;
            }

            for j in (1..seg.offset_col).rev() {
                let index = seg.idx(node_index_i, j);
                let index_l = seg.idx(node_index_i, 2 * j);
                let index_r = seg.idx(node_index_i, 2 * j + 1);
                seg.nodes[index] = seg.op.op(&seg.nodes[index_l], &seg.nodes[index_r]);
            }
        }

        for i in (1..seg.offset_row).rev() {
            for j in 1..2 * seg.offset_col {
                let index = seg.idx(i, j);
                let index_l = seg.idx(2 * i, j);
                let index_r = seg.idx(2 * i + 1, j);
                seg.nodes[index] = seg.op.op(&seg.nodes[index_l], &seg.nodes[index_r]);
            }
        }

        seg
    }
}

impl<O: Monoid, const N: usize, const M: usize> From<([[O::Value; M]; N], O)> for SegmentTree2d<O> {
    fn from((v, op): ([[O::Value; M]; N], O)) -> Self {
        let v: Vec<Vec<_>> = v.into_iter().map(|vi| vi.into_iter().collect()).collect();
        Self::from((v, op))
    }
}

impl<O: Monoid + Default> From<Vec<Vec<O::Value>>> for SegmentTree2d<O> {
    fn from(v: Vec<Vec<O::Value>>) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid + Default, const N: usize, const M: usize> From<[[O::Value; M]; N]>
    for SegmentTree2d<O>
{
    fn from(v: [[O::Value; M]; N]) -> Self {
        Self::from((v, O::default()))
    }
}

impl<O: Monoid> Index<(usize, usize)> for SegmentTree2d<O> {
    type Output = O::Value;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<O: Monoid> Index<[usize; 2]> for SegmentTree2d<O> {
    type Output = O::Value;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.get(index[0], index[1])
    }
}

pub struct EntryMut<'a, O: Monoid> {
    seg: &'a mut SegmentTree2d<O>,
    index_row: usize,
    index_col: usize,
}

impl<'a, O: Monoid> Deref for EntryMut<'a, O> {
    type Target = O::Value;
    fn deref(&self) -> &Self::Target {
        self.seg.get(self.index_row, self.index_col)
    }
}

impl<'a, O: Monoid> DerefMut for EntryMut<'a, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let node_index_i = self.index_row + self.seg.offset_row;
        let node_index_j = self.index_col + self.seg.offset_col;
        let index = self.seg.idx(node_index_i, node_index_j);
        &mut self.seg.nodes[index]
    }
}

impl<'a, O: Monoid> Drop for EntryMut<'a, O> {
    fn drop(&mut self) {
        let mut node_index_i = self.index_row + self.seg.offset_row;

        self.seg.rebuild_col(node_index_i, self.index_col);

        while node_index_i > 1 {
            node_index_i /= 2;

            let node_index_j = self.index_col + self.seg.offset_col;

            let index = self.seg.idx(node_index_i, node_index_j);
            let index_l = self.seg.idx(2 * node_index_i, node_index_j);
            let index_r = self.seg.idx(2 * node_index_i + 1, node_index_j);
            self.seg.nodes[index] = self
                .seg
                .op
                .op(&self.seg.nodes[index_l], &self.seg.nodes[index_r]);

            self.seg.rebuild_col(node_index_i, self.index_col);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{ops::op_add::OpAdd, utils::test_utils::initialize_rng};

    #[test]
    fn test_2d_add_basic() {
        let a = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        let mut seg = SegmentTree2d::<OpAdd<i64>>::from(a);

        assert_eq!(
            seg.fold(.., ..),
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12
        );

        assert_eq!(seg.fold(1..3, 1..4), 6 + 7 + 8 + 10 + 11 + 12);

        seg.set(0, 0, 100);
        assert_eq!(*seg.get(0, 0), 100);
        assert_eq!(seg.fold(0..1, 0..2), 100 + 2);
    }

    #[test]
    fn test_add_random() {
        let mut rng = initialize_rng();

        for _ in 0..50 {
            let h = rng.random_range(1..=20);
            let w = rng.random_range(1..=20);
            let mut a = vec![vec![0i64; w]; h];
            for i in 0..h {
                for j in 0..w {
                    a[i][j] = rng.random_range(-50..=50);
                }
            }
            let mut seg = SegmentTree2d::<OpAdd<i64>>::from(a.clone());

            for _ in 0..2000 {
                if rng.random_bool(0.5) {
                    // update
                    let i = rng.random_range(0..h);
                    let j = rng.random_range(0..w);
                    let v = rng.random_range(-50..=50);
                    a[i][j] = v;
                    seg.set(i, j, v);
                } else {
                    // query
                    let il = rng.random_range(0..=h);
                    let ir = rng.random_range(il..=h);
                    let jl = rng.random_range(0..=w);
                    let jr = rng.random_range(jl..=w);

                    let mut naive = 0i64;
                    for i in il..ir {
                        for j in jl..jr {
                            naive += a[i][j];
                        }
                    }
                    assert_eq!(seg.fold(il..ir, jl..jr), naive);
                }
            }
        }
    }
}
