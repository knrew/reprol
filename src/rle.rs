/// ランレングス圧縮(run-length encodeing)
/// TODO: イテレータにする
pub trait RunLengthEncoding {
    type Output;

    /// ランレングス圧縮を行う
    fn rle(&self) -> Self::Output;
}

impl<T> RunLengthEncoding for Vec<T>
where
    T: Clone + PartialEq,
{
    type Output = Vec<(T, usize)>;

    fn rle(&self) -> Self::Output {
        let mut res = vec![];

        for e in self {
            match res.last_mut() {
                Some((last_e, count)) if last_e == e => {
                    *count += 1;
                }
                _ => {
                    res.push((e.clone(), 1));
                }
            }
        }

        res
    }
}

impl<T> RunLengthEncoding for &[T]
where
    T: Clone + PartialEq,
{
    type Output = Vec<(T, usize)>;

    fn rle(&self) -> Self::Output {
        self.to_vec().rle()
    }
}

#[cfg(test)]
mod tests {
    use super::RunLengthEncoding;

    #[test]
    fn test_rle() {
        let s = "aaabbbcccddeee".chars().collect::<Vec<char>>();
        let expected = vec![('a', 3), ('b', 3), ('c', 3), ('d', 2), ('e', 3)];
        assert_eq!(s.rle(), expected);

        let s = "abcde".chars().collect::<Vec<char>>();
        let expected = vec![('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)];
        assert_eq!(s.rle(), expected);

        assert_eq!(Vec::<char>::new().rle(), Vec::new());
    }
}
