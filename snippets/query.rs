enum Query {
    Q1(usize),
    Q2(usize),
}
use Query::*;

impl Readable for Query {
    type Output = Self;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        match u8::read(source) {
            1 => {
                input! {
                    from source,
                    x: usize,
                }
                Q1(x)
            }
            2 => {
                input! {
                    from source,
                    x: usize,
                }
                Q2(x)
            }
            _ => unreachable!(),
        }
    }
}
