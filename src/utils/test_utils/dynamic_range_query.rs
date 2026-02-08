/// 点更新と範囲foldクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$naive_op`: fold演算(例: `|a: $ty, b| a + b`)
/// - `$naive_id`: fold演算の単位元
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造のfold操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
/// - `$element_value_range`: 要素値の範囲
macro_rules! randomized_point_set_range_fold_test {
    (
        $test_name: ident,
        $ty: ty,
        $naive_op: expr,
        $naive_id: expr,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr,
        $element_value_range: expr
    ) => {
        #[test]
        fn $test_name() {
            let mut rng = get_test_rng();

            for _ in 0..$num_testcases {
                let n = rng.random_range(1..=$num_elements_max);

                let mut v_naive: Vec<$ty> = (0..n)
                    .map(|_| rng.random_range($element_value_range))
                    .collect();
                let mut ds = $ds_from_vec(v_naive.clone());

                for _ in 0..$num_queries {
                    // set
                    // v[index] <- value
                    {
                        let index = rng.random_range(0..n);
                        let value = rng.random_range($element_value_range);
                        v_naive[index] = value;
                        $ds_set(&mut ds, index, value);
                    }

                    // range query
                    {
                        let l = rng.random_range(0..n);
                        let r = rng.random_range(l + 1..=n);
                        let naive = v_naive[l..r]
                            .iter()
                            .fold($naive_id, |prod, &vi| $naive_op(prod, vi));
                        assert_eq!($ds_fold(&mut ds, l..r), naive);
                    }
                }
            }
        }
    };
}
pub(crate) use randomized_point_set_range_fold_test;

/// 点更新と範囲和クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造の範囲和操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
/// - `$element_value_range`: 要素値の範囲
macro_rules! randomized_point_set_range_sum_test {
    (
        $test_name: ident,
        $ty: ty,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr,
        $element_value_range: expr
    ) => {
        randomized_point_set_range_fold_test!(
            $test_name,
            $ty,
            |a: $ty, b| a + b,
            0 as $ty,
            $ds_from_vec,
            $ds_fold,
            $ds_set,
            $num_testcases,
            $num_queries,
            $num_elements_max,
            $element_value_range
        );
    };
}
pub(crate) use randomized_point_set_range_sum_test;

/// 点更新と範囲最小クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造の範囲最小操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
macro_rules! randomized_point_set_range_min_test {
    (
        $test_name: ident,
        $ty: ty,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr
    ) => {
        randomized_point_set_range_fold_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.min(b),
            <$ty>::MAX,
            $ds_from_vec,
            $ds_fold,
            $ds_set,
            $num_testcases,
            $num_queries,
            $num_elements_max,
            <$ty>::MIN..=<$ty>::MAX
        );
    };
}
pub(crate) use randomized_point_set_range_min_test;

/// 点更新と範囲最大クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造の範囲最大操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
macro_rules! randomized_point_set_range_max_test {
    (
        $test_name: ident,
        $ty: ty,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr
    ) => {
        randomized_point_set_range_fold_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.max(b),
            <$ty>::MIN,
            $ds_from_vec,
            $ds_fold,
            $ds_set,
            $num_testcases,
            $num_queries,
            $num_elements_max,
            <$ty>::MIN..=<$ty>::MAX
        );
    };
}
pub(crate) use randomized_point_set_range_max_test;

/// 点更新と範囲XORクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造の範囲XOR操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
macro_rules! randomized_point_set_range_xor_test {
    (
        $test_name: ident,
        $ty: ty,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr
    ) => {
        randomized_point_set_range_fold_test!(
            $test_name,
            $ty,
            |a: $ty, b| a ^ b,
            0 as $ty,
            $ds_from_vec,
            $ds_fold,
            $ds_set,
            $num_testcases,
            $num_queries,
            $num_elements_max,
            <$ty>::MIN..=<$ty>::MAX
        );
    };
}
pub(crate) use randomized_point_set_range_xor_test;

/// 点更新と範囲GCDクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造の範囲GCD操作
/// - `$ds_set`: データ構造の点更新操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
macro_rules! randomized_point_set_range_gcd_test {
    (
        $test_name: ident,
        $ty: ty,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_set: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr
    ) => {
        randomized_point_set_range_fold_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.gcd(b),
            0 as $ty,
            $ds_from_vec,
            $ds_fold,
            $ds_set,
            $num_testcases,
            $num_queries,
            $num_elements_max,
            <$ty>::MIN / 2..=<$ty>::MAX / 2
        );
    };
}
pub(crate) use randomized_point_set_range_gcd_test;

/// 範囲作用、bisectクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$naive_fold_op`: fold演算(例: `|a: $ty, b| a.max(b)`)
/// - `$naive_fold_id`: fold演算の単位元
/// - `$naive_act`: act演算(例: `|x: $ty, f: $ty| x + f`)
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造のfold操作
/// - `$ds_act`: データ構造のact操作
/// - `$ds_bisect_right`: データ構造のbisect_right操作
/// - `$ds_bisect_left`: データ構造のbisect_left操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
/// - `$element_value_range`: 要素値の範囲
/// - `$action_value_range`: 作用値の範囲
macro_rules! randomized_range_act_bisect_test {
    (
        $test_name: ident,
        $ty: ty,
        $naive_fold_op: expr,
        $naive_fold_id: expr,
        $naive_act: expr,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_act: expr,
        $ds_bisect_right: expr,
        $ds_bisect_left: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr,
        $element_value_range: expr,
        $action_value_range: expr
    ) => {
        #[test]
        fn $test_name() {
            let mut rng = get_test_rng();

            for _ in 0..$num_testcases {
                let n = rng.random_range(1..=$num_elements_max);

                let mut v_naive: Vec<$ty> = (0..n)
                    .map(|_| rng.random_range($element_value_range))
                    .collect();
                let mut ds = $ds_from_vec(v_naive.clone());

                for _ in 0..$num_queries {
                    match rng.random_range(0..=2) {
                        0 => {
                            // range act
                            let l = rng.random_range(0..n);
                            let r = rng.random_range(l + 1..=n);
                            let f = rng.random_range($action_value_range);
                            for e in v_naive[l..r].iter_mut() {
                                *e = $naive_act(*e, f);
                            }
                            $ds_act(&mut ds, l..r, f);
                        }
                        1 => {
                            // bisect_right
                            let l = rng.random_range(0..=n);
                            let threshold = rng.random_range($element_value_range);
                            // naive: 最大の r で fold(l..r) <= threshold
                            let mut naive = l;
                            let mut cur = $naive_fold_id;
                            for r in l..n {
                                cur = $naive_fold_op(cur, v_naive[r]);
                                if cur <= threshold {
                                    naive = r + 1;
                                } else {
                                    break;
                                }
                            }
                            let ds_result = $ds_bisect_right(&mut ds, l, |m: &$ty| *m <= threshold);
                            assert_eq!(ds_result, naive);
                        }
                        2 => {
                            // bisect_left
                            let r = rng.random_range(0..=n);
                            let threshold = rng.random_range($element_value_range);
                            // naive: 最小の l で fold(l..r) <= threshold
                            let mut naive = r;
                            let mut cur = $naive_fold_id;
                            for l in (0..r).rev() {
                                cur = $naive_fold_op(v_naive[l], cur);
                                if cur <= threshold {
                                    naive = l;
                                } else {
                                    break;
                                }
                            }
                            let ds_result = $ds_bisect_left(&mut ds, r, |m: &$ty| *m <= threshold);
                            assert_eq!(ds_result, naive);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}
pub(crate) use randomized_range_act_bisect_test;

/// 範囲加算、bisectクエリのランダムテストを生成するマクロ（最大値用）
macro_rules! randomized_range_add_bisect_max_test {
    ($test_name: ident, $ty: ty, $range: expr) => {
        randomized_range_act_bisect_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.max(b),
            <$ty>::MIN,
            |x: $ty, f: $ty| x.wrapping_add(f),
            |v| LazySegmentTree::<OpMax<$ty>, ActAdd<$ty>>::from(v),
            |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
            |ds: &mut LazySegmentTree<_, _>, range, f| ds.act(range, &f),
            |ds: &mut LazySegmentTree<_, _>, l, f| ds.bisect_right(l, f),
            |ds: &mut LazySegmentTree<_, _>, r, f| ds.bisect_left(r, f),
            20,     // T
            50000,  // Q
            100,    // N_MAX
            $range,
            $range
        );
    };
}
pub(crate) use randomized_range_add_bisect_max_test;

/// 範囲加算、bisectクエリのランダムテストを生成するマクロ（最小値用）
macro_rules! randomized_range_add_bisect_min_test {
    ($test_name: ident, $ty: ty, $range: expr) => {
        randomized_range_act_bisect_ge_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.min(b),
            <$ty>::MAX,
            |x: $ty, f: $ty| x.wrapping_add(f),
            |v| LazySegmentTree::<OpMin<$ty>, ActAdd<$ty>>::from(v),
            |ds: &mut LazySegmentTree<_, _>, range| ds.fold(range),
            |ds: &mut LazySegmentTree<_, _>, range, f| ds.act(range, &f),
            |ds: &mut LazySegmentTree<_, _>, l, f| ds.bisect_right(l, f),
            |ds: &mut LazySegmentTree<_, _>, r, f| ds.bisect_left(r, f),
            20,     // T
            50000,  // Q
            100,    // N_MAX
            $range,
            $range
        );
    };
}
pub(crate) use randomized_range_add_bisect_min_test;

/// 範囲作用、bisectクエリのランダムテストを生成するマクロ（最小値用、>=条件）
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$naive_fold_op`: fold演算(例: `|a: $ty, b| a.min(b)`)
/// - `$naive_fold_id`: fold演算の単位元
/// - `$naive_act`: act演算(例: `|x: $ty, f: $ty| x + f`)
/// - `$ds_from_vec`: データ構造をVecから構築する式
/// - `$ds_fold`: データ構造のfold操作
/// - `$ds_act`: データ構造のact操作
/// - `$ds_bisect_right`: データ構造のbisect_right操作
/// - `$ds_bisect_left`: データ構造のbisect_left操作
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズの最大値
/// - `$element_value_range`: 要素値の範囲
/// - `$action_value_range`: 作用値の範囲
macro_rules! randomized_range_act_bisect_ge_test {
    (
        $test_name: ident,
        $ty: ty,
        $naive_fold_op: expr,
        $naive_fold_id: expr,
        $naive_act: expr,
        $ds_from_vec: expr,
        $ds_fold: expr,
        $ds_act: expr,
        $ds_bisect_right: expr,
        $ds_bisect_left: expr,
        $num_testcases: expr,
        $num_queries: expr,
        $num_elements_max: expr,
        $element_value_range: expr,
        $action_value_range: expr
    ) => {
        #[test]
        fn $test_name() {
            let mut rng = get_test_rng();

            for _ in 0..$num_testcases {
                let n = rng.random_range(1..=$num_elements_max);

                let mut v_naive: Vec<$ty> = (0..n)
                    .map(|_| rng.random_range($element_value_range))
                    .collect();
                let mut ds = $ds_from_vec(v_naive.clone());

                for _ in 0..$num_queries {
                    match rng.random_range(0..=2) {
                        0 => {
                            // range act
                            let l = rng.random_range(0..n);
                            let r = rng.random_range(l + 1..=n);
                            let f = rng.random_range($action_value_range);
                            for e in v_naive[l..r].iter_mut() {
                                *e = $naive_act(*e, f);
                            }
                            $ds_act(&mut ds, l..r, f);
                        }
                        1 => {
                            // bisect_right (>= condition for min)
                            let l = rng.random_range(0..=n);
                            let threshold = rng.random_range($element_value_range);
                            // naive: 最大の r で fold(l..r) >= threshold
                            let mut naive = l;
                            let mut cur = $naive_fold_id;
                            for r in l..n {
                                cur = $naive_fold_op(cur, v_naive[r]);
                                if cur >= threshold {
                                    naive = r + 1;
                                } else {
                                    break;
                                }
                            }
                            let ds_result = $ds_bisect_right(&mut ds, l, |m: &$ty| *m >= threshold);
                            assert_eq!(ds_result, naive);
                        }
                        2 => {
                            // bisect_left (>= condition for min)
                            let r = rng.random_range(0..=n);
                            let threshold = rng.random_range($element_value_range);
                            // naive: 最小の l で fold(l..r) >= threshold
                            let mut naive = r;
                            let mut cur = $naive_fold_id;
                            for l in (0..r).rev() {
                                cur = $naive_fold_op(v_naive[l], cur);
                                if cur >= threshold {
                                    naive = l;
                                } else {
                                    break;
                                }
                            }
                            let ds_result = $ds_bisect_left(&mut ds, r, |m: &$ty| *m >= threshold);
                            assert_eq!(ds_result, naive);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}
pub(crate) use randomized_range_act_bisect_ge_test;
