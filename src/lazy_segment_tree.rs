use std::ops::{Bound, RangeBounds};

pub trait MonoidAction {
    type Value;
    type Operator;
    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value;
    fn identity(&self) -> Self::Value;
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
    M::Operator: Clone,
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

    fn update(&mut self, k: usize) {
        self.nodes[k] = self.monoid.op(&self.nodes[2 * k], &self.nodes[2 * k + 1]);
    }

    fn apply_lazy(&mut self, k: usize, op: &M::Operator) {
        self.nodes[k] = self.monoid.map(op, &self.nodes[k]);
        if k < self.offset {
            self.lazy[k] = self.monoid.compose(op, &self.lazy[k]);
        }
    }

    fn push_lazy(&mut self, k: usize) {
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

    pub fn get(&mut self, index: usize) -> &M::Value {
        let index = index + self.offset;
        self.push(index);
        &self.nodes[index]
    }

    pub fn set(&mut self, index: usize, value: M::Value) {
        let index = index + self.offset;
        self.push(index);
        self.nodes[index] = value;
        for i in 1..=self.log {
            self.update(index >> i);
        }
    }

    pub fn apply<R: RangeBounds<usize>>(&mut self, range: R, f: &M::Operator) {
        let l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        } + self.offset;

        let r = match range.end_bound() {
            Bound::Unbounded => self.len,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        } + self.offset;

        if l == r {
            return;
        }

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
                if l % 2 == 1 {
                    self.apply_lazy(l, &f);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    self.apply_lazy(r, &f);
                }
                l /= 2;
                r /= 2;
            }
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn product<R: RangeBounds<usize>>(&mut self, range: R) -> M::Value {
        let mut l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        } + self.offset;

        let mut r = match range.end_bound() {
            Bound::Unbounded => self.len,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        } + self.offset;

        if l == r {
            return self.monoid.identity();
        }

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
            if l % 2 == 1 {
                sum_left = self.monoid.op(&sum_left, &self.nodes[l]);
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                sum_right = self.monoid.op(&self.nodes[r], &sum_right)
            }
            l /= 2;
            r /= 2;
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
            while l % 2 == 0 {
                l /= 2;
            }

            if !f(&self.monoid.op(&sum, &self.nodes[l])) {
                while l < self.offset {
                    self.push_lazy(l);
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
            while r > 1 && r % 2 == 1 {
                r /= 2;
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
}
