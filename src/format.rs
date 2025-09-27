//! 出力時に便利な関数など

use std::fmt::{Debug, Display};

fn format_iter_display<I>(
    f: &mut std::fmt::Formatter<'_>,
    iter: &mut I,
    sep: &str,
) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Display,
{
    if let Some(item) = iter.next() {
        write!(f, "{}", item)?;
        for item in iter.by_ref() {
            write!(f, "{}{}", sep, item)?;
        }
    }
    Ok(())
}

fn format_iter_debug<I>(
    f: &mut std::fmt::Formatter<'_>,
    iter: &mut I,
    sep: &str,
) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Debug,
{
    if let Some(item) = iter.next() {
        write!(f, "{:?}", item)?;
        for item in iter.by_ref() {
            write!(f, "{}{:?}", sep, item)?;
        }
    }
    Ok(())
}

pub struct FormatIter<I>(pub I, pub &'static str);
pub struct FormatVec<'a, T>(pub &'a [T], pub &'static str);
pub struct FormatUsize1Vec<'a>(pub &'a [usize], pub &'static str);
pub struct FormatChars<'a>(pub &'a [char]);
pub struct FormatBytes<'a>(pub &'a [u8]);

macro_rules! impl_fmt {
    ($fmt_trait:ident, $fmt_func:ident) => {
        impl<I> $fmt_trait for FormatIter<I>
        where
            I: Iterator + Clone,
            I::Item: $fmt_trait,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut iter = self.0.clone();
                let sep = self.1;
                $fmt_func(f, &mut iter, sep)
            }
        }

        impl<'a, T> $fmt_trait for FormatVec<'a, T>
        where
            T: $fmt_trait,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut iter = self.0.iter();
                let sep = self.1;
                $fmt_func(f, &mut iter, sep)
            }
        }

        impl<'a> $fmt_trait for FormatUsize1Vec<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut iter = self.0.iter().map(|i| i + 1);
                let sep = self.1;
                $fmt_func(f, &mut iter, sep)
            }
        }

        impl<'a> $fmt_trait for FormatChars<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut iter = self.0.iter();
                $fmt_func(f, &mut iter, "")
            }
        }

        impl<'a> $fmt_trait for FormatBytes<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut iter = self.0.iter().map(|&c| c as char);
                $fmt_func(f, &mut iter, "")
            }
        }
    };
}

impl_fmt!(Display, format_iter_display);
impl_fmt!(Debug, format_iter_debug);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_iter() {
        {
            let v = vec![3, 1, 4, 1, 5];
            assert_eq!(format!("{}", FormatIter(v.iter(), " ")), "3 1 4 1 5");
            assert_eq!(format!("{}", FormatIter(v.iter(), ", ")), "3, 1, 4, 1, 5");

            assert_eq!(format!("{:?}", FormatIter(v.iter(), " ")), "3 1 4 1 5");
            assert_eq!(format!("{:?}", FormatIter(v.iter(), ", ")), "3, 1, 4, 1, 5");
        }

        {
            use std::collections::VecDeque;
            let v = VecDeque::from(vec![2, 7, 1, 8, 2]);
            assert_eq!(format!("{}", FormatIter(v.iter(), " ")), "2 7 1 8 2");
            assert_eq!(format!("{}", FormatIter(v.iter(), ", ")), "2, 7, 1, 8, 2");

            assert_eq!(format!("{:?}", FormatIter(v.iter(), " ")), "2 7 1 8 2");
            assert_eq!(format!("{:?}", FormatIter(v.iter(), ", ")), "2, 7, 1, 8, 2");
        }
    }

    #[test]
    fn test_format_vec() {
        let v = vec![3, 1, 4, 1, 5];
        assert_eq!(format!("{}", FormatVec(&v, " ")), "3 1 4 1 5");
        assert_eq!(format!("{}", FormatVec(&v, ", ")), "3, 1, 4, 1, 5");

        assert_eq!(format!("{:?}", FormatVec(&v, " ")), "3 1 4 1 5");
        assert_eq!(format!("{:?}", FormatVec(&v, ", ")), "3, 1, 4, 1, 5");
    }

    #[test]
    fn test_format_usize1_vec() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(format!("{}", FormatUsize1Vec(&v, " ")), "1 2 3 4 5");
        assert_eq!(format!("{}", FormatUsize1Vec(&v, ", ")), "1, 2, 3, 4, 5");

        assert_eq!(format!("{:?}", FormatUsize1Vec(&v, " ")), "1 2 3 4 5");
        assert_eq!(format!("{:?}", FormatUsize1Vec(&v, ", ")), "1, 2, 3, 4, 5");
    }

    #[test]
    fn test_format_chars() {
        assert_eq!(format!("{}", FormatChars(&vec!['A', 'B', 'C'])), "ABC");
        assert_eq!(format!("{}", FormatChars(&['d', 'e', 'f'])), "def");
        assert_eq!(format!("{}", FormatChars(&['1', '2', '3'])), "123");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format!("{}", FormatBytes(&vec![b'A', b'B', b'C'])), "ABC");
        assert_eq!(format!("{}", FormatBytes(b"def")), "def");
    }
}
