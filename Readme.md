# Fly [![Build Status](https://travis-ci.org/thehydroimpulse/fly.svg)](https://travis-ci.org/thehydroimpulse/fly)

Convergent Replicated Data Types in Rust.

A set of data types built for eventually consistent systems.

## Data Types

Current data types are:

* GCounter: A grow only counter.
* PnCounter: A counter supporting increment and decrement operations.
* GSet: A grow only set.
* TwoPSet: A set supporting insert and remove operations.

## Getting Started

Add the crate to your `Cargo.toml`

```toml
[dependencies]
fly = "0.1.0"
```

And import the crate in your `lib.rs` file.

```rust
extern crate fly;
```

## GCounter

G-Counters are the building blocks for eventually consistent counters. They can only be incremented, never decremented.

```rust
// Let's create a new GCounter
let mut a = GCounter::new();
let mut b = GCounter::new();

a.incr(1, 2);
b.incr(2, 9);

// Converge onto an eventual counter. This returns a new `GCounter`.
let converged = a.merge(&b);
```

The first argument to `incr` is the current node or replica, which is a generic data type.

The second argument is the delta &mdash; how much we want to increment the counter.

The node id is bound to the following trait constraints:

```rust
Eq + Hash + Copy + Clone
```

Example replica ids:

```rust
counter.incr("node1".to_string(), 5);
counter.incr(Uuid::new_v4(), 4);
```

## PnCounter

A PN-Counter uses two `GCounter`s to build a full counter, with the ability to decrement. One of the `GCounter`s is used for increments and the second is used for decrements.

```rust
let mut a = PnCounter::new();
let mut b = PnCounter::new();

a.incr(1, 3);
a.incr(3, 2);
b.decr(2, 1);

let converged = a.merge(&b);
```

## GSet

## TwoPSet

# License

MIT
