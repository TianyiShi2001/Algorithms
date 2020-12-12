pub mod bfs;
pub mod dfs;
pub mod dijkstra_shortest_path;
pub mod topological_sort;
pub mod tree;

#[derive(Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: f32,
}
impl Edge {
    pub fn new(to: usize, cost: f32) -> Self {
        Self { to, cost }
    }
}

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
    /// Number of nodes
    pub fn len(&self) -> usize {
        self.edges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    /// Add a directed edge from node `u` to node `v` with cost `cost`.
    pub fn add_directed_edge(&mut self, u: usize, v: usize, cost: f32) {
        self.edges[u].push(Edge::new(v, cost))
    }
    /// Add an undirected edge between nodes `u` and `v`.
    pub fn add_undirected_edge(&mut self, u: usize, v: usize, cost: f32) {
        self.add_directed_edge(u, v, cost);
        self.add_directed_edge(v, u, cost);
    }
    pub fn new_directed(size: usize, edges: &[(usize, usize, f32)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_directed_edge(a, b, c);
        }
        graph
    }
    pub fn new_undirected(size: usize, edges: &[(usize, usize, f32)]) -> Self {
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
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        self.edges
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.to, b.cost)))
    }
}

impl std::ops::Index<usize> for WeightedAdjacencyList {
    type Output = Vec<Edge>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

pub struct UnweightedAdjacencyList {
    edges: Vec<Vec<usize>>,
}

impl UnweightedAdjacencyList {
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
}

impl std::ops::Index<usize> for UnweightedAdjacencyList {
    type Output = Vec<usize>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
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
}
