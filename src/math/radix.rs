pub trait ToRadix {
    /// 10進法の非負整数をn進法に変換する
    /// 返り値の配列のi番目の要素は変換後の桁数をdとしてbase^{d-i}の係数が格納される
    fn to_radix(self, base: Self) -> Vec<u8>;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(
        impl ToRadix for $ty {
            fn to_radix(self, base: Self) -> Vec<u8> {
                if self == 0 {
                    return vec![0];
                }
                let mut n = self;
                let mut res = vec![];
                while n > 0 {
                    let x = (n % base) as u8;
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
