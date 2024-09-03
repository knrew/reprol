use crate::integer::Integer;

pub fn gcd<T: Integer>(m: T, n: T) -> T {
    if n == T::ZERO {
        abs(m)
    } else {
        gcd(n, m % n)
    }
}

pub fn lcm<T: Integer>(m: T, n: T) -> T {
    abs(m) / gcd(m, n) * abs(n)
}

fn abs<T: Integer>(n: T) -> T {
    if n < T::ZERO {
        T::ZERO - n
    } else {
        n
    }
}
