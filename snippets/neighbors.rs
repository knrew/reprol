//! グリッド上の隣接するマスを列挙する

const D: [[usize; 2]; 4] = [
    [1, 0],
    [0, 1],
    [1usize.wrapping_neg(), 0],
    [0, 1usize.wrapping_neg()],
];

fn collect_neighbors(
    &(h, w): &(usize, usize),
    &(i, j): &(usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    D.iter()
        .map(move |&[di, dj]| (i.wrapping_add(di), j.wrapping_add(dj)))
        .filter(move |&(i, j)| i < h && j < w)
}
