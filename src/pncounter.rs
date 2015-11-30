use gcounter::GCounter;
use counter::Counter;
use crdt::Crdt;

#[derive(Debug)]
pub struct PnCounter {
    p: GCounter,
    n: GCounter
}

impl PnCounter {
    pub fn new() -> PnCounter {
        PnCounter {
            p: GCounter::new(),
            n: GCounter::new()
        }
    }

    /// Return the value by merging both the add and sub
    /// together.
    pub fn to_gcounter(&self) -> GCounter {
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

impl Counter for PnCounter {
    fn incr(&mut self, node: u32, delta: u64) {
        self.p.incr(node, delta);
    }

    fn decr(&mut self, node: u32, delta: u64) {
        self.n.incr(node, delta);
    }

    /// XXX: We don't support negative integers yet.
    fn value(&self) -> u64 {
        self.p.value() - self.n.value()
    }
}

impl Crdt for PnCounter {
    fn merge(&self, other: &PnCounter) -> PnCounter {
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
