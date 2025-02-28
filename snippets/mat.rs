//! 行列計算

/// 正方行列mに対してm^expを計算する
fn pow<T>(m: &[Vec<T>], mut exp: u64) -> Vec<Vec<T>>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Integer,
{
    assert_eq!(m.len(), m[0].len());

    let mut res = vec![vec![T::zero(); m.len()]; m.len()];
    for i in 0..m.len() {
        res[i][i] = T::one();
    }

    let mut base = m.to_vec();

    while exp > 0 {
        if exp & 1 == 1 {
            res = mul(&res, &base);
        }
        base = mul(&base, &base);

        exp >>= 1;
    }

    res
}

// 行列mとnの積を計算する
fn mul<T>(m: &[Vec<T>], n: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Integer,
{
    let nrows_m = m.len();
    let ncols_m = m[0].len();
    let ncols_n = n[0].len();

    let mut res = vec![vec![T::zero(); ncols_n]; nrows_m];

    for i in 0..nrows_m {
        for j in 0..ncols_n {
            for k in 0..ncols_m {
                res[i][j] = res[i][j].clone() + m[i][k].clone() * n[k][j].clone();
            }
        }
    }

    res
}

pub trait Integer {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Integer for Mi {
    fn zero() -> Self {
        0.into()
    }
    fn one() -> Self {
        1.into()
    }
}
