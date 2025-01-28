use crate::math::modint::ModInt;

/// 法Pのもとで階乗(やそれに関連する値)を計算する
/// 二項係数など
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

#[cfg(test)]
mod tests {
    use super::FactorialMod998244353;

    #[test]
    fn test_factorial() {
        let f = FactorialMod998244353::new(20);
        assert_eq!(f.factorial(0).value(), 1);
        assert_eq!(f.factorial(1).value(), 1);
        assert_eq!(f.factorial(2).value(), 2);
        assert_eq!(f.factorial(3).value(), 6);
        assert_eq!(f.factorial(4).value(), 24);
        assert_eq!(f.factorial(10).value(), 3628800);
        assert_eq!(f.factorial(20).value(), 401576539);
    }

    #[test]
    fn test_factorial_inv() {
        let f = FactorialMod998244353::new(20);
        assert_eq!(f.factorial_inv(0).value(), 1);
        assert_eq!(f.factorial_inv(1).value(), 1);
        assert_eq!(f.factorial_inv(2).value(), 499122177);
        assert_eq!(f.factorial_inv(3).value(), 166374059);
        assert_eq!(f.factorial_inv(20).value(), 400962745);
    }

    #[test]
    fn test_binomial() {
        let f = FactorialMod998244353::new(10);
        assert_eq!(f.binomial(0, 0).value(), 1);
        assert_eq!(f.binomial(6, 0).value(), 1);
        assert_eq!(f.binomial(7, 7).value(), 1);
        assert_eq!(f.binomial(5, 2).value(), 10);
        assert_eq!(f.binomial(10, 3).value(), 120);
        assert_eq!(f.binomial(2, 6).value(), 0);
    }
}
