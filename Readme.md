# Fly [![Build Status](https://travis-ci.org/thehydroimpulse/fly.svg)](https://travis-ci.org/thehydroimpulse/fly)

Convergent Replicated Data Types in Rust.

A set of data types built for eventually consistent systems. Currently, only the G-Counter and PN-Counter data types are implemented.

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

## G-Counter

G-Counters are the building blocks for eventually consistent counters. They can only be incremented, never decremented.

```rust
// Let's create a new GCounter to track page views.
let mut views = GCounter::new();
```

We can then increment it:

```rust
views.incr(1, 1);
```

The first argument is the current node or replica as a `u32` data type. Fly doesn't have any requirements as to what this value is derived from, it's only important that it's use is consistent (i.e., a replica uses the same id everywhere).

The second argument is the delta &mdash; how much we want to increment the counter.

### Merging

Given two counters from two theoretical replicas, we want to converge into a single counter.

```rust
let mut a = GCounter::new();
let mut b = GCounter::new();

a.incr(1, 2);
b.incr(2, 9);
```

We're creating two different counters to eventually converge on with the merge function.

```rust
let converged = a.merge(&b);
```

The `merge` method returns a *new* `GCounter` with the converged data.

We can verify the merge by calling the `value` method.

```rust
assert_eq!(converged.value(), 11); // 'true'
```

## PN-Counter

A PN-Counter uses two `GCounter`s to build a full counter, with the ability to decrement. One of the `GCounter`s is used for increments and the second is used for decrements.

```rust
let mut online = PnCounter::new();

online.incr(1, 1);
online.decr(2, 1);
online.incr(3, 3);
```

### Merge

Merging works exactly like before, but we're merging the two internal `GCounter`s.

```rust
let mut a = PnCounter::new();
let mut b = PnCounter::new();

a.incr(1, 3);
a.incr(3, 2);
b.decr(2, 1);

let converged = a.merge(&b);
```

We can expect the final count to be equal to `4`, after a merge.

```rust
assert_eq!(converted.value(), 4); // 'true'
```

# License

MIT
