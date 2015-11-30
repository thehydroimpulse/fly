pub trait Counter<N> {
    fn incr(&mut self, node: N, delta: u64);
    fn decr(&mut self, node: N, delta: u64) {
        panic!("The decrement operation is not supported
                on the current Counter.");
    }

    fn value(&self) -> u64;
}
