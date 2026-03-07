//! 区間和演算
//!
//! 区間和(値と長さを同じ型`T`で持つペア)を演算とする可換モノイド．
//! 単位元は `{ value: 0, len: 0 }`(整数型では`0`，`ModInt`では`ModInt::new(0)`)．
//!
//! LazySegmentTree で区間加算・区間代入・アフィン変換と組み合わせて
//! 区間和クエリを処理するための演算として利用する．
//!
//! # Examples
//!
//! ```
//! use reprol::ops::monoid::Monoid;
//! use reprol::ops::op_range_sum::{OpRangeSum, OpRangeSumElement};
//!
//! let m = OpRangeSum::<i64>::default();
//! let a = OpRangeSumElement::leaf(3);
//! let b = OpRangeSumElement::leaf(5);
//! assert_eq!(m.op(&a, &b).value(), 8);
//! assert_eq!(m.op(&m.id(), &a), a);
//! ```
//!
//! # Notes
//!
//! - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
//! - `OpRangeSumElement::len` は「要素数を`T`で表した値」であり，実際の要素数そのものではない．
//!   そのため`len`同士の加算は`T`上で行われ，`u8`等ではwrapping，`ModInt<P>`では法`P`で折り返す．

use std::marker::PhantomData;

use crate::{
    math::modint::ModInt,
    ops::monoid::{CommutativeMonoid, Monoid},
};

/// 区間和を保持するノード
///
/// `value` に区間和，`len` に要素数を`T`で表した値を格納する．
/// [`OpRangeSum`] の要素型として使用される．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpRangeSumElement<T> {
    /// 区間和
    value: T,

    /// 要素数を`T`で表した値
    ///
    /// 実際の要素数そのものではなく，`T`上の加算で更新される．
    len: T,
}

impl<T> OpRangeSumElement<T> {
    /// 区間和と要素数(`T`表現)を直接指定して生成する．
    ///
    /// # Notes
    ///
    /// - `count` は実際の要素数そのものではなく，`T`上の値として扱われる．
    /// - 呼び出し側で`count`の妥当性を担保する必要があるため，通常は[`Self::leaf`]を優先する．
    #[inline]
    pub fn with_count(value: T, count: T) -> Self {
        Self { value, len: count }
    }

    /// 区間和を返す．
    #[inline]
    pub fn value(&self) -> T
    where
        T: Copy,
    {
        self.value
    }

    #[inline]
    pub fn len(&self) -> T
    where
        T: Copy,
    {
        self.len
    }

    /// 葉ノードを生成する．
    ///
    /// `len` を `1` に設定した要素を返す
    #[allow(private_bounds)]
    pub fn leaf(value: T) -> Self
    where
        T: HasOneValue,
    {
        Self { value, len: T::ONE }
    }
}

/// 区間和演算
///
/// 二項演算として区間和の結合(各フィールドの加算)を，
/// 単位元としてゼロ値を持つ可換モノイド．
/// 標準のプリミティブ整数型と[`ModInt`]に対応する．
///
/// # Notes
///
/// - 整数型では wrapping 演算を用いるため，オーバーフロー時は折り返す．
/// - `OpRangeSumElement::len` は「要素数を`T`で表した値」であり，演算は`T`上で行われる．
#[derive(Default, Clone)]
pub struct OpRangeSum<T>(PhantomData<T>);

impl<T> Monoid for OpRangeSum<T>
where
    T: Copy + HasZeroValue + HasAdd,
{
    type Element = OpRangeSumElement<T>;

    #[inline]
    fn id(&self) -> Self::Element {
        OpRangeSumElement {
            value: T::ZERO,
            len: T::ZERO,
        }
    }

    #[inline]
    fn op(&self, lhs: &Self::Element, rhs: &Self::Element) -> Self::Element {
        OpRangeSumElement {
            value: lhs.value.add(rhs.value),
            len: lhs.len.add(rhs.len),
        }
    }
}

impl<T> CommutativeMonoid for OpRangeSum<T> where T: Copy + HasZeroValue + HasAdd {}

/// 型固有のゼロ値を提供するトレイト．
trait HasZeroValue {
    const ZERO: Self;
}

/// 型固有の1値を提供するトレイト．
trait HasOneValue {
    const ONE: Self;
}

/// 型固有の加算を提供するトレイト．
trait HasAdd {
    fn add(self, rhs: Self) -> Self;
}

macro_rules! impl_op_range_sum_traits_inner {
    ($ty: ty) => {
        impl HasZeroValue for $ty {
            const ZERO: Self = 0;
        }

        impl HasOneValue for $ty {
            const ONE: Self = 1;
        }

        impl HasAdd for $ty {
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }
    };
}

macro_rules! impl_range_sum_traits {
    ($($ty: ty),* $(,)?) => {
        $( impl_op_range_sum_traits_inner!($ty); )*
    };
}

impl_range_sum_traits! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl<const P: u64> HasZeroValue for ModInt<P> {
    const ZERO: Self = Self::new(0);
}

impl<const P: u64> HasOneValue for ModInt<P> {
    const ONE: Self = Self::new(1);
}

impl<const P: u64> HasAdd for ModInt<P> {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::monoid::{CommutativeMonoid, Monoid};

    fn assert_commutative_monoid<T: CommutativeMonoid>() {}

    fn e<T>(value: T, len: T) -> OpRangeSumElement<T> {
        OpRangeSumElement::with_count(value, len)
    }

    // ========== 要素型 ==========

    #[test]
    fn test_with_count_and_accessors_roundtrip() {
        let x = OpRangeSumElement::with_count(-1030i64, 7);
        assert_eq!(x.value(), -1030);
        assert_eq!(x.len(), 7);
    }

    // ========== 符号なし整数 (u64) ==========

    #[test]
    fn test_op_basic_unsigned() {
        let m = OpRangeSum::<u64>::default();
        assert_eq!(m.op(&e(74, 2), &e(33, 3)), e(107, 5));
        assert_eq!(m.op(&e(1030, 1), &e(7, 1)), e(1037, 2));
    }

    #[test]
    fn test_id_returns_zero_unsigned() {
        let m = OpRangeSum::<u64>::default();
        let id = m.id();
        assert_eq!(id.value, 0, "id.value = 0");
        assert_eq!(id.len, 0, "id.len = 0");
    }

    #[test]
    fn test_op_identity_unsigned() {
        let m = OpRangeSum::<u64>::default();
        let x = e(1030, 3);
        assert_eq!(m.op(&m.id(), &x), x, "id * x = x");
        assert_eq!(m.op(&x, &m.id()), x, "x * id = x");
    }

    #[test]
    fn test_op_commutativity_unsigned() {
        let m = OpRangeSum::<u64>::default();
        let a = e(3, 1);
        let b = e(1030, 4);
        assert_eq!(m.op(&a, &b), m.op(&b, &a), "a * b = b * a");
    }

    #[test]
    fn test_op_associativity_unsigned() {
        let m = OpRangeSum::<u64>::default();
        let (a, b, c) = (e(3, 1), e(1030, 4), e(7, 2));
        assert_eq!(
            m.op(&m.op(&a, &b), &c),
            m.op(&a, &m.op(&b, &c)),
            "(a * b) * c = a * (b * c)"
        );
    }

    #[test]
    fn test_op_boundary_unsigned() {
        let m = OpRangeSum::<u64>::default();
        assert_eq!(m.op(&e(0, 0), &e(0, 0)), e(0, 0), "0 + 0 = 0");
        assert_eq!(
            m.op(&e(u64::MAX, 1), &e(0, 0)),
            e(u64::MAX, 1),
            "MAX + 0 = MAX"
        );
        assert_eq!(
            m.op(&e(0, 0), &e(u64::MAX, 1)),
            e(u64::MAX, 1),
            "0 + MAX = MAX"
        );
        assert_eq!(
            m.op(&e(u64::MAX, 1), &e(1, 1)),
            e(0, 2),
            "MAX + 1 wraps to 0"
        );
        assert_eq!(
            m.op(&e(u64::MAX, 1), &e(u64::MAX, 1)),
            e(u64::MAX.wrapping_add(u64::MAX), 2),
            "MAX + MAX wraps"
        );
    }

    #[test]
    fn test_len_boundary_unsigned() {
        let m = OpRangeSum::<u8>::default();
        assert_eq!(
            m.op(&e(0, u8::MAX), &e(0, 1)),
            e(0, 0),
            "len: MAX + 1 wraps to 0"
        );
        assert_eq!(
            m.op(&e(0, u8::MAX), &e(0, u8::MAX)),
            e(0, u8::MAX.wrapping_add(u8::MAX)),
            "len: MAX + MAX wraps"
        );
    }

    // ========== 符号付き整数 (i64) ==========

    #[test]
    fn test_op_basic_signed() {
        let m = OpRangeSum::<i64>::default();
        assert_eq!(m.op(&e(22, 2), &e(-4, 1)), e(18, 3), "22 + (-4) = 18");
        assert_eq!(m.op(&e(-7, 1), &e(10, 1)), e(3, 2), "(-7) + 10 = 3");
        assert_eq!(m.op(&e(-8, 3), &e(-55, 2)), e(-63, 5), "(-8) + (-55) = -63");
    }

    #[test]
    fn test_op_boundary_signed() {
        let m = OpRangeSum::<i64>::default();
        assert_eq!(
            m.op(&e(i64::MIN, 1), &e(i64::MAX, 1)),
            e(-1, 2),
            "MIN + MAX = -1"
        );
        assert_eq!(
            m.op(&e(i64::MIN, 1), &e(-1, 1)),
            e(i64::MAX, 2),
            "MIN + (-1) wraps to MAX"
        );
        assert_eq!(
            m.op(&e(i64::MAX, 1), &e(1, 1)),
            e(i64::MIN, 2),
            "MAX + 1 wraps to MIN"
        );
        assert_eq!(
            m.op(&e(i64::MIN, 1), &e(i64::MIN, 1)),
            e(i64::MIN.wrapping_add(i64::MIN), 2),
            "MIN + MIN wraps"
        );
    }

    #[test]
    fn test_len_boundary_signed() {
        let m = OpRangeSum::<i8>::default();
        assert_eq!(
            m.op(&e(0, i8::MAX), &e(0, 1)),
            e(0, i8::MIN),
            "len: MAX + 1 wraps to MIN"
        );
        assert_eq!(
            m.op(&e(0, i8::MIN), &e(0, -1)),
            e(0, i8::MAX),
            "len: MIN + (-1) wraps to MAX"
        );
    }

    // ========== leaf コンストラクタ ==========

    #[test]
    fn test_leaf_constructor() {
        let leaf_i = OpRangeSumElement::leaf(42i64);
        assert_eq!(leaf_i.value, 42, "leaf value (i64)");
        assert_eq!(leaf_i.len, 1, "leaf len (i64)");

        let leaf_u = OpRangeSumElement::leaf(1030u64);
        assert_eq!(leaf_u.value, 1030, "leaf value (u64)");
        assert_eq!(leaf_u.len, 1, "leaf len (u64)");

        let leaf_m = OpRangeSumElement::leaf(ModInt::<998_244_353>::new(1030));
        assert_eq!(leaf_m.value, ModInt::new(1030), "leaf value (ModInt)");
        assert_eq!(leaf_m.len, ModInt::new(1), "leaf len (ModInt)");
    }

    #[test]
    fn test_leaf_op_combination() {
        let m = OpRangeSum::<i64>::default();
        let leaves = [3, 1, 4, 1, 5].map(OpRangeSumElement::leaf);
        let result = leaves.iter().fold(m.id(), |acc, x| m.op(&acc, x));
        assert_eq!(result.value, 14, "sum of values");
        assert_eq!(result.len, 5, "count of leaves");
    }

    // ========== 全探索 (u8) ==========

    #[test]
    fn test_op_exhaustive_u8() {
        assert_commutative_monoid::<OpRangeSum<u8>>();
        let m = OpRangeSum::<u8>::default();
        // 単位元性(代表 len で検証)
        for a in 0..=u8::MAX {
            let x = e(a, 1);
            assert_eq!(m.op(&m.id(), &x), x, "id * x = x for value={a}");
            assert_eq!(m.op(&x, &m.id()), x, "x * id = x for value={a}");
        }
        // 全 value ペア × len=1 固定
        for a in 0..=u8::MAX {
            for b in 0..=u8::MAX {
                assert_eq!(
                    m.op(&e(a, 1), &e(b, 1)),
                    e(a.wrapping_add(b), 2),
                    "{a} + {b}"
                );
            }
        }
    }

    // ========== ModInt ==========

    #[test]
    fn test_op_basic_modint() {
        let m = OpRangeSum::<ModInt<998_244_353>>::default();
        let a = e(ModInt::new(3), ModInt::new(1));
        let b = e(ModInt::new(5), ModInt::new(1));
        assert_eq!(
            m.op(&a, &b),
            e(ModInt::new(8), ModInt::new(2)),
            "3 + 5 = 8 (mod P)"
        );
        // mod 折り返し
        let x = e(ModInt::new(998_244_350), ModInt::new(1));
        let y = e(ModInt::new(10), ModInt::new(1));
        assert_eq!(
            m.op(&x, &y),
            e(ModInt::new(7), ModInt::new(2)),
            "998244350 + 10 = 7 (mod 998244353)"
        );
    }

    #[test]
    fn test_len_boundary_modint() {
        let m = OpRangeSum::<ModInt<7>>::default();
        assert_eq!(
            m.op(
                &e(ModInt::new(0), ModInt::new(6)),
                &e(ModInt::new(0), ModInt::new(1))
            ),
            e(ModInt::new(0), ModInt::new(0)),
            "len: 6 + 1 = 0 (mod 7)"
        );
    }

    #[test]
    fn test_op_monoid_laws_signed_and_modint() {
        {
            let m = OpRangeSum::<i64>::default();
            let (a, b, c) = (e(-3, 1), e(1030, 4), e(7, 2));
            assert_eq!(m.op(&m.id(), &a), a, "id * a = a (i64)");
            assert_eq!(m.op(&a, &m.id()), a, "a * id = a (i64)");
            assert_eq!(m.op(&a, &b), m.op(&b, &a), "a * b = b * a (i64)");
            assert_eq!(
                m.op(&m.op(&a, &b), &c),
                m.op(&a, &m.op(&b, &c)),
                "(a * b) * c = a * (b * c) (i64)"
            );
        }
        {
            let m = OpRangeSum::<ModInt<998_244_353>>::default();
            let (a, b, c) = (
                e(ModInt::new(1030), ModInt::new(1)),
                e(ModInt::new(998_244_000), ModInt::new(2)),
                e(ModInt::new(7), ModInt::new(3)),
            );
            assert_eq!(m.op(&m.id(), &a), a, "id * a = a (ModInt)");
            assert_eq!(m.op(&a, &m.id()), a, "a * id = a (ModInt)");
            assert_eq!(m.op(&a, &b), m.op(&b, &a), "a * b = b * a (ModInt)");
            assert_eq!(
                m.op(&m.op(&a, &b), &c),
                m.op(&a, &m.op(&b, &c)),
                "(a * b) * c = a * (b * c) (ModInt)"
            );
        }
    }

    // ========== 全型スモークテスト ==========

    #[test]
    fn test_op_smoke_all_integer_types() {
        macro_rules! test {
            ($ty: ty) => {
                assert_commutative_monoid::<OpRangeSum<$ty>>();
                let m = OpRangeSum::<$ty>::default();
                assert_eq!(
                    m.op(&e(3 as $ty, 1 as $ty), &e(5 as $ty, 1 as $ty)),
                    e(8 as $ty, 2 as $ty),
                    "op for {}",
                    stringify!($ty)
                );
                let id = m.id();
                assert_eq!(id.value, 0 as $ty, "id.value for {}", stringify!($ty));
                assert_eq!(id.len, 0 as $ty, "id.len for {}", stringify!($ty));
            };
        }

        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
        test!(isize);
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(usize);

        // ModInt
        assert_commutative_monoid::<OpRangeSum<ModInt<998_244_353>>>();
        let m = OpRangeSum::<ModInt<998_244_353>>::default();
        assert_eq!(
            m.op(
                &e(ModInt::new(3), ModInt::new(1)),
                &e(ModInt::new(5), ModInt::new(1))
            ),
            e(ModInt::new(8), ModInt::new(2)),
            "op for ModInt"
        );
        let id = m.id();
        assert_eq!(id.value, ModInt::new(0), "id.value for ModInt");
        assert_eq!(id.len, ModInt::new(0), "id.len for ModInt");
    }
}
