use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

/// for AOJ
pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn read_line(&mut self) {
        let mut input = String::new();
        self.stdin.read_line(&mut input).unwrap();
        for e in input.split_whitespace() {
            self.buffer.push_back(e.to_string());
        }
    }

    pub fn get_string(&mut self) -> String {
        if self.buffer.is_empty() {
            self.read_line();
        }
        return self.buffer.pop_front().unwrap();
    }

    pub fn get<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.get_string().parse::<T>().unwrap()
    }
}
