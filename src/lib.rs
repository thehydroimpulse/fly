//! LRW Register (Last-write win)
//! G-Counter
//! PN-Counter
//! G-Set
//! Two-Phase Set

mod gcounter;
mod pncounter;
mod counter;
mod crdt;

pub use gcounter::GCounter;
pub use crdt::Crdt;
pub use counter::Counter;

#[cfg(test)]
mod tests {
    use super::*;
}
