//! ランレングス圧縮(run-length encoding)
//!
//! 配列をランレングス圧縮する．
//! 連続する同じ要素をまとめて，その値とその個数のペアで表現する．
//! たとえば，文字列`"aaabbbbcc"`に対してランレングス圧縮を行うと，`[('a', 3), ('b', 4), ('c', 2)]`となる．
//!
//! # 使用例
//! ```
//! use reprol::rle::Rle;
//!
//! let s = b"aaabbbbcc";
//! assert_eq!(s.rle(), vec![(b'a', 3), (b'b', 4), (b'c', 2)]);
//!
//! let v = vec![1, 1, 2, 2, 2, 3, 3];
//! assert_eq!(v.rle(), vec![(1, 2), (2, 3), (3, 2)]);
//! ```

pub trait Rle {
    type Output;

    fn rle(&self) -> Self::Output;
}

impl<T> Rle for [T]
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

#[cfg(test)]
mod tests {
    use super::*;

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
