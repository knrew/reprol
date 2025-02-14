//! 出力時に便利な関数など

use std::fmt::{Display, Write};

pub trait IteratorFormater {
    fn join_with(&mut self, sep: &str) -> String;
}

pub trait VecFormater {
    fn join_with(&self, sep: &str) -> String;
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

impl<T> VecFormater for Vec<T>
where
    T: Display,
{
    fn join_with(&self, sep: &str) -> String {
        self.iter().join_with(sep)
    }
}

#[cfg(test)]
mod tests {
    use super::{IteratorFormater, VecFormater};

    #[test]
    fn test_join() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().join_with(" "), "1 2 3 4 5");
        assert_eq!(v.join_with(" "), "1 2 3 4 5");
    }
}
