pub trait ChangeMinMax {
    fn change_min(&mut self, rhs: Self) -> bool;
    fn change_max(&mut self, rhs: Self) -> bool;
}

impl<T> ChangeMinMax for T
where
    T: PartialOrd,
{
    fn change_min(&mut self, rhs: T) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }

    fn change_max(&mut self, rhs: T) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

/// Option型に対してNoneなら代入，Someなら最小値(最大値)を更新する
pub trait ChangeMinMaxOrSet {
    type Item;
    fn change_min_or_set(&mut self, value: Self::Item) -> bool;
    fn change_max_or_set(&mut self, value: Self::Item) -> bool;
}

impl<T> ChangeMinMaxOrSet for Option<T>
where
    T: PartialOrd,
{
    type Item = T;

    fn change_min_or_set(&mut self, rhs: Self::Item) -> bool {
        match self {
            Some(lhs) if *lhs <= rhs => false,
            _ => {
                *self = Some(rhs);
                true
            }
        }
    }

    fn change_max_or_set(&mut self, rhs: Self::Item) -> bool {
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
    use super::{ChangeMinMax, ChangeMinMaxOrSet};

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
    fn test_change_min_or_set() {
        let mut x = None;
        let f = x.change_min_or_set(10);
        assert!(f);
        assert_eq!(x, Some(10));
        let f = x.change_min_or_set(3);
        assert!(f);
        assert_eq!(x, Some(3));
        let f = x.change_min_or_set(20);
        assert!(!f);
        assert_eq!(x, Some(3));
    }

    #[test]
    fn test_change_max_or_set() {
        let mut x = None;
        let f = x.change_max_or_set(10);
        assert!(f);
        assert_eq!(x, Some(10));
        let f = x.change_max_or_set(20);
        assert!(f);
        assert_eq!(x, Some(20));
        let f = x.change_max_or_set(2);
        assert!(!f);
        assert_eq!(x, Some(20));
    }
}
