use crate::math::modint::ModInt;

/// 法Pのもとで階乗や二項係数を計算する
pub struct ModFactorial<const P: u64> {
    /// 階乗
    factorial: Vec<ModInt<P>>,

    /// 階乗の逆数
    factorial_inv: Vec<ModInt<P>>,
}

impl<const P: u64> ModFactorial<P> {
    /// $0!$から$n!$までの階乗を前計算する
    pub fn new(n: usize) -> Self {
        let mut factorial = vec![ModInt::new(1); n + 1];
        let mut factorial_inv = vec![ModInt::new(1); n + 1];
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i.into();
        }
        factorial_inv[n] = factorial[n].inv();
        for i in (1..=n).rev() {
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

pub type ModFactorial998244353 = ModFactorial<998244353>;
pub type ModFactorial1000000007 = ModFactorial<1000000007>;

#[cfg(test)]
mod tests {
    use super::ModFactorial998244353;

    #[test]
    fn test_factorial() {
        let f = ModFactorial998244353::new(20);
        assert_eq!(f.factorial(0).inner(), 1);
        assert_eq!(f.factorial(1).inner(), 1);
        assert_eq!(f.factorial(2).inner(), 2);
        assert_eq!(f.factorial(3).inner(), 6);
        assert_eq!(f.factorial(4).inner(), 24);
        assert_eq!(f.factorial(10).inner(), 3628800);
        assert_eq!(f.factorial(20).inner(), 401576539);
    }

    #[test]
    fn test_factorial_inv() {
        let f = ModFactorial998244353::new(20);
        assert_eq!(f.factorial_inv(0).inner(), 1);
        assert_eq!(f.factorial_inv(1).inner(), 1);
        assert_eq!(f.factorial_inv(2).inner(), 499122177);
        assert_eq!(f.factorial_inv(3).inner(), 166374059);
        assert_eq!(f.factorial_inv(20).inner(), 400962745);
    }

    #[test]
    fn test_binomial() {
        let f = ModFactorial998244353::new(10);
        assert_eq!(f.binomial(0, 0).inner(), 1);
        assert_eq!(f.binomial(6, 0).inner(), 1);
        assert_eq!(f.binomial(7, 7).inner(), 1);
        assert_eq!(f.binomial(5, 2).inner(), 10);
        assert_eq!(f.binomial(10, 3).inner(), 120);
        assert_eq!(f.binomial(2, 6).inner(), 0);
    }
}
