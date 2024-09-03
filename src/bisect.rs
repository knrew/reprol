use crate::integer::Integer;

/// x \in [l, r)の範囲を探索
/// !f(x)となる最小のxを返す(f(x-1)==true and f(x)==false)
pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> T {
    if !f(&l) {
        return l;
    }
    let (mut ok, mut ng) = (l, r);
    while ng > ok + T::ONE {
        // TODO: implement checked_mid
        let mid = ok + (ng - ok) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    ng
}

pub trait LowerBound {
    type Item: Ord;
    fn lower_bound(&self, x: &Self::Item) -> usize;
}

impl<T: Ord> LowerBound for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> usize {
        bisect(0, self.len(), |&i| unsafe { self.get_unchecked(i) } < x)
    }
}
