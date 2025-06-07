//! change_min / change_max
//!
//! 最小値・最大値の更新に関するトレイト．
//!
//! - [`ChangeMinMax`] : 最小値・最大値更新．
//! - [`ChangeMinMaxOrInsert`] : `Option<T>`に対して，代入or最小値・最大値更新．
//!
//! # 使用例
//! ```
//! use reprol::change_min_max::ChangeMinMax;
//! let mut a = 10;
//! assert!(a.change_min(5));
//! assert!(!a.change_min(7));
//! assert!(a.change_max(12));
//! assert!(!a.change_max(9));
//!```
//!
//!```
//! use reprol::change_min_max::ChangeMinMaxOrInsert;
//! let mut a: Option<i32> = None;
//! assert!(a.change_min_or_insert(10));
//! assert!(!a.change_min_or_insert(15));
//! assert!(a.change_min_or_insert(5));
//! assert!(a.change_max_or_insert(20));
//! assert!(!a.change_max_or_insert(18));
//! ```

/// 最小値・最大値更新を行うためのトレイト．
pub trait ChangeMinMax {
    /// `rhs`の値が`self`より小さい場合，`self`を`rhs`に変更し，trueを返す．
    /// そうでない場合は，何も変更せずにfalseを返す．
    fn change_min(&mut self, rhs: Self) -> bool;

    /// `rhs`の値が`self`より大きい場合，`self`を`rhs`に変更し，trueを返す．
    /// そうでない場合は，何も変更せずにfalseを返す．
    fn change_max(&mut self, rhs: Self) -> bool;
}

impl<T> ChangeMinMax for T
where
    T: PartialOrd,
{
    #[inline]
    fn change_min(&mut self, rhs: T) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }

    #[inline]
    fn change_max(&mut self, rhs: T) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

/// `Option<T>`に対して，代入or最小値・最大値更新を行うためのトレイト．
pub trait ChangeMinMaxOrInsert {
    type Item;

    /// `self`が`None`である場合，または，`self`が`Some(lhs)`で`rhs`の値が`lhs`より小さい場合，`self`を`Some(rhs)`に変更し，trueを返す．
    /// そうでない場合は，何も変更せずにfalseを返す．
    fn change_min_or_insert(&mut self, rhs: Self::Item) -> bool;

    /// `self`が`None`である場合，または，`self`が`Some(lhs)`で`rhs`の値が`lhs`より大きい場合，`self`を`Some(rhs)`に変更し，trueを返す．
    /// そうでない場合は，何も変更せずにfalseを返す．
    fn change_max_or_insert(&mut self, lhs: Self::Item) -> bool;
}

impl<T> ChangeMinMaxOrInsert for Option<T>
where
    T: PartialOrd,
{
    type Item = T;

    #[inline]
    fn change_min_or_insert(&mut self, rhs: Self::Item) -> bool {
        match self {
            Some(lhs) if *lhs <= rhs => false,
            _ => {
                *self = Some(rhs);
                true
            }
        }
    }

    #[inline]
    fn change_max_or_insert(&mut self, rhs: Self::Item) -> bool {
        match self {
            Some(lhs) if *lhs >= rhs => false,
            _ => {
                *self = Some(rhs);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_min() {
        let mut x = 5;
        let f = x.change_min(10);
        assert!(!f);
        assert_eq!(x, 5);
        let f = x.change_min(3);
        assert!(f);
        assert_eq!(x, 3);
        let f = x.change_min(3);
        assert!(!f);
        assert_eq!(x, 3);
    }

    #[test]
    fn test_change_max() {
        let mut x = 5;
        let f = x.change_max(2);
        assert!(!f);
        assert_eq!(x, 5);
        let f = x.change_max(10);
        assert!(f);
        assert_eq!(x, 10);
        let f = x.change_max(10);
        assert!(!f);
        assert_eq!(x, 10);
    }

    #[test]
    fn test_change_min_or_insert() {
        let mut x = None;
        let f = x.change_min_or_insert(10);
        assert!(f);
        assert_eq!(x, Some(10));
        let f = x.change_min_or_insert(3);
        assert!(f);
        assert_eq!(x, Some(3));
        let f = x.change_min_or_insert(20);
        assert!(!f);
        assert_eq!(x, Some(3));
    }

    #[test]
    fn test_change_max_or_insert() {
        let mut x = None;
        let f = x.change_max_or_insert(10);
        assert!(f);
        assert_eq!(x, Some(10));
        let f = x.change_max_or_insert(20);
        assert!(f);
        assert_eq!(x, Some(20));
        let f = x.change_max_or_insert(2);
        assert!(!f);
        assert_eq!(x, Some(20));
    }
}
