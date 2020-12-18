pub mod bfs;
pub mod dfs;
pub mod eulerian_path;
pub mod minimum_spanning_tree;
pub mod network_flow;
pub mod shortest_path;
pub mod tarjan_scc;
pub mod topological_sort;
pub mod tree;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: f64,
}
impl Edge {
    pub fn new(to: usize, cost: f64) -> Self {
        Self { to, cost }
    }
}

#[derive(Debug)]
pub struct WeightedAdjacencyList {
    edges: Vec<Vec<Edge>>,
}

impl WeightedAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Add a directed edge from node `u` to node `v` with cost `cost`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, cost: f64) {
        self.edges[u].push(Edge::new(v, cost))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, cost: f64) {
        self.add_directed_edge(u, v, cost);
        self.add_directed_edge(v, u, cost);
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
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        self.edges
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.to, b.cost)))
    }
    pub fn edge_count(&self) -> usize {
        self.edges().count()
    }
    pub fn vertices(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.edges.iter().enumerate()
    }
    pub fn node_count(&self) -> usize {
        self.edges.len()
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

#[derive(Debug)]
pub struct UnweightedAdjacencyList {
    edges: Vec<Vec<usize>>,
    // is_directed: bool,
}

impl UnweightedAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
            //is_directed: true,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Add a directed edge from node `u` to node `v`
    pub fn add_directed_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v)
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
    pub fn edges(&self) -> impl Iterator<Item = [usize; 2]> + '_ {
        self.edges
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |&b| [a, b]))
    }
    pub fn edge_count(&self) -> usize {
        self.edges().count()
    }
    pub fn vertices(&self) -> impl Iterator<Item = (usize, &Vec<usize>)> {
        self.edges.iter().enumerate()
    }
    pub fn node_count(&self) -> usize {
        self.edges.len()
    }
}

impl std::ops::Index<usize> for UnweightedAdjacencyList {
    type Output = Vec<usize>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

pub struct WeightedAdjacencyMatrix {
    inner: Vec<Vec<f64>>,
}

impl WeightedAdjacencyMatrix {
    #[allow(clippy::needless_range_loop)]
    pub fn with_size(n: usize) -> Self {
        let mut inner = vec![vec![f64::INFINITY; n]; n];
        // distance of each vertex to itself defaults to zero.
        for i in 0..n {
            inner[i][i] = 0.;
        }
        Self { inner }
    }
    pub fn node_count(&self) -> usize {
        self.inner.len()
    }
    pub fn from_adjacency_list(inp: &WeightedAdjacencyList) -> Self {
        let mut res = Self::with_size(inp.node_count());
        for (from, edges) in inp.vertices() {
            for &Edge { to, cost } in edges {
                res.inner[from][to] = cost;
            }
        }
        res
    }
}

impl From<WeightedAdjacencyList> for WeightedAdjacencyMatrix {
    fn from(inp: WeightedAdjacencyList) -> Self {
        Self::from_adjacency_list(&inp)
    }
}

impl From<Vec<Vec<f64>>> for WeightedAdjacencyMatrix {
    fn from(inner: Vec<Vec<f64>>) -> Self {
        Self { inner }
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyMatrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

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

impl fmt::Display for WeightedAdjacencyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", WeightedAdjacencyMatrix::from_adjacency_list(self))
    }
}

#[derive(Debug)]
pub struct WeightedUndirectedAdjacencyMatrixCondensed {
    inner: Vec<f64>,
    n: usize,
}

impl WeightedUndirectedAdjacencyMatrixCondensed {
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
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        (0..self.n - 1)
            .flat_map(move |i| (i + 1..self.n).map(move |j| (i, j)))
            .zip(self.inner.iter())
            .map(|((i, j), w)| (i, j, *w))
    }

    pub fn node_count(&self) -> usize {
        self.n
    }
}

impl std::ops::Index<(usize, usize)> for WeightedUndirectedAdjacencyMatrixCondensed {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        use std::cmp::Ordering::*;
        match i.cmp(&j) {
            Less => {
                let (mut _i, mut _j, mut j, mut k) = (i, self.n, j - 1, 0);
                while _i > 0 {
                    j -= 1;
                    _j -= i;
                    k += _j;
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
                let (mut _i, mut _j, mut j, mut k) = (i, self.n, j - 1, 0);
                while _i > 0 {
                    j -= 1;
                    _j -= 1;
                    k += _j;
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
