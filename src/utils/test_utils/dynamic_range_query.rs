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
                        assert_eq!($ds_fold(&ds, l..r), naive);
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
