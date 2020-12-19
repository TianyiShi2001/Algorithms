# Rusty Algorithms and Data Structures for Education & Leetcode Solutions

![Continuous Integration](https://github.com/TianyiShi2001/Algorithms/workflows/CI/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/TianyiShi2001/Algorithms/badge.svg?branch=main)](https://coveralls.io/github/TianyiShi2001/Algorithms?branch=main)
![lines of code](https://img.shields.io/badge/lines%20of%20code-7221-blue)

This repository presents Rust implementations of common algorithms and data structures, most of which are based on William Fiset's Java implementation: https://github.com/williamfiset/Algorithms . I highly recommend [his YouTube channel](https://www.youtube.com/user/purpongie), where he explains many of these algorithms in detail using illustrations, animations and pseudocode.

In addition to implementing W. Fiset's algorithms, I also write solutions to competitive programming problems. Some representative problems are explained in `src/problems`, and there is also a `leetcode` folder for my leetcode solutions. Both are far from completion. <!-- particularly in Leetcode. Where appropriate, I will note, or `use`, the relevent algorithm/data structure(s) in this crate. -->

## Usage

The implementation details are explained in comments and docs and the example usage is implied in unit tests. To run tests:

```
cargo test
```

<!-- Although these algorithms and data structures are mainly for learning purposes, many of them can be directly applied (copy-pasted) in the context of competitive programming. -->

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

# Contents

## Graph

### Graph Representations

- Adjacency Matrix (Weighted & Unweighted)
- Adjacency List (Weighted & Unweighted)
- Condensed Adjacency Matrix (Weighted)

### Fundamental Graph Algorithms

- Depth-first search (iterative and recursive)
- Breadth-first search (iterative)

### Tree Algorithms

- Tree representation: with or without pointer to parent
- Fundamentals (DFS, tree height, tree sum, etc.)
- Tree Center
- Tree rooting
- Tree isomorphism
- Lowest common ancestor (LCA)

### Minimum Spanning Tree/Forest

- Prim's Algorithm
- Kruskal's Algorithm

### Network Flow

- Ford-Fulkerson + DFS
- DFS with capacity scaling
- Edmonds-Karp Algorithm (BFS)
- Dinic's Algorithm (BFS + DFS)

### Shortest Path

- BFS (unweighted)
- DAG shortest path with topological sorting
- Dijkstra's algorithm (non-negative weights, SSSP)
- Bellman-Ford algorithm (SSSP)
- Floyd-Warshall algorithm (APSP)

### Others

- Bipartite check
- Topological sorting of DAG graphs and DAG shortest path
- Eulerian path/circuit
- Strongly connected components (Tarjan's algorithm)

## Data Structures

- Bit manipulation
- Priority queue (binary heap)
- Balanced tree
  - AVL tree
- Disjoin set (union-find)
- Sparse table (range query) (generic)

## Machine Learning

### Clustering

- Hierarchical clustering

## Math

- GCD/LCM
- log2

# Problems

## Dynamic Programming

- Edit distance
- Knapsack 0/1

## Back Tracking

- Sudoku
- N-Queens

## Graph

- Travelling salesman problem (brutal force & DP)

## Network Flow

- Mice and owls