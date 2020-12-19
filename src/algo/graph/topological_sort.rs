//! # Topological Sort
//!
//! This topological sort implementation takes an adjacency list of an acyclic graph and returns an
//! array with the indexes of the nodes in a (non unique) topological order which tells you how to
//! process the nodes in the graph. More precisely from wiki: A topological ordering is a linear
//! ordering of its vertices such that for every directed edge uv from vertex u to vertex v, u comes
//! before v in the ordering.
//!
//! - Time Complexity: O(V + E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=eL-KzMXSXXI&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=15)
//! - [W. Fiset's video (Khan's algorithm)](https://www.youtube.com/watch?v=cIBFEhD77b4&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=16)

use crate::algo::graph::{Edge, WeightedAdjacencyList};
use partial_min_max::min;

impl WeightedAdjacencyList {
    pub fn toposort(&self) -> Vec<usize> {
        let n = self.node_count();
        let mut visited = vec![false; n];
        let mut ordering = vec![0usize; n];
        let mut i = n - 1;

        fn _dfs(
            mut i: usize,
            at: usize,
            visited: &mut [bool],
            ordering: &mut [usize],
            graph: &WeightedAdjacencyList,
        ) -> usize {
            visited[at] = true;
            for &edge in &graph[at] {
                if !visited[edge.to] {
                    i = _dfs(i, edge.to, visited, ordering, graph);
                }
            }
            ordering[i] = at;
            i.saturating_sub(1)
        }

        for at in 0..n {
            if !visited[at] {
                i = _dfs(i, at, &mut visited, &mut ordering, &self);
            }
        }

        ordering
    }
    /// Imagine building a program with dependencies
    pub fn toposort_khan(&self) -> Vec<usize> {
        let n = self.node_count();
        // `dependencies[i]` is the number of nodes pointing to node `i`
        let mut dependencies = vec![0; n];
        // identify all dependencies
        for (_dependency, dependent, _cost) in self.edges() {
            dependencies[dependent] += 1;
        }
        // a "buildable" is not pointed to by other nodes
        let mut buildables: Vec<_> = (0..n).filter(|&i| dependencies[i] == 0).collect();
        let mut i = 0;
        // Remove buildable nodes and decrease the degree of each node adding new buildable nodes progressively
        // until only the centers remain.
        let mut ordering = vec![0; n];
        while i < n {
            let mut new_buildables = Vec::new();
            for &buildable in &buildables {
                ordering[i] = buildable;
                i += 1;
                for &dependent in &self[buildable] {
                    let x = &mut dependencies[dependent.to];
                    *x -= 1;
                    if *x == 0 {
                        new_buildables.push(dependent.to);
                    }
                }
            }
            buildables = new_buildables;
        }
        ordering
    }
    pub fn dag_shortest_path(&self, start: usize) -> Vec<f64> {
        let toposort = self.toposort_khan();
        let mut dists = vec![f64::INFINITY; self.node_count()];
        dists[start] = 0.;
        let i = toposort
            .iter()
            .position(|&node_id| node_id == start)
            .unwrap();
        toposort.into_iter().skip(i).for_each(|node_id| {
            let cur_dist = dists[node_id];
            if cur_dist.is_finite() {
                for &Edge { to, weight } in &self[node_id] {
                    let new_dist = cur_dist + weight;
                    let dist = &mut dists[to];
                    *dist = min(*dist, new_dist);
                }
            }
        });
        dists
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_toposort() {
        // Example from https://www.youtube.com/watch?v=cIBFEhD77b4&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=16
        // 7:12 of the video.
        let edges = [
            [0, 2],
            [3, 1],
            [1, 4],
            [4, 5],
            [3, 4],
            [2, 6],
            [6, 7],
            [4, 8],
            [9, 2],
            [9, 10],
            [10, 6],
            [6, 11],
            [11, 12],
            [12, 8],
            [7, 12],
            [0, 6],
        ];
        let graph = WeightedAdjacencyList::new_directed_unweighted(13, &edges);
        let ordering = graph.toposort_khan();
        assert!(check_sort_result(&ordering, &edges));
        let ordering = graph.toposort();
        assert!(check_sort_result(&ordering, &edges));

        fn check_sort_result(result: &[usize], edges: &[[usize; 2]]) -> bool {
            let mut rank = vec![0; result.len()];
            for (i, &node) in result.iter().enumerate() {
                rank[node] = i;
            }
            edges
                .iter()
                .all(|&[dependency, dependent]| rank[dependency] < rank[dependent])
        }
    }
    #[test]
    fn test_dag_shortest_path() {
        let edges = &[
            (0, 1, 3.),
            (0, 2, 2.),
            (0, 5, 3.),
            (1, 3, 1.),
            (1, 2, 6.),
            (2, 3, 1.),
            (2, 4, 10.),
            (3, 4, 5.),
            (5, 4, 7.),
        ];
        let graph = WeightedAdjacencyList::new_directed(7, edges);
        let dists = graph.dag_shortest_path(0);
        assert_eq!(&dists, &[0., 3., 2., 3., 8., 3., f64::INFINITY])
    }
}
