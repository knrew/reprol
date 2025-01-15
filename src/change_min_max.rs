pub trait ChangeMinMax {
    fn change_min(&mut self, value: Self) -> bool;
    fn change_max(&mut self, value: Self) -> bool;
}

impl<T> ChangeMinMax for T
where
    T: PartialOrd,
{
    fn change_min(&mut self, value: T) -> bool {
        if value < *self {
            *self = value;
            true
        } else {
            false
        }
    }

    fn change_max(&mut self, value: T) -> bool {
        if *self < value {
            *self = value;
            true
        } else {
            false
        }
    }
}
