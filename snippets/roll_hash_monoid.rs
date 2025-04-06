#[derive(Default)]
struct Op<const P: u64>;

impl<const P: u64> Monoid for Op<P> {
    type Value = (u64, u64);

    fn identity(&self) -> Self::Value {
        (0, 1)
    }

    fn op(&self, (hx, bx): &Self::Value, (hy, by): &Self::Value) -> Self::Value {
        ((hx * by + hy) % P, (bx * by) % P)
    }
}
