use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

/// 主に AOJ(Aizu Online Judge)用
pub struct Io {
    buffer: VecDeque<String>,
}

impl Io {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    fn read_line(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for e in input.split_whitespace() {
            self.buffer.push_back(e.to_string());
        }
    }

    pub fn read_string(&mut self) -> String {
        while self.buffer.is_empty() {
            self.read_line();
        }
        return self.buffer.pop_front().unwrap();
    }

    pub fn read_chars(&mut self) -> Vec<char> {
        self.read_string().chars().collect()
    }

    pub fn read<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.read_string().parse::<T>().unwrap()
    }

    pub fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr + Sized,
        T::Err: Debug,
    {
        (0..n).map(|_| self.read::<T>()).collect()
    }
}
