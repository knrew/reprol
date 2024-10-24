//! グリッド上の隣接するマスを列挙する

fn collect_neighbors(
    &(h, w): &(usize, usize),
    &(i, j): &(usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    const D: [(usize, usize); 4] = [
        (1, 0),
        (0, 1),
        (1usize.wrapping_neg(), 0),
        (0, 1usize.wrapping_neg()),
    ];
    (0..4)
        .map(move |k| (i.wrapping_add(D[k].0), j.wrapping_add(D[k].1)))
        .filter(move |&(i, j)| i < h && j < w)
        .map(move |(i, j)| (i, j))
}

fn func() {
    // vec
    let collect_neighbors = |(i, j): (usize, usize)| -> Vec<(usize, usize)> {
        const D: [(usize, usize); 4] = [
            (1, 0),
            (0, 1),
            (1usize.wrapping_neg(), 0),
            (0, 1usize.wrapping_neg()),
        ];
        (0..4)
            .map(|k| (i.wrapping_add(D[i].0), j.wrapping_add(D[i].1)))
            .filter(|&(i, j)| i < h && j < w)
            .collect()
    };

    // iterator
    const D: [(usize, usize); 4] = [
        (1, 0),
        (0, 1),
        (1usize.wrapping_neg(), 0),
        (0, 1usize.wrapping_neg()),
    ];
    let iter = (0..4)
        .map(|k| (i.wrapping_add(D[i].0), j.wrapping_add(D[i].1)))
        .filter(move |&(i, j)| i < h && j < w);
}
