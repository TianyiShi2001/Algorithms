pub mod bfs;
pub mod dfs;

#[derive(Copy, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: i32,
}
impl Edge {
    pub fn new(from: usize, to: usize, cost: i32) -> Self {
        Self { from, to, cost }
    }
}

pub struct AdjacencyList {
    edges: Vec<Vec<Edge>>,
}

impl AdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }
    /// Number of nodes
    pub fn len(&self) -> usize {
        self.edges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Add a directed edge from node `u` to node `v` with cost `cost`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, cost: i32) {
        self.edges[u].push(Edge::new(u, v, cost))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, cost: i32) {
        self.add_directed_edge(u, v, cost);
        self.add_directed_edge(v, u, cost);
    }
    /// Add an undirected unweighted edge between nodes `u` and `v`. The edge added
    /// will have a weight of 1 since its intended to be unweighted.
    pub fn add_unweighted_undirected_edge(&mut self, u: usize, v: usize) {
        self.add_undirected_edge(u, v, 1);
    }
}

impl std::ops::Index<usize> for AdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}
