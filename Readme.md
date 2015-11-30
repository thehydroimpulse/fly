# Fly

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

The second argument is the delta, how much we want to increment the counter.

We can try and create a second counter from another replica:

```rust
let mut replica = GCounter::new();
```

And increment it with a different replica ID:

```rust
replica.incr(2, 1);
```

Now, the goal of CRDTs is to eventually merge the state across all replicas. We can use the merge function to achieve that.

```rust
let merged = views.merge(&replica);
```

This returns a new `GCounter` data type with the merged data from two different replicas.

### Syncing

Fly doesn't handle any synchronizations between real replicas/nodes. You'll have to handle communicating between nodes. When you retrieve new state from a replica, you'll create a new CRDT (e.g., `GCounter`) and merge locally.


# License

MIT
