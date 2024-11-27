/// ランレングス圧縮(run-length encodeing)
pub fn rle<T>(s: &[T]) -> Vec<(T, usize)>
where
    T: Copy + PartialEq,
{
    let mut res = vec![];

    for &c in s {
        match res.last_mut() {
            Some((last_c, count)) if *last_c == c => {
                *count += 1;
            }
            _ => {
                res.push((c, 1));
            }
        }
    }

    res
}
