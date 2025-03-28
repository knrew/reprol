/// 転倒数を計算する
use reprol::{ds::fenwick_tree::FenwickTree, ops::op_add::OpAdd};
fn inversion<T: Copy + Ord>(v: &[T]) -> u64 {
    // 座標圧縮
    let (mx, v) = {
        let mut mp = BTreeMap::new();
        for &e in v {
            mp.entry(e).or_insert(0);
        }
        mp.iter_mut().enumerate().for_each(|(i, (_, v))| *v = i);

        let mx = mp.len();
        let v = v.iter().map(|e| mp[e]).collect::<Vec<_>>();
        (mx, v)
    };

    let mut res = 0;
    let mut ft = FenwickTree::<OpAdd<_>>::new(mx);

    for vi in v {
        res += ft.product(vi + 1..);
        ft.mul(vi, &1);
    }

    res
}
