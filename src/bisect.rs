mod bisect {
    use std::{
        fmt::Debug,
        ops::{Add, BitAnd, BitOr, Div, Mul, Shl, Shr, Sub},
    };

    /// x \in [l, r)の範囲を探索
    /// !f(x)となる最小のxを返す(f(x-1)==true and f(x)==false)
    pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> T {
        if !f(&l) {
            return l;
        }
        let (mut ok, mut ng) = (l, r);
        while ng > ok + T::ONE {
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

    pub trait Integer:
        Sized
        + Copy
        + PartialOrd
        + Debug
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
    {
        const ONE: Self;
        const TWO: Self;
    }

    macro_rules! impl_integer {
        ($($ty:ident),*) => {$(
            impl Integer for $ty {
                const ONE: Self = 1;
                const TWO: Self = 2;
            }
        )*};
    }

    impl_integer! { i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize }
}
