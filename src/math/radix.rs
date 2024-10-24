/// 10進法の非負整数をbase進法に変換する
/// base^iの係数がres[i]に格納される
pub trait ToRadix {
    fn to_radix(self, base: Self) -> Vec<u8>;
}

macro_rules! impl_integer {
    ($($ty:ident),*) => {$(

        impl ToRadix for $ty {
            fn to_radix(self, base: Self) -> Vec<u8> {
                let mut n = self;
                let mut res = vec![];
                while n > 0 {
                    let x = (n % base) as u8;
                    res.push(x);
                    n /= base;
                }
                res
            }
        }

    )*};
}

impl_integer! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
