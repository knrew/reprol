//! 出力時に便利な関数など

use std::fmt::{Display, Write};

pub trait JoinForIterator {
    fn join_with(&mut self, sep: &str) -> String;
    fn join_with_space(&mut self) -> String {
        self.join_with(" ")
    }
    fn join_with_newline(&mut self) -> String {
        self.join_with("\n")
    }
}

impl<I> JoinForIterator for I
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

pub trait JoinForSlice {
    fn join_with(&self, sep: &str) -> String;
    fn join_with_space(&mut self) -> String {
        self.join_with(" ")
    }
    fn join_with_newline(&mut self) -> String {
        self.join_with("\n")
    }
}

impl<T> JoinForSlice for [T]
where
    T: Display,
{
    fn join_with(&self, sep: &str) -> String {
        self.iter().join_with(sep)
    }
}

pub trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for [char] {
    fn as_string(&self) -> String {
        self.iter().collect::<String>()
    }
}

impl AsString for [u8] {
    fn as_string(&self) -> String {
        self.iter().map(|&c| c as char).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::{AsString, JoinForIterator, JoinForSlice};

    #[test]
    fn test_join() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().join_with(" "), "1 2 3 4 5");
        assert_eq!(v.join_with(" "), "1 2 3 4 5");
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
