use std::fmt::{Display, Write};

/// 出力時に便利な関数など
pub trait Format {
    fn join_with(&mut self, sep: &str) -> String;
}

impl<I> Format for I
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

#[cfg(test)]
mod tests {
    use crate::format::Format;

    #[test]
    fn test_formatter() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().join_with(" "), "1 2 3 4 5");
    }
}
