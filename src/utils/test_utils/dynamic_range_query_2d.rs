/// 2D配列での点更新と範囲foldクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$naive_op`: fold演算(例: `|a: $ty, b| a + b`)
/// - `$naive_id`: fold演算の単位元
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造のfold操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
/// - `$element_value_range`: 要素値の範囲
macro_rules! randomized_point_set_range_fold_2d_test {
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
                let h = rng.random_range(1..=$num_elements_max);
                let w = rng.random_range(1..=$num_elements_max);

                let mut v_naive: Vec<Vec<$ty>> = (0..h)
                    .map(|_| {
                        (0..w)
                            .map(|_| rng.random_range($element_value_range))
                            .collect()
                    })
                    .collect();

                let mut ds = $ds_from_vec(v_naive.clone());

                for _ in 0..$num_queries {
                    // set
                    // v_naive[i][j] <- value
                    {
                        let i = rng.random_range(0..h);
                        let j = rng.random_range(0..w);
                        let value = rng.random_range($element_value_range);
                        v_naive[i][j] = value;
                        $ds_set(&mut ds, i, j, value);
                    }

                    // range query
                    {
                        let il = rng.random_range(0..h);
                        let ir = rng.random_range(il + 1..=h);
                        let jl = rng.random_range(0..w);
                        let jr = rng.random_range(jl + 1..=w);

                        let naive = v_naive[il..ir].iter().fold($naive_id, |prod, vi| {
                            vi[jl..jr]
                                .iter()
                                .fold(prod, |prod, &vij| $naive_op(prod, vij))
                        });

                        assert_eq!($ds_fold(&ds, il..ir, jl..jr), naive);
                    }
                }
            }
        }
    };
}
pub(crate) use randomized_point_set_range_fold_2d_test;

/// 2D配列での点更新と範囲和クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造の範囲和操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
/// - `$element_value_range`: 要素値の範囲
macro_rules! randomized_point_set_range_sum_2d_test {
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
        randomized_point_set_range_fold_2d_test!(
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
pub(crate) use randomized_point_set_range_sum_2d_test;

/// 2D配列での点更新と範囲最小クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造の範囲最小操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
macro_rules! randomized_point_set_range_min_2d_test {
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
        randomized_point_set_range_fold_2d_test!(
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
pub(crate) use randomized_point_set_range_min_2d_test;

/// 2D配列での点更新と範囲最大クエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造の範囲最大操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
macro_rules! randomized_point_set_range_max_2d_test {
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
        randomized_point_set_range_fold_2d_test!(
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
pub(crate) use randomized_point_set_range_max_2d_test;

/// 2D配列での点更新と範囲XORクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造の範囲XOR操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
macro_rules! randomized_point_set_range_xor_2d_test {
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
        randomized_point_set_range_fold_2d_test!(
            $test_name,
            $ty,
            |a: $ty, b| a ^ b,
            0,
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
pub(crate) use randomized_point_set_range_xor_2d_test;

/// 2D配列での点更新と範囲GCDクエリのランダムテストを生成するマクロ
///
/// # パラメータ
/// - `$test_name`: テスト関数の名前
/// - `$ty`: 要素の型
/// - `$ds_from_vec`: データ構造をVec<Vec>から構築する式
/// - `$ds_fold`: データ構造の範囲GCD操作(`il..ir, jl..jr`を受け取る)
/// - `$ds_set`: データ構造の点更新操作(`i, j, value`を受け取る)
/// - `$num_testcases`: テストケースの数
/// - `$num_queries`: 各テストケースでのクエリ数
/// - `$num_elements_max`: 配列サイズ(row・col)の最大値
macro_rules! randomized_point_set_range_gcd_2d_test {
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
        randomized_point_set_range_fold_2d_test!(
            $test_name,
            $ty,
            |a: $ty, b| a.gcd(b),
            0,
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
pub(crate) use randomized_point_set_range_gcd_2d_test;
