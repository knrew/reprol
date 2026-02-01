//! Range Tree
//!
//! 2次元平面上の点集合を管理するデータ構造．
//! 点は事前に登録し，登録した点に対して以下の操作を行うことができる．
//! - 要素の1点変更．
//! - 任意の長方形区間に含まれる点の要素の総積(和，最小値など)の取得．
//!
//! # 計算量
//! - 構築: `O(N log N)` ただし N は点の数
//! - 点の更新: `O(log^2 N)`
//! - 長方形区間クエリ: `O(log^2 N)`
//!
//! # 使用例
//! ```
//! use reprol::{ds::range_tree::RangeTree, ops::op_add::OpAdd};
//!
//! let points = vec![(10, 3), (10, 8), (20, 3), (30, 100)];
//! let mut seg = RangeTree::<OpAdd<i64>>::new(points);
//! seg.set((10, 3), 5);
//! seg.set((20, 3), 7);
//! assert_eq!(seg.fold(10..30, 0..10), 12);
//! ```

use std::ops::Range;

use crate::{bisect::Bounds, ds::segment_tree::SegmentTree, ops::monoid::Monoid};

/// Range Tree
pub struct RangeTree<O: Monoid> {
    // 葉ノードの開始インデックス
    offset: usize,

    /// ソートされたuniqueなx座標のリスト
    xs: Vec<O::Element>,

    /// 各ノードのソートされたuniqueなy座標のリスト
    ys: Vec<Vec<O::Element>>,

    /// 各ノードのy方向のセグメントツリー
    y_segs: Vec<SegmentTree<O>>,

    /// 演算(モノイド)
    op: O,
}

impl<O: Monoid> RangeTree<O>
where
    O::Element: Ord,
{
    /// 点の集合`points`からRange Treeを構築する．
    pub fn new(points: impl IntoIterator<Item = (O::Element, O::Element)>) -> Self
    where
        O: Default + Clone,
        O::Element: Clone,
    {
        Self::with_op(points, O::default())
    }

    /// 演算`op`を指定して，点の集合`points`からRange Treeを構築する．
    pub fn with_op(points: impl IntoIterator<Item = (O::Element, O::Element)>, op: O) -> Self
    where
        O: Clone,
        O::Element: Clone,
    {
        let points = points.into_iter().collect::<Vec<_>>();

        let mut xs = points.iter().map(|(x, _)| x.clone()).collect::<Vec<_>>();
        xs.sort_unstable();
        xs.dedup();

        let nx = xs.len();
        let offset = nx.next_power_of_two().max(1);
        let n_nodes = 2 * offset;

        let mut ys = vec![vec![]; n_nodes];
        for (x, y) in points {
            let xi = xs.lower_bound(&x);
            assert!(xi < nx && xs[xi] == x);

            let mut v = xi + offset;
            while v > 0 {
                ys[v].push(y.clone());
                v /= 2;
            }
        }

        let mut y_segs = Vec::with_capacity(n_nodes);

        for ysi in &mut ys {
            ysi.sort_unstable();
            ysi.dedup();
            y_segs.push(SegmentTree::with_op(ysi.len(), op.clone()));
        }

        Self {
            offset,
            xs,
            ys,
            y_segs,
            op,
        }
    }

    /// 点`point`の値を返す．
    ///
    /// # 戻り値
    ///
    /// `point` が登録されている場合はその値を返す．
    /// 登録されていない場合は `None` を返す．
    pub fn get(&self, point: (O::Element, O::Element)) -> Option<&O::Element> {
        let (x, y) = point;
        let xi = self.xs.lower_bound(&x);
        (xi < self.xs.len() && self.xs[xi] == x)
            .then(|| self.get_at(xi + self.offset, &y))
            .flatten()
    }

    /// 点`point`の値を`value`に更新する．
    ///
    /// # パニック
    ///
    /// `point` が事前に登録されていない点である場合は panic する．
    pub fn set(&mut self, point: (O::Element, O::Element), value: O::Element)
    where
        O::Element: Clone,
    {
        let (x, y) = point;
        let xi = self.xs.lower_bound(&x);

        assert!(
            xi < self.xs.len() && self.xs[xi] == x,
            "point not registered in RangeTree"
        );

        let mut index = xi + self.offset;
        self.set_at(index, &y, value);

        while index > 1 {
            let p = index / 2;

            let value_l = self
                .get_at(2 * p, &y)
                .cloned()
                .unwrap_or_else(|| self.op.id());
            let value_r = self
                .get_at(2 * p + 1, &y)
                .cloned()
                .unwrap_or_else(|| self.op.id());
            self.set_at(p, &y, self.op.op(&value_l, &value_r));

            index = p
        }
    }

    /// 長方形区間`[x_range] × [y_range]`に含まれる点の要素の総積を返す．
    pub fn fold(&self, x_range: Range<O::Element>, y_range: Range<O::Element>) -> O::Element {
        let mut l = self.xs.lower_bound(&x_range.start);
        let mut r = self.xs.lower_bound(&x_range.end);

        assert!(l <= r);

        l += self.offset;
        r += self.offset;

        let mut prod_l = self.op.id();
        let mut prod_r = self.op.id();

        while l < r {
            if l % 2 == 1 {
                let tmp = self.fold_y(l, &y_range.start, &y_range.end);
                prod_l = self.op.op(&prod_l, &tmp);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                let tmp = self.fold_y(r, &y_range.start, &y_range.end);
                prod_r = self.op.op(&tmp, &prod_r);
            }

            l /= 2;
            r /= 2;
        }

        self.op.op(&prod_l, &prod_r)
    }

    /// ノード`index`におけるy座標`y`の要素を`value`に更新する．
    fn set_at(&mut self, index: usize, y: &O::Element, value: O::Element) {
        let ys = &self.ys[index];
        let yi = ys.lower_bound(y);
        assert!(
            yi < ys.len() && &ys[yi] == y,
            "point not registered in RangeTree"
        );
        self.y_segs[index].set(yi, value);
    }

    /// ノード`index`におけるy座標`y`の要素を返す．
    fn get_at(&self, index: usize, y: &O::Element) -> Option<&O::Element> {
        let ys = &self.ys[index];
        let yi = ys.lower_bound(y);
        (yi < ys.len() && &ys[yi] == y).then(|| self.y_segs[index].get(yi))
    }

    /// ノード`index`におけるy区間`[yl, yr)`の要素の総積を返す．
    fn fold_y(&self, index: usize, yl: &O::Element, yr: &O::Element) -> O::Element {
        let ys = &self.ys[index];
        if ys.is_empty() {
            return self.op.id();
        }
        let l = ys.lower_bound(yl);
        let r = ys.lower_bound(yr);
        assert!(l <= r);
        self.y_segs[index].fold(l..r)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::{
        ops::{op_add::OpAdd, op_max::OpMax, op_min::OpMin},
        utils::test_utils::initialize_rng,
    };

    #[test]
    fn test_add() {
        let points = vec![(10, 3), (10, 8), (20, 3), (30, 100)];
        let mut seg = RangeTree::<OpAdd<i64>>::new(points);
        seg.set((10, 3), 5);
        seg.set((20, 3), 7);
        assert_eq!(seg.fold(10..30, 0..10), 12);
        assert_eq!(seg.fold(10..31, 0..101), 12);
    }

    #[test]
    fn test_min() {
        let points = vec![(5, 2), (5, 6), (10, 3), (10, 7), (15, 1)];
        let mut seg = RangeTree::<OpMin<i32>>::new(points);
        seg.set((5, 2), 10);
        seg.set((5, 6), 2);
        seg.set((10, 3), 5);
        seg.set((10, 7), 3);
        seg.set((15, 1), 1);
        assert_eq!(seg.fold(5..11, 2..8), 2);
        assert_eq!(seg.fold(5..16, 1..8), 1);
    }

    #[test]
    fn test_max() {
        let points = vec![(5, 2), (5, 6), (10, 3), (10, 7), (15, 1)];
        let mut seg = RangeTree::<OpMax<i32>>::new(points);
        seg.set((5, 2), 10);
        seg.set((5, 6), 20);
        seg.set((10, 3), 15);
        seg.set((10, 7), 5);
        seg.set((15, 1), 30);
        assert_eq!(seg.fold(5..11, 2..8), 20);
        assert_eq!(seg.fold(5..16, 1..8), 30);
    }

    #[test]
    fn test_get() {
        let points = vec![(10, 3), (10, 8), (20, 3), (30, 100)];
        let mut seg = RangeTree::<OpAdd<i64>>::new(points);
        seg.set((10, 3), 5);
        seg.set((20, 3), 7);
        assert_eq!(seg.get((10, 3)), Some(&5));
        assert_eq!(seg.get((20, 3)), Some(&7));
        assert_eq!(seg.get((10, 8)), Some(&0));
        assert_eq!(seg.get((40, 50)), None);
        assert_eq!(seg.get((10, 50)), None);
    }

    #[test]
    fn test_custom_monoid_mod() {
        #[derive(Clone, Copy, Debug)]
        struct OpModAdd {
            m: i64,
        }

        impl Monoid for OpModAdd {
            type Element = i64;

            fn id(&self) -> Self::Element {
                0
            }

            fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
                (lhs + rhs) % self.m
            }
        }

        let op = OpModAdd { m: 7 };
        let points = vec![(1, 1), (1, 2), (2, 1), (2, 2)];
        let mut seg = RangeTree::with_op(points, op);
        seg.set((1, 1), 10);
        seg.set((1, 2), 20);
        seg.set((2, 1), 30);
        seg.set((2, 2), 40);
        assert_eq!(seg.fold(1..3, 1..3), (10 + 20 + 30 + 40) % 7);
        assert_eq!(seg.fold(1..2, 1..3), (10 + 20) % 7);
    }

    macro_rules! random_test {
        ($test_name:ident, $ty:ty, $op:ty, $fold_init:expr, $fold_op:expr, $val_range:expr) => {
            #[test]
            fn $test_name() {
                use std::collections::HashMap;

                let mut rng = initialize_rng();

                const T: usize = 50;
                const Q: usize = 2000;
                const N_MAX: usize = 20;

                for _ in 0..T {
                    let n = rng.random_range(1..=N_MAX);
                    let mut points = Vec::new();
                    for _ in 0..n {
                        let x = rng.random_range(0..N_MAX as $ty);
                        let y = rng.random_range(0..N_MAX as $ty);
                        points.push((x, y));
                    }
                    points.sort_unstable();
                    points.dedup();

                    let mut values = HashMap::new();
                    let mut seg = RangeTree::<$op>::new(points.clone());

                    for (x, y) in points.iter() {
                        let v = rng.random_range($val_range);
                        values.insert((*x, *y), v);
                        seg.set((*x, *y), v);
                    }

                    for _ in 0..Q {
                        match rng.random_range(0..=2) {
                            0 => {
                                let idx = rng.random_range(0..points.len());
                                let (x, y) = points[idx];
                                let v = rng.random_range($val_range);
                                values.insert((x, y), v);
                                seg.set((x, y), v);
                                assert_eq!(seg.get((x, y)), Some(&v));
                            }
                            1 => {
                                let idx = rng.random_range(0..points.len());
                                let (x, y) = points[idx];
                                let d = rng.random_range($val_range);
                                *values.get_mut(&(x, y)).unwrap() += d;
                                seg.set((x, y), values[&(x, y)]);
                                assert_eq!(seg.get((x, y)), Some(&values[&(x, y)]));
                            }
                            2 => {
                                let xl = rng.random_range(0..N_MAX as $ty);
                                let xr = rng.random_range(xl..N_MAX as $ty);
                                let yl = rng.random_range(0..N_MAX as $ty);
                                let yr = rng.random_range(yl..N_MAX as $ty);

                                let mut naive = $fold_init;
                                for ((x, y), v) in values.iter() {
                                    if xl <= *x && *x < xr && yl <= *y && *y < yr {
                                        naive = $fold_op(naive, *v);
                                    }
                                }
                                assert_eq!(seg.fold(xl..xr, yl..yr), naive);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        };
    }

    random_test!(
        test_random_add_i64,
        i64,
        OpAdd<i64>,
        0i64,
        |a, b| a + b,
        -50..=50
    );
    random_test!(
        test_random_add_u64,
        u64,
        OpAdd<u64>,
        0,
        |a, b| a + b,
        0..=50
    );

    random_test!(
        test_random_min_i32,
        i32,
        OpMin<i32>,
        i32::MAX,
        |a: i32, b| a.min(b),
        -100..=100
    );
    random_test!(
        test_random_min_u32,
        u32,
        OpMin<_>,
        u32::MAX,
        |a: u32, b| a.min(b),
        0..=100
    );

    random_test!(
        test_random_max_i32,
        i32,
        OpMax<i32>,
        i32::MIN,
        |a: i32, b| a.max(b),
        -100..=100
    );

    random_test!(
        test_random_max_u32,
        u32,
        OpMax<_>,
        u32::MIN,
        |a: u32, b| a.max(b),
        0..=100
    );

    #[test]
    #[should_panic(expected = "point not registered in RangeTree")]
    fn test_set_unregistered_point() {
        let points = vec![(10, 3), (10, 8), (20, 3), (30, 100)];
        let mut seg = RangeTree::<OpAdd<i64>>::new(points);
        seg.set((40, 50), 5);
    }

    #[test]
    #[should_panic(expected = "point not registered in RangeTree")]
    fn test_set_unregistered_y() {
        let points = vec![(10, 3), (10, 8), (20, 3), (30, 100)];
        let mut seg = RangeTree::<OpAdd<i64>>::new(points);
        seg.set((10, 50), 5);
    }
}
