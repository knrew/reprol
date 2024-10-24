#[derive(Default)]
struct M;

impl Monoid for M {
    type Value = todo!();

    fn identity(&self) -> Self::Value {
        todo!()
    }

    fn op(&self, x: &Self::Value, y: &Self::Value) -> Self::Value {
        todo!()
    }
}
