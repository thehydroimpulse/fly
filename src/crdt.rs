pub trait Crdt {
    fn merge(&self, other: &Self) -> Self;
}
