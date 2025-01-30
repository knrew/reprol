use std::io::{Read, Write};

fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut stdin = s.split_whitespace();
    let mut stdout = std::io::BufWriter::new(std::io::stdout().lock());

    let n = read::<usize>(&mut stdin);
    writeln!(stdout, "{}", n).unwrap();
}

#[inline]
fn read<T>(iter: &mut std::str::SplitWhitespace<'_>) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    iter.next().unwrap().parse::<T>().unwrap()
}
