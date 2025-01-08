pub trait ChangeMinMax {
    fn change_min(&mut self, value: Self) -> bool;
    fn change_max(&mut self, value: Self) -> bool;
}

impl<T: PartialOrd> ChangeMinMax for T {
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
