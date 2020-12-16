# Rusty Algorithms and Data Structures

![Continuous Integration](https://github.com/TianyiShi2001/Algorithms/workflows/CI/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/TianyiShi2001/Algorithms/badge.svg?branch=main)](https://coveralls.io/github/TianyiShi2001/Algorithms?branch=main)
![lines of code](https://img.shields.io/badge/lines%20of%20code-5733-blue)

This repository presents Rust implementation of common algorithms and data structures, most of which are based on William Fiset's Java implementation: https://github.com/williamfiset/Algorithms . I highly recommend [his YouTube channel](https://www.youtube.com/user/purpongie), where he explains many of these algorithms in detail using illustrations, animations and pseudocode.

In addition to implementing W. Fiset's algorithms, I also add original content that might be helpful, such as solutions of classical puzzles e.g. N-Queens and Sudoku.

## Usage

The implementation details are explained in comments and docs and the example usage is implied in unit tests. To run tests:

```
cargo test
```

These algorithms and data structures are not designed for production usage, but might be directly applicable in competitve programming.

## Recommended Environment

- Editor: Visual Studio Code
  - Extension: [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)

This simple setup provides most features a decent IDE would provide (importantly, jump to definition and type labelling)

<!-- ## Rusticity

This is not a verbatim translation of W. Fiset's Java implementation. Instead, I try to make the code idiomatic in Rust, according to these rules:

### Avoid Long Names Using `mod`s

For example, perfer

```
crate::algo::graph::bfs::adjacency_list_iterative::fast_deque
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

### Use `Option<T>` to Represent Nullable Values

Genrerally, `Option::None` is an idiomatic representation of `null`.  This makes the code work better with the standard library and cause less surprises. -->