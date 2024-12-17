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

impl_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

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
