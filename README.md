# Rusty Implementations of William Fiset's Algorithms

This repository presents the Rust implementation of the provided Algorithms and data structures from William Fiset at: https://github.com/williamfiset/Algorithms

I also highly recommend [his YouTube channel](https://www.youtube.com/user/purpongie), which explains many of these algorithms in detail.

## Usage

The implementation details are explained in comments and docs and the example usage can be found in unit tests.

The output of tests are implied in `assert_eq!`s, but in case you really want to see the output, I also preserved `println!`. To make the ouput appear on the console, use:

```
cargo test [name_of_the_test] -- --nocapture
```


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