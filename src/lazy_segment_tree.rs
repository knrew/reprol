use std::ops::{Bound, Range, RangeBounds};

pub trait MonoidAction {
    type Value;
    type Operator;
    fn identity(&self) -> Self::Value;
    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value;
    fn identity_operator(&self) -> Self::Operator;
    fn map(&self, f: &Self::Operator, x: &Self::Value) -> Self::Value;
    fn compose(&self, g: &Self::Operator, f: &Self::Operator) -> Self::Operator;
}

pub struct LazySegmentTree<M>
where
    M: MonoidAction,
{
    len: usize,
    offset: usize,
    log: u32,
    nodes: Vec<M::Value>,
    lazy: Vec<M::Operator>,
    monoid: M,
}

impl<M> LazySegmentTree<M>
where
    M: MonoidAction,
    M::Value: Clone,
    M::Operator: Clone + Eq,
{
    pub fn new(len: usize, monoid: M) -> Self {
        let offset = len.next_power_of_two();
        let log = offset.trailing_zeros();
        let nodes = vec![monoid.identity(); 2 * offset];
        let lazy = vec![monoid.identity_operator(); 2 * offset];
        Self {
            len,
            offset,
            log,
            nodes,
            lazy,
            monoid,
        }
    }

    pub fn get(&mut self, index: usize) -> &M::Value {
        let index = index + self.offset;
        self.push(index);
        &self.nodes[index]
    }

    pub fn set(&mut self, index: usize, value: M::Value) {
        let index = index + self.offset;
        self.push(index);
        self.nodes[index] = value;
        self.pull(index);
    }

    pub fn apply<R: RangeBounds<usize>>(&mut self, range: R, f: &M::Operator) {
        let Range { start: l, end: r } = to_open(range, self.len);

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

    pub fn product<R: RangeBounds<usize>>(&mut self, range: R) -> M::Value {
        let Range { start: l, end: r } = to_open(range, self.len);

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

    pub fn max_right(&mut self, l: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        debug_assert!(f(&self.monoid.identity()));

        if l == self.len {
            return self.len;
        }

        let mut l = l + self.offset;
        let mut sum = self.monoid.identity();

        loop {
            while l & 1 == 0 {
                l >>= 1;
            }

            if !f(&self.monoid.op(&sum, &self.nodes[l])) {
                while l < self.offset {
                    self.push_lazy(l);
                    l <<= 2;
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

            if l & (!l + 1) == l {
                break;
            }
        }

        self.len
    }

    pub fn min_left(&mut self, r: usize, mut f: impl FnMut(&M::Value) -> bool) -> usize {
        debug_assert!(f(&self.monoid.identity()));

        if r == 0 {
            return 0;
        }

        let mut r = r + self.offset;
        self.push(r - 1);
        let mut sum = self.monoid.identity();
        loop {
            r -= 1;
            while r > 1 && r & 1 != 0 {
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

            if r & (!r + 1) == r {
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
        if self.lazy[k] == self.monoid.identity_operator() {
            return;
        }
        let lzk = self.lazy[k].clone();
        self.apply_lazy(2 * k, &lzk);
        self.apply_lazy(2 * k + 1, &lzk);
        self.lazy[k] = self.monoid.identity_operator();
    }

    fn push(&mut self, k: usize) {
        for i in (1..=self.log).rev() {
            self.push_lazy(k >> i);
        }
    }

    fn apply_lazy(&mut self, k: usize, f: &M::Operator) {
        self.nodes[k] = self.monoid.map(f, &self.nodes[k]);
        if k < self.offset {
            self.lazy[k] = self.monoid.compose(f, &self.lazy[k]);
        }
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

impl<M> From<(&[M::Value], M)> for LazySegmentTree<M>
where
    M: MonoidAction,
    M::Value: Clone,
    M::Operator: Clone + Eq,
{
    fn from((v, monoid): (&[M::Value], M)) -> Self {
        let mut res = Self::new(v.len(), monoid);

        for i in 0..res.len {
            res.nodes[i + res.offset] = v[i].clone();
        }

        for i in (1..res.offset).rev() {
            res.pull_node(i)
        }

        res
    }
}

impl<M> From<(&Vec<M::Value>, M)> for LazySegmentTree<M>
where
    M: MonoidAction,
    M::Value: Clone,
    M::Operator: Clone + Eq,
{
    fn from((v, m): (&Vec<M::Value>, M)) -> Self {
        Self::from((v.as_slice(), m))
    }
}

impl<M> From<&[M::Value]> for LazySegmentTree<M>
where
    M: MonoidAction + Default,
    M::Value: Clone,
    M::Operator: Clone + Eq,
{
    fn from(v: &[M::Value]) -> Self {
        Self::from((v, M::default()))
    }
}

impl<M> From<&Vec<M::Value>> for LazySegmentTree<M>
where
    M: MonoidAction + Default,
    M::Value: Clone,
    M::Operator: Clone + Eq,
{
    fn from(v: &Vec<M::Value>) -> Self {
        Self::from((v, M::default()))
    }
}
