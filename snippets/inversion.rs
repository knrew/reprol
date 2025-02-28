/// 転倒数を求める
fn inversion(v: &[usize]) -> usize {
    let (m, v) = {
        let mut mp = BTreeMap::new();
        for &e in v {
            mp.entry(e).or_insert_with(|| 0);
        }
        mp.iter_mut().enumerate().for_each(|(i, (_, v))| *v = i);
        let res = v.iter().map(|e| mp[e]).collect::<Vec<_>>();
        (mp.len(), res)
    };

    let mut res = 0;
    let mut ft = FenwickTree::<OpAdd<usize>>::new(m);

    for i in 0..v.len() {
        res += ft.product(v[i] + 1..);
        ft.mul(v[i], 1);
    }

    res
}
