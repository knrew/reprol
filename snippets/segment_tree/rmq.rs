/// Range Minimum Query
#[derive(Default)]
pub struct RmqMonoid;

impl Monoid for RmqMonoid {
    // 型チェック
    type Value = i64;

    fn identity(&self) -> Self::Value {
        i64::MAX
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        *x.min(y)
    }
}
