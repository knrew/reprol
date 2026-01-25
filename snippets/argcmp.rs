use std::cmp::Ordering;

/// 2次元平面上の点`(x, y)`を偏角順に並べるための比較関数．
///
/// - 原点は入力として不可．
/// - 始線(偏角0)は正のx軸方向(`(1, 0)`)方向．
///
/// ## Reference
/// - [整数のまま行う偏角ソート（行列式のあれです。）の実装バリエーションの検討とご紹介です。 - ブログ名](https://ngtkana.hatenablog.com/entry/2021/11/13/202103)
pub fn argcmp((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> Ordering {
    debug_assert_ne!((x0, y0), (0, 0));
    debug_assert_ne!((x1, y1), (0, 0));
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}
