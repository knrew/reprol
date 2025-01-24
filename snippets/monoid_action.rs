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

#[derive(Default)]
struct A;

impl Monoid for A {
    type Value = todo!();

    fn identity(&self) -> Self::Value {
        todo!()
    }

    // NOTE: gが後に作用する．g(f(x))
    fn op(&self, g: &Self::Value, f: &Self::Value) -> Self::Value {
        todo!()
    }
}

impl Action<M> for A {
    fn act(&self, f: &Self::Value, x: &<M as Monoid>::Value) -> <M as Monoid>::Value {
        todo!()
    }
}
