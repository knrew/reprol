//! 転倒数(inversion number)
//!
//! 数列の転倒数を計算する関数．
//! 転倒数とは，`i < j` かつ `a[i] > a[j]` を満たす組`(i, j)`の個数．
//!
//! # 使用例
//! ```
//! use reprol::inversion::Inversion;
//! let v = vec![3, 1, 4, 1, 5];
//! assert_eq!(v.inversion(), 3);
//! ```

use crate::{bisect::Bounds, ds::fenwick_tree::FenwickTree, ops::op_add::OpAdd};

pub trait Inversion {
    fn inversion(&self) -> u64;
}

impl<T: Ord> Inversion for [T] {
    fn inversion(&self) -> u64 {
        let mut v = self.iter().collect::<Vec<_>>();
        v.sort_unstable();
        v.dedup();

        let mut res = 0;
        let mut ft = FenwickTree::<OpAdd<_>>::new(v.len());

        for e in self {
            let i = v.lower_bound(&e);
            res += ft.fold(i + 1..);
            ft.op(i, &1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::inversion::*;

    #[test]
    fn test_inversion() {
        {
            let v: Vec<i32> = vec![];
            assert_eq!(v.inversion(), 0);
        }

        {
            let v = vec![5];
            assert_eq!(v.inversion(), 0);
        }

        {
            let v = vec![1, 2, 3, 4, 5];
            assert_eq!(v.inversion(), 0);
        }

        {
            let v = vec![5, 4, 3, 2, 1];
            assert_eq!(v.inversion(), 10);
        }

        {
            let v = vec![2, 3, 3, 2, 1];
            assert_eq!(v.inversion(), 6);
        }

        {
            let v = vec![3, 1, 2, 5, 4];
            assert_eq!(v.inversion(), 3);
        }

        {
            let v = vec![7, 7, 7, 7];
            assert_eq!(v.inversion(), 0);
        }
    }

    #[test]
    fn test_inversion_random() {
        fn naive<T: Ord>(v: &[T]) -> u64 {
            let n = v.len();

            let mut res = 0;

            for i in 0..n {
                for j in i + 1..n {
                    if v[i] > v[j] {
                        res += 1;
                    }
                }
            }

            res
        }

        macro_rules! define_test_function {
            ($name:ident, $ty:ident) => {
                fn $name(rng: &mut StdRng) {
                    const T: usize = 100;
                    const N: usize = 100;
                    for _ in 0..T {
                        let v = (0..N).map(|_| rng.gen()).collect::<Vec<$ty>>();
                        assert_eq!(v.inversion(), naive(&v));
                    }
                }
            };
        }

        define_test_function!(test_i64, i64);
        define_test_function!(test_u64, u64);

        let mut rng = StdRng::seed_from_u64(30);
        test_i64(&mut rng);
        test_u64(&mut rng);
    }
}
