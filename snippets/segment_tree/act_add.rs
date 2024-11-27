/// Range Add Query
#[derive(Default)]
pub struct ActAdd;

impl Monoid for ActAdd {
    // TODO: 型チェック
    type Value = i64;

    fn identity(&self) -> Self::Value {
        0
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        x + y
    }
}

impl Action<M> for ActAdd {
    fn act(&self, f: &<Self as Monoid>::Value, x: &<M as Monoid>::Value) -> <M as Monoid>::Value {
        x + f
    }
}
