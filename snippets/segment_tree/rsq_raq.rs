/// Range Sum Query and Range Add Query

#[derive(Default)]
pub struct OpAdd;

impl Monoid for OpAdd {
    // TODO: 型チェック
    type Value = (i64, i64);

    fn identity(&self) -> Self::Value {
        (0, 0)
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        (x.0 + y.0, x.1 + y.1)
    }
}

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

impl Action<OpAdd> for ActAdd {
    fn act(
        &self,
        f: &<Self as Monoid>::Value,
        x: &<OpAdd as Monoid>::Value,
    ) -> <OpAdd as Monoid>::Value {
        (x.0 + f * x.1, x.1)
    }
}
