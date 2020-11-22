# Rusty Implementations of William Fiset's Algorithms

This repository presents the Rust implementation of the provided Algorithms and data structures from William Fiset at: https://github.com/williamfiset/Algorithms

I highly recommend [his YouTube channel](https://www.youtube.com/user/purpongie), where he explains many of these algorithms in detail.

## Usage

The implementation details are explained in comments and docs and the example usage is implied in unit tests. To run tests:

```
cargo test
```

## Recommended Environment

- Editor: Visual Studio Code
  - Extension: [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)

This simple setup provides most features a decent IDE would provide (importantly, jump to definition and type labelling)

## Rusticity

This is not a verbatim translation of the original implementation in Java. Instead, I try to make the code idiomatic in Rust, according to these rules

### Avoid Long Names Using `mod`s

For example, perfer

```
crate::graph::bfs::adjacency_list_iterative::fast_deque
```

over

```
com.williamfiset.algorithms.graphtheory.BreadthFirstSearchAdjacencyListIterativeFastQueue
```

### Custom Data Structures Have Unsurprising Method Names and Behaviour

Follow the conventions of `std` types as much as possible.

For example, when implementing a `Queue`, prefer

```rust
pub fn push_back(&mut self, value: T);
pub fn pop_front(&mut self) -> Option<T>;
```

over

```rust
pub fn enqueue(&mut self, value: T);
pub fn dequeue(&mut self) -> T;
// or
pub fn offer(&mut self, value: T);
pub fn poll(&mut self) -> T;
```

### Use `Option<T>` to Represent `Nullable Values

Genrerally, `Option::None` is an idiomatic representation of `null`.  This makes the code work better with the standard library and cause less surprises.