pub trait ChangeMinMax {
    fn change_min(&mut self, value: Self) -> bool;
    fn change_max(&mut self, value: Self) -> bool;
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

    fn change_min_or_set(&mut self, rhs: T) -> bool {
        match self {
            Some(current) if *current < rhs => false,
            _ => {
                *self = Some(rhs);
                true
            }
        }
    }

    fn change_max_or_set(&mut self, rhs: Self::Item) -> bool {
        match self {
            Some(current) if *current > rhs => false,
            _ => {
                *self = Some(rhs);
                true
            }
        }
    }
}
