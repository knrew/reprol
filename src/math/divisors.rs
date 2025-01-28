/// NOTE: イテレータまわりはrustのバージョンアップデート後に修正する
pub trait Divisors: Sized {
    type Output: Iterator<Item = Self>;

    /// 約数を列挙する
    /// 返り値は昇順にソート済み
    fn divisors(self) -> Self::Output;

    /// 約数を列挙する
    /// 返り値はソートされていない
    fn divisors_unsorted(self) -> Self::Output;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl Divisors for $ty {
            type Output = <Vec<Self> as IntoIterator>::IntoIter;

            #[allow(unused_comparisons)]
            fn divisors_unsorted(self) -> Self::Output {
                debug_assert!(self >= 0);
                let n = self;
                (1..)
                    .take_while(|i| i * i <= n)
                    .filter(|i| n % i == 0)
                    .flat_map(|i| if n / i == i { vec![i] } else { vec![i, n / i] }.into_iter())
                    .collect::<Vec<_>>()
                    .into_iter()
            }

            fn divisors(self) -> Self::Output {
                let mut res = self.divisors_unsorted().collect::<Vec<_>>();
                res.sort_unstable();
                res.into_iter()
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

#[cfg(test)]
mod tests {
    use super::Divisors;

    #[test]
    fn test_divisors() {
        let test_cases = vec![
            (1i32, vec![1]),
            (2, vec![1, 2]),
            (3, vec![1, 3]),
            (4, vec![1, 2, 4]),
            (6, vec![1, 2, 3, 6]),
            (12, vec![1, 2, 3, 4, 6, 12]),
            (28, vec![1, 2, 4, 7, 14, 28]),
            (36, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]),
            (47, vec![1, 47]),
            (49, vec![1, 7, 49]),
            (100, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(n.divisors().collect::<Vec<_>>(), expected);
        }
    }
}
