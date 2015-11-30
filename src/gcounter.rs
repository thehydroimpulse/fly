use std::collections::HashMap;
use crdt::Crdt;
use counter::Counter;

#[derive(Debug)]
pub struct GCounter {
    pub data: HashMap<u32, u64>
}

impl GCounter {
    pub fn new() -> GCounter {
        GCounter { data: HashMap::new() }
    }
}

impl Counter for GCounter {
    fn incr(&mut self, node: u32, delta: u64) {
        if self.data.contains_key(&node) {
            if let Some(key) = self.data.get_mut(&node) {
                *key += delta;
            }
        } else {
            self.data.insert(node, delta);
        }
    }

    fn value(&self) -> u64 {
        let mut val = 0;

        for (k, v) in self.data.iter() {
            val += *v;
        }

        val
    }
}

impl Crdt for GCounter {
    fn merge(&self, other: &GCounter) -> GCounter {
        let mut cloned = other.data.clone();

        for (node, delta) in self.data.iter() {
            let found = match cloned.get_mut(&node) {
                Some(e) => if *delta > *e {
                    *e = *delta;
                    false
                } else { false },
                None => true
            };

            if found {
                cloned.insert(*node, *delta);
            }
        }

        GCounter { data: cloned }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crdt::Crdt;
    use counter::Counter;

    #[test]
    fn set_g_counter() {
        let mut a = GCounter::new();
        a.incr(1, 5);

        assert_eq!(a.data.get(&1), Some(&5));
    }

    #[test]
    fn incr_g_counter() {
        let mut a = GCounter::new();
        a.incr(1, 5);
        a.incr(1, 1);

        assert_eq!(a.data.get(&1), Some(&6));
    }

    #[test]
    #[should_panic]
    fn decr_g_counter() {
        let mut a = GCounter::new();
        a.decr(1, 5);
    }

    #[test]
    fn merge_g_counter() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();

        a.incr(1, 2);
        b.incr(2, 3);

        let merged = a.merge(&b);

        assert_eq!(merged.data.get(&1), Some(&2));
        assert_eq!(merged.data.get(&2), Some(&3));
    }

    #[test]
    fn merge_g_counter_conflict() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();

        a.incr(1, 2);
        b.incr(1, 3);

        let merged = a.merge(&b);

        assert_eq!(merged.data.get(&1), Some(&3));
    }

    #[test]
    fn value() {
        let mut a = GCounter::new();

        a.incr(1, 2);
        a.incr(2, 2);
        a.incr(3, 5);

        assert_eq!(a.value(), 9);
    }
}
