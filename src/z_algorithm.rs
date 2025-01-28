/// Z-algorithm
/// 配列sに対して，i番目の要素がsとs[i..]の最長共通接頭辞の長さであるような配列を構築する
pub fn construct_z_array<T>(s: &[T]) -> Vec<usize>
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

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::construct_z_array;

    fn check<T: PartialOrd + Debug>(s: &[T]) {
        let z = construct_z_array(&s);
        let n = s.len();
        for i in 0..n {
            let l = z[i];
            assert_eq!(&s[0..l], &s[i..(i + l)]);
            if i + l < s.len() {
                assert!(s[l..(l + 1)] != s[(i + l)..(i + l + 1)]);
            }
        }
    }

    #[test]
    fn test_z_algorithm() {
        let s =b"xkevvvwnqswzyanzdptrcvwcokjkdmlrbbxdwycoeyrlboklgukinxkhrxzfeakjkshqpurjntnrretcqmpvupjiskdagpxubdpjxkevvvwnqswzyanzdptrcvwcokjkdmlrbbxdwycoeyrlboklgukinxkhrxzfeakjkshqpurjntnrretcqmpvupjiskdagpxubdpj";
        check(s);
        let s = b"mgdimasxapolbeewjltejnwnqyhmisbquatxqszeuwlxieqwwumgdimasxapolbeewjltejnwnqyhmisbquatxqszeuwlxieqwwu";
        check(s);
        let s = "eqjxcaarubgfbjiwazubmkyujgjcgegjqzfeqjxcaarubgfbjiwazubmkyujgjcgegjqzf";
        check(s.as_bytes());
    }
}
