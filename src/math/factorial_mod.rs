use crate::math::modint::ModInt;

/// 法Pのもとで階乗(やそれに関連する値)を計算する
pub struct FactorialMod<const P: u64> {
    /// 階乗
    factorial: Vec<ModInt<P>>,

    /// 階乗の逆数
    factorial_inv: Vec<ModInt<P>>,
}

impl<const P: u64> FactorialMod<P> {
    pub fn new(len: usize) -> Self {
        let mut factorial = vec![ModInt::new(1); len + 1];
        let mut factorial_inv = vec![ModInt::new(1); len + 1];
        for i in 1..=len {
            factorial[i] = factorial[i - 1] * i.into();
        }
        factorial_inv[len] = factorial[len].inv();
        for i in (1..=len).rev() {
            factorial_inv[i - 1] = factorial_inv[i] * i.into();
        }
        Self {
            factorial,
            factorial_inv,
        }
    }

    /// 階乗$n!$
    pub fn factorial(&self, n: usize) -> ModInt<P> {
        self.factorial[n]
    }

    /// 階乗の逆数$1/n!$
    pub fn factorial_inv(&self, n: usize) -> ModInt<P> {
        self.factorial_inv[n]
    }

    /// 二項係数${}_n C_k$
    pub fn binomial(&self, n: usize, k: usize) -> ModInt<P> {
        if n < k {
            0.into()
        } else {
            self.factorial[n] * self.factorial_inv[n - k] * self.factorial_inv[k]
        }
    }
}

pub type FactorialMod998244353 = FactorialMod<998244353>;
pub type FactorialMod1000000007 = FactorialMod<1000000007>;
