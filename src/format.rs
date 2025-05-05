//! 出力時に便利な関数など

use std::fmt::{Display, Write};

pub trait IterFormatter {
    /// 要素のイテレータからsep区切りで文字列に変換する．
    fn to_string(&mut self, sep: &str) -> String;
}

impl<I> IterFormatter for I
where
    I: Iterator,
    I::Item: Display,
{
    fn to_string(&mut self, sep: &str) -> String {
        let mut res = String::new();
        if let Some(item) = self.next() {
            write!(&mut res, "{}", item).unwrap();
            while let Some(item) = self.next() {
                write!(&mut res, "{}{}", sep, item).unwrap();
            }
        }
        res
    }
}

pub trait ArrayFormatter {
    /// 配列をsep区切りで文字列に変換する．
    fn to_string(&self, sep: &str) -> String;
}

impl<T> ArrayFormatter for [T]
where
    T: Display,
{
    fn to_string(&self, sep: &str) -> String {
        self.iter().to_string(sep)
    }
}

pub trait Usize1ArrayFormatter {
    /// usizeの配列を1-indexedの文字列に変換する．
    fn to_string_usize1(&self, sep: &str) -> String;
}

impl Usize1ArrayFormatter for [usize] {
    fn to_string_usize1(&self, sep: &str) -> String {
        self.iter().map(|i| i + 1).to_string(sep)
    }
}

pub trait CharsFormatter {
    // char型またはu8型の配列を文字列に変換する．
    fn as_string(&self) -> String;
}

impl CharsFormatter for [char] {
    fn as_string(&self) -> String {
        self.iter().collect::<String>()
    }
}

impl CharsFormatter for [u8] {
    fn as_string(&self) -> String {
        self.iter().map(|&c| c as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::{ArrayFormatter, CharsFormatter, IterFormatter};

    #[test]
    fn test_as_string() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().to_string(" "), "1 2 3 4 5");
        assert_eq!(v.to_string(" "), "1 2 3 4 5");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(vec!['A', 'B', 'C'].as_string(), String::from("ABC"));
        assert_eq!(['d', 'e', 'f'].as_string(), String::from("def"));
        assert_eq!(vec!['1', '2', '3'].as_string(), String::from("123"));
        assert_eq!(vec![b'A', b'B', b'C'].as_string(), String::from("ABC"));
        assert_eq!(b"def".as_string(), String::from("def"));
    }
}
