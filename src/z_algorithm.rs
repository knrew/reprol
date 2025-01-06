pub trait ZAlgorithm {
    fn z_algorithm(&self) -> Vec<usize>;
}

impl<T> ZAlgorithm for Vec<T>
where
    T: PartialEq,
{
    fn z_algorithm(&self) -> Vec<usize> {
        z_algorithm(self)
    }
}

pub fn z_algorithm<T>(s: &[T]) -> Vec<usize>
where
    T: PartialEq,
{
    if s.is_empty() {
        return vec![];
    }

    let n = s.len();

    let mut z = vec![0; n];
    z[0] = s.len();

    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        z[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < n && k + z[k] < j {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }

    z
}
