//! # Graph Theory Algorithms
//!
//! This module contains implementations of graph and tree representations and algorithms.
//!
//! I highly recommend watching [William Fiset's video series on graph theory algorithms](https://www.youtube.com/watch?v=DgXR2OWQnLc&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P)
//! While following along, try implementing the algorithms yourself before comparing them to implementations presented here or
//! William's original Java implementations.

pub mod bfs;
pub mod bipartite_check;
pub mod dfs;
pub mod eulerian_path;
pub mod minimum_spanning_tree;
pub mod network_flow;
pub mod shortest_path;
pub mod tarjan_scc;
pub mod topological_sort;
pub mod tree;

use std::fmt;

/// The Edge type used in [`WeightedAdjacencyList`].
/// A `from` field is not required because it's implied by its position in the adjacency list.
#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}
impl Edge {
    /// Consruct a new (directed) weighted `Edge`
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

/// A graph represented by a weighted adjacency list.
/// Under the hood, a weighted adjacency list is a `Vec` of `Vec` of `Edge`s.
/// For an adjacency list `g`, `g[i]` is a `Vec` of edges pointing from `i` to other nodes (vertices).
/// Thus, the number of nodes is implied by the `len` of the (outer) `Vec`.
/// For each node `i` that do not have outgoing edges, `g[i]` is an empty vector.
#[derive(Debug)]
pub struct WeightedAdjacencyList {
    inner: Vec<Vec<Edge>>,
}

impl WeightedAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            inner: vec![vec![]; n],
        }
    }
    /// Is the graph devoid of vertices?
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Add a directed edge from node `u` to node `v` with weight `weight`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.inner[u].push(Edge::new(v, weight))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, weight: f64) {
        self.add_directed_edge(u, v, weight);
        self.add_directed_edge(v, u, weight);
    }
    pub fn new_directed(size: usize, edges: &[(usize, usize, f64)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_directed_edge(a, b, c);
        }
        graph
    }
    pub fn new_undirected(size: usize, edges: &[(usize, usize, f64)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_undirected_edge(a, b, c);
        }
        graph
    }
    pub fn new_directed_unweighted(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_directed_edge(a, b, 1.);
        }
        graph
    }
    pub fn new_undirected_unweighted(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_undirected_edge(a, b, 1.);
        }
        graph
    }
    /// Iterates over all edges in the gragh.
    /// Each item is a tuples of 3: `(from, to, weight)`
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.to, b.weight)))
    }
    /// Number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.edges().count()
    }
    /// Iterates over all nodes in the graph.
    /// Each item is a tuple of the node id and a `Vec` of all its outgoing edges
    pub fn nodes(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.inner.iter().enumerate()
    }
    /// Number of nodes (vertices) in the graph
    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
}

/// Allows the outgoing edges of a node to be accessed easily.
impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

/// An unweighted graph represented by an unweighted adjacency list.
/// This is in principle the same as `WeightedAdjacencyList`, except that no weights are involved
/// and thus a simple `usize` is able to represent an outgoing edge.
#[derive(Debug)]
pub struct UnweightedAdjacencyList {
    inner: Vec<Vec<usize>>,
}

impl UnweightedAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            inner: vec![vec![]; n],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Add a directed edge from node `u` to node `v`
    pub fn add_directed_edge(&mut self, u: usize, v: usize) {
        self.inner[u].push(v)
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_directed_edge(u, v);
        self.add_directed_edge(v, u);
    }
    pub fn new_directed(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_directed_edge(a, b);
        }
        graph
    }
    pub fn new_undirected(size: usize, edges: &[[usize; 2]]) -> Self {
        let mut graph = Self::with_size(size);
        for &[a, b] in edges.iter() {
            graph.add_undirected_edge(a, b);
        }
        graph
    }
    /// Iterates over all edges in the gragh.
    /// Each item is an array of length 2, showing the source and destination of this edge.
    pub fn edges(&self) -> impl Iterator<Item = [usize; 2]> + '_ {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |&b| [a, b]))
    }
    /// Number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.edges().count()
    }
    /// Iterates over all nodes in the graph.
    /// Each item is a tuple of the node id and a `Vec` of all its outgoing edges
    pub fn nodes(&self) -> impl Iterator<Item = (usize, &Vec<usize>)> {
        self.inner.iter().enumerate()
    }
    /// Number of nodes (vertices) in the graph
    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
}

/// Allows the outgoing edges of a node to be accessed easily.
impl std::ops::Index<usize> for UnweightedAdjacencyList {
    type Output = Vec<usize>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

/// Dense graphs, are sometimes more efficient to be represented as adjacency matrices.
/// A `WeightedAdjacencyMatrix` is based on a Matrix of size `n * n` where n is the number of nodes (vertices) in
/// the graph.
/// For a `WeightedAdjacencyMatrix` `g`, `g[i][j]` is the weight of the edge pointing from node `i`
/// to node `j`. By convention, for two nodes `i` and `j` that are *not* connected, `g[i][j] = INFINITY`,
/// and each node by default has a weight of `0` to point to itself (i.e. `g[i][i]` = 0).
pub struct WeightedAdjacencyMatrix {
    inner: Vec<Vec<f64>>,
}

impl WeightedAdjacencyMatrix {
    #[allow(clippy::needless_range_loop)]
    pub fn with_size(n: usize) -> Self {
        // By default, all vertices are not connected and this is indicated by a weight of `INFINITY` between them
        let mut inner = vec![vec![f64::INFINITY; n]; n];
        // distance of each vertex to itself defaults to zero.
        for i in 0..n {
            inner[i][i] = 0.;
        }
        Self { inner }
    }
    /// Number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
    /// Converts a `WeightedAdjacencyList` to `WeightedAdjacencyMatrix`
    pub fn from_adjacency_list(inp: &WeightedAdjacencyList) -> Self {
        let mut res = Self::with_size(inp.node_count());
        for (from, edges) in inp.nodes() {
            for &Edge { to, weight } in edges {
                res.inner[from][to] = weight;
            }
        }
        res
    }
    /// Builds a `WeightedAdjacencyMatrix` from its underlying representation.
    pub fn from_inner(matrix: Vec<Vec<f64>>) -> Self {
        Self { inner: matrix }
    }
}

/// This allows us to access the weight of edge `i -> j` in graph `g`
/// by `g[i][j]` rather than `g.inner[i][j]`
impl std::ops::Index<usize> for WeightedAdjacencyMatrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

/// For convinience
impl From<WeightedAdjacencyList> for WeightedAdjacencyMatrix {
    fn from(inp: WeightedAdjacencyList) -> Self {
        Self::from_adjacency_list(&inp)
    }
}

/// For convinience
impl From<Vec<Vec<f64>>> for WeightedAdjacencyMatrix {
    fn from(matrix: Vec<Vec<f64>>) -> Self {
        Self::from_inner(matrix)
    }
}

/// Pretty-prints a small graph represented by an adjacency matrix
impl fmt::Display for WeightedAdjacencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.node_count();
        write!(f, "   ")?;
        for i in 0..n {
            write!(f, "{:>2} ", i)?;
        }
        writeln!(f)?;
        for i in 0..n {
            write!(f, "{:>2} ", i)?;
            for j in 0..n {
                let x = self[i][j];
                if x == f64::INFINITY {
                    write!(f, " ∞ ")?;
                } else if x == f64::NEG_INFINITY {
                    write!(f, "-∞ ")?;
                } else {
                    write!(f, "{:>2} ", self[i][j])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Pretty-prints a small graph represented by a weighted adjacency list
/// The graph is first converted to a `WeightedAdjacencyMatrix` before being printed
impl fmt::Display for WeightedAdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", WeightedAdjacencyMatrix::from_adjacency_list(self))
    }
}

/// An adjacency matrix representing an undirected graph is symmetric with respect to the main diagonal,
/// i.e. `g[i][j]` = `g[j][i]`. Thus, only half of the values in the matrix need to be stored. In addition,
/// The assumption that `g[i][i] = 0` works fine in most cases, so weights representing these self-pointing
/// edges are also not stored.
///
/// Thus a condensed matrix stores only `g[i][j]` where `i < j` in a vector of length ${}^{n}C_{2} = \\dfrac{n(n-1)}{2}$
/// (n-choose-2). For example, in a graph with 4 vertices, the 6 weights in the condensed vector represents
/// `g[0 -> 1], g[0 -> 2], g[0 -> 3], g[1 -> 2], g[1 -> 3], g[2 -> 3]`, respectively.
#[derive(Debug)]
pub struct WeightedUndirectedAdjacencyMatrixCondensed {
    inner: Vec<f64>,
    n: usize,
}

impl WeightedUndirectedAdjacencyMatrixCondensed {
    pub fn new(node_count: usize) -> Self {
        Self {
            inner: vec![f64::INFINITY; node_count * (node_count - 1) / 2],
            n: node_count,
        }
    }
    /// Build a `WeightedUndirectedAdjacencyMatrixCondensed` from [`WeightedAdjacencyList`].
    /// The graph must be undirected. Even if the [`WeightedAdjacencyList`] were build with
    /// directed edges, they are treated as undirected edges.
    ///
    /// # Panics
    ///
    /// Panics if the [`WeightedAdjacencyList`] contains both `g[i -> j]` and `g[j -> i]` but
    /// their weights differ
    pub fn from_adjacency_list(inp: &WeightedAdjacencyList) -> Self {
        let n = inp.node_count();
        let mut m = Self {
            inner: vec![f64::INFINITY; n * (n - 1) / 2],
            n,
        };
        for (i, j, weight) in inp.edges() {
            let w = &mut m[(i, j)];
            if w.is_finite() {
                assert!(
                    (*w - weight).abs() < f64::EPSILON,
                    "Graph contains directed edge(s)!"
                );
            } else {
                *w = weight;
            }
        }
        m
    }
    /// Builds a `WeightedUndirectedAdjacencyMatrixCondensed` from its inner representation.
    pub fn from_slice(inp: &[f64]) -> Self {
        assert!(!inp.is_empty(), "Inpud cannot be empty.");
        let mut n = 2;
        loop {
            n += 1;
            let len = n * (n - 1) / 2;
            if len == inp.len() {
                return Self {
                    inner: inp.to_owned(),
                    n,
                };
            }
            if len > inp.len() {
                panic!("Invalid input length.")
            }
        }
    }
    /// Iterate over all pairs of nodes `(i, j)` where `i < j` with the weight associated with the pair.
    /// Each item is a tuple of 3: `(i, j, weight)`.
    /// Note that only `(i, j)` but not `(j, i)` is outputted.
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        (0..self.n - 1)
            .flat_map(move |i| (i + 1..self.n).map(move |j| (i, j)))
            .zip(self.inner.iter())
            .map(|((i, j), w)| (i, j, *w))
    }
    /// Number of nodes (vertices) in the graph
    pub fn node_count(&self) -> usize {
        self.n
    }
    pub fn resized(&self, new_node_count: usize) -> Self {
        let mut new = Self::new(new_node_count);
        for (i, j, weight) in self.edges() {
            if i < new_node_count && j < new_node_count {
                new[(i, j)] = weight;
            }
        }
        new
    }
}

/// This allows indexing into graphs represented by [`WeightedUndirectedAdjacencyMatrixCondensed`] easier.
/// For example, for a graph `g`, either `g[(i, j)]` or `g[(j, i)]` will give the weight associated with node
/// `i` and node `j` (remember the graph is undirected)
impl std::ops::Index<(usize, usize)> for WeightedUndirectedAdjacencyMatrixCondensed {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        use std::cmp::Ordering::*;
        assert!(i < self.n && j < self.n, "Index out of bound.");
        match i.cmp(&j) {
            Less => {
                let (mut _i, mut j_, mut j, mut k) = (i, self.n - 1, j - 1, 0);
                while _i > 0 {
                    k += j_;
                    j_ -= 1;
                    j -= 1;
                    _i -= 1;
                }
                k += j;
                &self.inner[k]
            }
            Greater => self.index((j, i)),
            Equal => &0.,
        }
    }
}
impl std::ops::IndexMut<(usize, usize)> for WeightedUndirectedAdjacencyMatrixCondensed {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        use std::cmp::Ordering::*;
        match i.cmp(&j) {
            Less => {
                let (mut _i, mut j_, mut j, mut k) = (i, self.n - 1, j - 1, 0);
                while _i > 0 {
                    k += j_;
                    j_ -= 1;
                    j -= 1;
                    _i -= 1;
                }
                k += j;
                &mut self.inner[k]
            }
            Greater => self.index_mut((j, i)),
            Equal => panic!("Not allowed to assign a weight from a vetex to itself!"),
        }
    }
}

impl From<WeightedAdjacencyList> for WeightedUndirectedAdjacencyMatrixCondensed {
    fn from(inp: WeightedAdjacencyList) -> Self {
        Self::from_adjacency_list(&inp)
    }
}

impl fmt::Display for WeightedUndirectedAdjacencyMatrixCondensed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.node_count();
        write!(f, "   ")?;
        for i in 1..n {
            write!(f, "{:6} ", i)?;
        }
        writeln!(f)?;
        for i in 0..n - 1 {
            write!(f, "{:2}", i)?;
            for _ in 0..i {
                write!(f, "       ")?;
            }
            for j in i + 1..n {
                let x = self[(i, j)];
                if x == f64::INFINITY {
                    write!(f, "      ∞")?;
                } else if x == f64::NEG_INFINITY {
                    write!(f, "     -∞")?;
                } else {
                    write!(f, " {:6.2}", x)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_graph_adj_list() {
        let mut edges = vec![[0, 1], [1, 2], [0, 2], [1, 1]];
        let g = UnweightedAdjacencyList::new_directed(3, &edges);
        for edge in g.edges() {
            let i = edges.iter().position(|e| *e == edge).unwrap();
            edges.remove(i);
        }
        assert!(edges.is_empty());
    }

    #[test]
    fn test_weighted_undirected_condensed() {
        let edges = &[
            (0, 1, 1.),
            (0, 2, 2.),
            (0, 3, 3.),
            (1, 2, 4.),
            (1, 3, 5.),
            (2, 3, 6.),
        ];
        let g = WeightedAdjacencyList::new_undirected(4, edges);
        let m = WeightedAdjacencyMatrix::from_adjacency_list(&g);
        let g1 = WeightedUndirectedAdjacencyMatrixCondensed::from_adjacency_list(&g);
        let g2 = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&[1., 2., 3., 4., 5., 6.]);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(m[i][j], g1[(i, j)]);
                assert_eq!(m[i][j], g2[(i, j)]);
            }
        }
        assert_eq!(&g1.edges().collect::<Vec<_>>(), edges);
    }
}
