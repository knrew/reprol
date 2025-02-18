//! 出力時に便利な関数など

use std::fmt::{Display, Write};

pub trait IteratorFormater {
    fn join_with(&mut self, sep: &str) -> String;
}

impl<I> IteratorFormater for I
where
    I: Iterator,
    I::Item: Display,
{
    fn join_with(&mut self, sep: &str) -> String {
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

pub trait VecFormater {
    fn join_with(&self, sep: &str) -> String;
}

impl<T> VecFormater for Vec<T>
where
    T: Display,
{
    fn join_with(&self, sep: &str) -> String {
        self.iter().join_with(sep)
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}

impl ToString for [u8] {
    fn to_string(&self) -> String {
        self.iter().map(|&c| c as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::{IteratorFormater, ToString, VecFormater};

    #[test]
    fn test_join() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().join_with(" "), "1 2 3 4 5");
        assert_eq!(v.join_with(" "), "1 2 3 4 5");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(vec!['A', 'B', 'C'].to_string(), String::from("ABC"));
        assert_eq!(['d', 'e', 'f'].to_string(), String::from("def"));
        assert_eq!(vec!['1', '2', '3'].to_string(), String::from("123"));
        assert_eq!(vec![b'A', b'B', b'C'].to_string(), String::from("ABC"));
        assert_eq!(b"def".to_string(), String::from("def"));
    }
}
