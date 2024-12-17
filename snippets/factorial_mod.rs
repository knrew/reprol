// n個からk個選ぶ組み合わせ(${}_n C_k$)
let (factorial, factorial_inv) = {
    let mut factorial = vec![Mi::new(1); len + 1];
    let mut factorial_inv = vec![Mi::new(1); len + 1];
    for i in 1..=len {
        factorial[i] = factorial[i - 1] * i.into();
    }
    factorial_inv[len] = factorial[len].inv();
    for i in (1..=len).rev() {
        factorial_inv[i - 1] = factorial_inv[i] * i.into();
    }
    (factorial, factorial_inv)
};

let binominal = |n: usize, k: usize| -> Mi {
    if n < k {
        Mi::new(0)
    } else {
        factorial[n] * factorial_inv[n - k] * factorial_inv[k]
    }
};
