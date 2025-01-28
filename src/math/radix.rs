pub trait ToRadix {
    /// 10進法の非負整数をn進法に変換する
    /// 返り値の配列のi番目の要素は変換後の桁数をdとしてbase^{d-i}の係数が格納される
    fn to_radix(self, base: Self) -> Vec<u32>;
}

pub trait FromRadix {
    type Output;

    /// n進法の整数を10進法に変換する
    fn from_radix(&self, n: u32) -> Self::Output;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl ToRadix for $ty {
            fn to_radix(self, base: Self) -> Vec<u32> {
                if self == 0 {
                    return vec![0];
                }
                let mut n = self;
                let mut res = Vec::new();
                while n > 0 {
                    let x = (n % base) as u32;
                    res.push(x);
                    n /= base;
                }
                res.reverse();
                res
            }
        }
    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl FromRadix for String {
    type Output = u64;

    fn from_radix(&self, n: u32) -> Self::Output {
        u64::from_str_radix(self, n).unwrap()
    }
}

impl FromRadix for &str {
    type Output = u64;

    fn from_radix(&self, n: u32) -> Self::Output {
        u64::from_str_radix(self, n).unwrap()
    }
}

/// ToRadixの逆
/// n進法の数(桁数d)のn^iの位の数が(d-i)番目の要素として格納されている配列に対して
/// 10進法に変換した値を返す
/// Bytesをそのまま突っ込むものではないので注意
impl FromRadix for Vec<u32> {
    type Output = u64;
    fn from_radix(&self, n: u32) -> Self::Output {
        let n = n as u64;
        let mut res = 0;
        let mut base = 1;
        for &e in self.iter().rev() {
            res += e as u64 * base;
            base *= n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::{FromRadix, ToRadix};

    #[test]
    fn test_to_radix() {
        let x = 697;
        assert_eq!(x.to_radix(10), vec![6, 9, 7]);
        assert_eq!(x.to_radix(2), vec![1, 0, 1, 0, 1, 1, 1, 0, 0, 1]);
        assert_eq!(x.to_radix(8), vec![1, 2, 7, 1]);
        assert_eq!(x.to_radix(16), vec![2, 11, 9]);
        assert_eq!(x.to_radix(5), vec![1, 0, 2, 4, 2]);
        let x = 1234;
        assert_eq!(x.to_radix(10), vec![1, 2, 3, 4]);
        assert_eq!(x.to_radix(2), vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0]);
        assert_eq!(x.to_radix(8), vec![2, 3, 2, 2]);
        assert_eq!(x.to_radix(16), vec![4, 13, 2]);
        assert_eq!(x.to_radix(5), vec![1, 4, 4, 1, 4]);
    }

    #[test]
    fn test_from_radix() {
        assert_eq!(vec![6, 9, 7].from_radix(10), 697);
        assert_eq!(vec![1, 0, 1, 0, 1, 1, 1, 0, 0, 1].from_radix(2), 697);
        assert_eq!(vec![1, 2, 7, 1].from_radix(8), 697);
        assert_eq!(vec![2, 11, 9].from_radix(16), 697);
        assert_eq!(vec![1, 0, 2, 4, 2].from_radix(5), 697);
        assert_eq!(vec![1, 2, 3, 4].from_radix(10), 1234);
        assert_eq!(vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0].from_radix(2), 1234);
        assert_eq!(vec![2, 3, 2, 2].from_radix(8), 1234);
        assert_eq!(vec![4, 13, 2].from_radix(16), 1234);
        assert_eq!(vec![1, 4, 4, 1, 4].from_radix(5), 1234);
        assert_eq!("1271".from_radix(8), 697);
        assert_eq!("4d2".from_radix(16), 1234);
    }
}
