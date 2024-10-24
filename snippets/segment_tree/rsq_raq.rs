#[derive(Default)]
pub struct M;

impl Monoid for M {
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
pub struct A;

impl Monoid for A {
    // TODO: 型チェック
    type Value = i64;

    fn identity(&self) -> Self::Value {
        0
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        x + y
    }
}

impl Action<M> for A {
    fn act(&self, f: &<Self as Monoid>::Value, x: &<M as Monoid>::Value) -> <M as Monoid>::Value {
        (x.0 + f * x.1, x.1)
    }
}
