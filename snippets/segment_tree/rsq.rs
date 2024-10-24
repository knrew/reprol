/// Range Sum Query
#[derive(Default)]
pub struct RsqMonoid;

impl Monoid for RsqMonoid {
    // TODO: 型チェック
    type Value = i64;

    fn identity(&self) -> Self::Value {
        0
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        x + y
    }
}
