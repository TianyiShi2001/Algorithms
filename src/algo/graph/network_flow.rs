pub mod max_flow;

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub flow: i32,
    pub capacity: i32,
}
impl Edge {
    pub fn new(to: usize, capacity: i32) -> Self {
        Self {
            to,
            capacity,
            flow: 0,
        }
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
    pub fn add_directed_edge(&mut self, u: usize, v: usize, cost: i32) {
        self.edges[u].push(Edge::new(v, cost))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, cost: i32) {
        self.add_directed_edge(u, v, cost);
        self.add_directed_edge(v, u, cost);
    }
    pub fn new_directed(size: usize, edges: &[(usize, usize, i32)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_directed_edge(a, b, c);
        }
        graph
    }
    pub fn new_undirected(size: usize, edges: &[(usize, usize, i32)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_undirected_edge(a, b, c);
        }
        graph
    }
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, i32, i32)> + '_ {
        self.edges
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.to, b.flow, b.capacity)))
    }
    pub fn edges_count(&self) -> usize {
        self.edges().count()
    }
    pub fn vertices(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.edges.iter().enumerate()
    }
    pub fn vertices_count(&self) -> usize {
        self.edges.len()
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}
