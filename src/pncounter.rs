use gcounter::GCounter;
use counter::Counter;
use crdt::Crdt;
use std::hash::Hash;

pub struct PnCounter<N> {
    p: GCounter<N>,
    n: GCounter<N>
}

impl<N> PnCounter<N>
    where N: Eq + Hash + Clone + Copy
{
    pub fn new() -> PnCounter<N> {
        PnCounter {
            p: GCounter::new(),
            n: GCounter::new()
        }
    }

    /// Return the value by merging both the add and sub
    /// together.
    pub fn to_gcounter(&self) -> GCounter<N> {
        let mut counter = GCounter::new();

        for (k, v) in self.p.data.iter() {
            if self.n.data.contains_key(k) {
                let val = self.n.data.get(k).unwrap();
                counter.incr(*k, *v - val);
            } else {
                counter.incr(*k, *v);
            }
        }

        counter
    }
}

impl<N> Counter<N> for PnCounter<N>
    where N: Eq + Hash + Clone + Copy
{
    fn incr(&mut self, node: N, delta: u64) {
        self.p.incr(node, delta);
    }

    fn decr(&mut self, node: N, delta: u64) {
        self.n.incr(node, delta);
    }

    /// XXX: We don't support negative integers yet.
    fn value(&self) -> u64 {
        self.p.value() - self.n.value()
    }
}

impl<N> Crdt for PnCounter<N>
    where N: Eq + Hash + Clone + Copy
{
    fn merge(&self, other: &PnCounter<N>) -> PnCounter<N> {
        // Merge each g-counter individually.
        PnCounter {
            p: self.p.merge(&other.p),
            n: self.n.merge(&other.n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crdt::Crdt;
    use counter::Counter;
    use gcounter::GCounter;

    #[test]
    fn get_val() {
        let mut a = PnCounter::new();

        a.incr(1, 5);
        a.decr(1, 3);

        let b = a.to_gcounter();

        assert_eq!(b.data.get(&1), Some(&2));
    }

    #[test]
    fn merge() {
        let mut a = PnCounter::new();
        let mut b = PnCounter::new();

        a.incr(1, 5);
        b.decr(1, 3);

        let c = a.merge(&b);

        assert_eq!(c.p.data.get(&1), Some(&5));
        assert_eq!(c.n.data.get(&1), Some(&3));
    }

    #[test]
    fn merge_and_get_gcounter() {
        let mut a = PnCounter::new();
        let mut b = PnCounter::new();

        a.incr(1, 5);
        b.decr(1, 3);

        let c = a.merge(&b).to_gcounter();

        assert_eq!(c.data.get(&1), Some(&2));
    }

    #[test]
    fn merge_and_get_value() {
        let mut a = PnCounter::new();
        let mut b = PnCounter::new();

        a.incr(1, 5);
        b.decr(1, 3);

        let c = a.merge(&b).value();

        assert_eq!(c, 2);
    }
}
