// 転倒数を計算する
fn inversion<T: Ord>(v: &[T]) -> u64 {
    let mut mp = BTreeMap::new();
    for e in v {
        mp.entry(e).or_insert(0);
    }
    mp.iter_mut().enumerate().for_each(|(i, (_, v))| *v = i);

    let mut res = 0;
    let mut ft = FenwickTree::<OpAdd<_>>::new(mp.len());

    for vi in v.iter().map(|e| mp[e]) {
        res += ft.fold(vi + 1..);
        ft.op(vi, &1);
    }

    res
}
