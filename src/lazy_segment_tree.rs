use std::ops::{Range, RangeBounds};

use crate::{action::MonoidAction, monoid::Monoid, range::to_open_range};

pub struct LazySegmentTree<M, A>
where
    M: Monoid,
    A: MonoidAction<M>,
{
    len: usize,
    offset: usize,
    log: u32,
    nodes: Vec<M::Value>,
    lazy: Vec<A::Value>,
    monoid: M,
    action: A,
}

impl<M, A> LazySegmentTree<M, A>
where
    M: Monoid,
    M::Value: Clone,
    A: MonoidAction<M>,
    A::Value: Clone + PartialEq,
{
    pub fn new(v: Vec<M::Value>) -> Self
    where
        M: Default,
        A: Default,
    {
        Self::from(v)
    }

    pub fn with_len(n: usize) -> Self
    where
        M: Default,
        A: Default,
    {
        Self::with_op(n, M::default(), A::default())
    }

    /// 演算(モノイドと作用)を引数で指定
    pub fn with_op(n: usize, monoid: M, action: A) -> Self {
        let offset = n.next_power_of_two();
        let log = offset.trailing_zeros();
        let nodes = vec![monoid.identity(); 2 * offset];
        let lazy = vec![action.identity(); 2 * offset];
        Self {
            len: n,
            offset,
            log,
            nodes,
            lazy,
            monoid,
            action,
        }
    }

    pub fn get(&mut self, index: usize) -> &M::Value {
        assert!(index < self.len);
        let index = index + self.offset;
        self.push(index);
        &self.nodes[index]
    }

    pub fn set(&mut self, index: usize, value: M::Value) {
        assert!(index < self.len);
        let index = index + self.offset;
        self.push(index);
        self.nodes[index] = value;
        self.pull(index);
    }

    /// 区間内の要素すべてにfを作用させる
    pub fn apply(&mut self, range: impl RangeBounds<usize>, f: &A::Value) {
        let Range { start: l, end: r } = to_open_range(range, self.len);

        assert!(r <= self.len);
        assert!(l <= r);

        if l == r {
            return;
        }

        let (l, r) = (l + self.offset, r + self.offset);

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push_lazy(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push_lazy((r - 1) >> i);
            }
        }

        {
            let mut l = l;
            let mut r = r;

            while l < r {
                if l & 1 != 0 {
                    self.apply_lazy(l, &f);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.apply_lazy(r, &f);
                }
                l >>= 1;
                r >>= 1;
            }
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.pull_node(l >> i);
            }
            if ((r >> i) << i) != r {
                self.pull_node((r - 1) >> i);
            }
        }
    }

    /// 区間積を取得する
    pub fn product(&mut self, range: impl RangeBounds<usize>) -> M::Value {
        let Range { start: l, end: r } = to_open_range(range, self.len);

        assert!(r <= self.len);
        assert!(l <= r);

        if l == r {
            return self.monoid.identity();
        }

        let (mut l, mut r) = (l + self.offset, r + self.offset);

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push_lazy(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push_lazy((r - 1) >> i);
            }
        }

        let mut sum_left = self.monoid.identity();
        let mut sum_right = self.monoid.identity();

        while l < r {
            if l & 1 != 0 {
                sum_left = self.monoid.op(&sum_left, &self.nodes[l]);
                l += 1;
            }

            if r & 1 != 0 {
                r -= 1;
                sum_right = self.monoid.op(&self.nodes[r], &sum_right)
            }

            l >>= 1;
            r >>= 1;
        }

        self.monoid.op(&sum_left, &sum_right)
    }

    /// f(op(a[l], a[l + 1], ..., a[r - 1])) = true となる最大のr
    pub fn max_right(&mut self, l: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(l <= self.len);

        #[cfg(debug_assertions)]
        assert!(f(&self.monoid.identity()));

        if l == self.len {
            return self.len;
        }

        let mut l = l + self.offset;
        self.push(l);

        let mut sum = self.monoid.identity();

        loop {
            while l % 2 == 0 {
                l >>= 1;
            }

            if !f(&self.monoid.op(&sum, &self.nodes[l])) {
                while l < self.offset {
                    self.push_lazy(l);
                    l = 2 * l;
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

        self.len
    }

    /// f(op(a[l], a[l + 1], ..., a[r - 1])) = true となる最小のl
    pub fn min_left(&mut self, r: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        assert!(r <= self.len);

        #[cfg(debug_assertions)]
        assert!(f(&self.monoid.identity()));

        if r == 0 {
            return 0;
        }

        let mut r = r + self.offset;
        self.push(r - 1);
        let mut sum = self.monoid.identity();

        loop {
            r -= 1;
            while r > 1 && (r % 2 == 1) {
                r >>= 1;
            }
            if !f(&self.monoid.op(&self.nodes[r], &sum)) {
                while r < self.offset {
                    self.push_lazy(r);
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

    fn pull_node(&mut self, k: usize) {
        self.nodes[k] = self.monoid.op(&self.nodes[2 * k], &self.nodes[2 * k + 1]);
    }

    fn pull(&mut self, k: usize) {
        for i in 1..=self.log {
            self.pull_node(k >> i);
        }
    }

    fn push_lazy(&mut self, k: usize) {
        if self.lazy[k] == self.action.identity() {
            return;
        }
        let lzk = self.lazy[k].clone();
        self.apply_lazy(2 * k, &lzk);
        self.apply_lazy(2 * k + 1, &lzk);
        self.lazy[k] = self.action.identity();
    }

    fn push(&mut self, k: usize) {
        for i in (1..=self.log).rev() {
            self.push_lazy(k >> i);
        }
    }

    fn apply_lazy(&mut self, k: usize, f: &A::Value) {
        self.nodes[k] = self.action.act(f, &self.nodes[k]);
        if k < self.offset {
            self.lazy[k] = self.action.op(f, &self.lazy[k]);
        }
    }
}

impl<M, A> From<Vec<M::Value>> for LazySegmentTree<M, A>
where
    M: Monoid + Default,
    M::Value: Clone,
    A: MonoidAction<M> + Default,
    A::Value: Clone + PartialEq,
{
    fn from(v: Vec<M::Value>) -> Self {
        Self::from(v.as_slice())
    }
}

impl<M, A> From<&Vec<M::Value>> for LazySegmentTree<M, A>
where
    M: Monoid + Default,
    M::Value: Clone,
    A: MonoidAction<M> + Default,
    A::Value: Clone + PartialEq,
{
    fn from(v: &Vec<M::Value>) -> Self {
        Self::from(v.as_slice())
    }
}

impl<M, A> From<&[M::Value]> for LazySegmentTree<M, A>
where
    M: Monoid + Default,
    M::Value: Clone,
    A: MonoidAction<M> + Default,
    A::Value: Clone + PartialEq,
{
    fn from(v: &[M::Value]) -> Self {
        let mut res = Self::with_op(v.len(), M::default(), A::default());

        for i in 0..res.len {
            res.nodes[i + res.offset] = v[i].clone();
        }

        for i in (1..res.offset).rev() {
            res.pull_node(i)
        }

        res
    }
}

impl<M, A> FromIterator<M::Value> for LazySegmentTree<M, A>
where
    M: Monoid + Default,
    M::Value: Clone,
    A: MonoidAction<M> + Default,
    A::Value: Clone + PartialEq,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::{LazySegmentTree, Monoid, MonoidAction};

    #[derive(Default)]
    struct Op;

    impl Monoid for Op {
        type Value = i64;

        fn identity(&self) -> Self::Value {
            0
        }

        fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
            *x.max(y)
        }
    }

    #[derive(Default)]
    struct Act;

    impl Monoid for Act {
        type Value = i64;

        fn identity(&self) -> Self::Value {
            0
        }

        fn op(&self, g: &Self::Value, f: &Self::Value) -> Self::Value {
            f + g
        }
    }

    impl MonoidAction<Op> for Act {
        fn act(&self, f: &Self::Value, x: &<Op as Monoid>::Value) -> <Op as Monoid>::Value {
            x + f
        }
    }

    #[test]
    fn test_lazy_segment_tree() {
        let v = vec![4, 4, 4, 4, 4];
        let mut seg = LazySegmentTree::<Op, Act>::from(&v);
        seg.apply(1..4, &2);
        assert_eq!(
            (0..5).map(|i| *seg.get(i)).collect::<Vec<_>>(),
            vec![4, 6, 6, 6, 4]
        );
        assert_eq!(seg.product(0..=2), 6);
        seg.apply(0..5, &(-1));
        assert_eq!(
            (0..5).map(|i| *seg.get(i)).collect::<Vec<_>>(),
            vec![3, 5, 5, 5, 3]
        );
        assert_eq!(seg.product(..), 5);
    }
}
