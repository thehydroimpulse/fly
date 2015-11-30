use std::collections::HashSet;
use std::hash::Hash;
use crdt::Crdt;
use std::fmt::Debug;

pub struct GSet<E> {
    data: HashSet<E>,
}

impl<E> GSet<E>
    where E: Eq + Hash + Clone
{
    pub fn new() -> GSet<E> {
        GSet { data: HashSet::new() }
    }

    pub fn add(&mut self, el: E) {
        self.data.insert(el);
    }
}

impl<E> Crdt for GSet<E>
    where E: Eq + Hash + Clone
{
    fn merge(&self, other: &GSet<E>) -> GSet<E> {
        let mut set = HashSet::new();

        for el in self.data.union(&other.data) {
            set.insert(el.clone());
        }

        GSet { data: set }
    }
}

pub struct TwoPSet<E> {
    p: HashSet<E>,
    n: HashSet<E>,
}

impl<E> TwoPSet<E>
    where E: Eq + Hash + Clone
{
    pub fn new() -> TwoPSet<E> {
        TwoPSet {
            p: HashSet::new(),
            n: HashSet::new()
        }
    }

    pub fn add(&mut self, key: E) {
        self.p.insert(key);
    }

    pub fn remove(&mut self, key: E) {
        self.n.insert(key);
    }
}

impl<E> Crdt for TwoPSet<E>
    where E: Eq + Hash + Clone
{
    fn merge(&self, other: &TwoPSet<E>) -> TwoPSet<E> {
        let mut pset = HashSet::new();
        let mut nset = HashSet::new();

        // Merge the two N sets
        for n in self.n.union(&other.n) {
            nset.insert(n.clone());
        }

        // Merge the two P sets
        for p in self.p.union(&other.p) {
            // Only merge the set if it's not in the converged N set.
            if !nset.contains(p) {
                pset.insert(p.clone());
            }
        }

        TwoPSet {
            p: pset,
            n: HashSet::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crdt::Crdt;

    #[test]
    fn merge_gset() {
        let mut a = GSet::new();
        let mut b = GSet::new();

        a.add("foobar");
        b.add("bigya");

        let c = a.merge(&b);

        assert_eq!(c.data.len(), 2);
        assert_eq!(c.data.contains("foobar"), true);
        assert_eq!(c.data.contains("bigya"), true);
    }

    #[test]
    fn merge_twopset() {
        let mut a = TwoPSet::new();
        let mut b = TwoPSet::new();

        a.add("foobar");
        b.remove("foobar");

        let c = a.merge(&b);

        assert_eq!(c.p.len(), 0);
        assert_eq!(c.n.len(), 0);
    }

    #[test]
    fn merge_twopset_remove_priority() {

        let mut a = TwoPSet::new();
        let mut b = TwoPSet::new();

        a.add("foobar");
        b.remove("foobar");
        b.add("foobar");

        let c = a.merge(&b);

        assert_eq!(c.p.len(), 0);
        assert_eq!(c.n.len(), 0);
    }

    #[test]
    fn merge_twopset_remove_add() {

        let mut a = TwoPSet::new();
        let mut b = TwoPSet::new();

        a.add("foobar");
        b.remove("foobar");
        b.add("foobar1");

        let c = a.merge(&b);

        assert_eq!(c.p.len(), 1);
        assert_eq!(c.n.len(), 0);
    }
}
