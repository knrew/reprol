use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

/// for AOJ
pub struct Scanner {
    buffer: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    fn update(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for e in input.split_whitespace() {
            self.buffer.push_back(e.to_string());
        }
    }

    pub fn read_string(&mut self) -> String {
        if self.buffer.is_empty() {
            self.update();
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
}
