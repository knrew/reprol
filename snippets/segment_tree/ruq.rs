/// Range Update Query(Range Set Query)
#[derive(Default)]
pub struct RuqAction;

impl Monoid for RuqAction {
    // TODO: 型チェック
    type Value = i64;

    fn identity(&self) -> Self::Value {
        i64::MAX
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        *if x == &self.identity() { y } else { x }
    }
}

impl Action<M> for RuqAction {
    fn act(&self, f: &<Self as Monoid>::Value, x: &<M as Monoid>::Value) -> <M as Monoid>::Value {
        *if f == &self.identity() { x } else { f }
    }
}
