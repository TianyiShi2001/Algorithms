//! # Breadth First Search (Iterative Implementation)
//!
//! This implementation does not track the depth, and thus can make use of the faster fixed size queue.
//!
//! - Time Complexity: O(V + E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=oDqjPvD54Ss&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=5)

use crate::algo::graph::UnweightedAdjacencyList;
use crate::data_structures::queue::Queue;

pub trait BfsReconstructPath {
    fn bfs<T: Queue<usize>>(&self, start: usize) -> Vec<Option<usize>>;

    fn reconstruct_path<T: Queue<usize>>(&self, start: usize, end: usize) -> Vec<usize> {
        let prev = self.bfs::<T>(start);
        let mut path = Vec::new();
        let mut at = end;
        while let Some(prev_parent) = prev[at] {
            at = prev_parent;
            path.push(at);
        }
        path.reverse();
        path
    }
}

impl BfsReconstructPath for UnweightedAdjacencyList {
    /// Perform a breadth first search on a graph a starting node `start`.
    fn bfs<T: Queue<usize>>(&self, start: usize) -> Vec<Option<usize>> {
        // number of nodes
        let n = self.node_count();
        // tracks who the parent of `i` was
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut queue = T::with_capacity(n);

        // Start by visiting the `start` node and push it to the queue.
        queue.push_back(start);
        visited[start] = true;

        // Continue until the BFS is donw.
        while let Some(node) = queue.pop_front() {
            let neighbours = &self[node];

            // Loop through all edges attached to this node. Mark nodes as visited once they`re
            // in the queue. This will prevent having duplicate nodes in the queue and speedup the BFS.
            for &neighbour in neighbours {
                if !visited[neighbour] {
                    visited[neighbour] = true;
                    prev[neighbour] = Some(node);
                    queue.push_back(neighbour);
                }
            }
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::queue::FixedCapacityQueue;
    use std::collections::VecDeque;
    #[test]
    fn test_bfs_adjacency_list_iterative() {
        const N: usize = 13;
        let mut graph = UnweightedAdjacencyList::with_size(N);
        graph.add_undirected_edge(0, 7);
        graph.add_undirected_edge(0, 9);
        graph.add_undirected_edge(0, 11);
        graph.add_undirected_edge(7, 11);
        graph.add_undirected_edge(7, 6);
        graph.add_undirected_edge(7, 3);
        graph.add_undirected_edge(6, 5);
        graph.add_undirected_edge(3, 4);
        graph.add_undirected_edge(2, 3);
        graph.add_undirected_edge(2, 12);
        graph.add_undirected_edge(12, 8);
        graph.add_undirected_edge(8, 1);
        graph.add_undirected_edge(1, 10);
        graph.add_undirected_edge(10, 9);
        graph.add_undirected_edge(9, 8);

        let (start, end) = (10, 5);

        let path = graph.reconstruct_path::<VecDeque<usize>>(start, end);
        let fmtpath = format_path(&path);
        println!(
            "The shortest path from {} to {} is: {}\n",
            start, end, fmtpath
        );
        assert_eq!(&fmtpath, "10 -> 9 -> 0 -> 7 -> 6");

        let path = graph.reconstruct_path::<FixedCapacityQueue<usize>>(start, end);
        let fmtpath = format_path(&path);
        println!(
            "The shortest path from {} to {} is: {}\n",
            start, end, fmtpath
        );
        assert_eq!(&fmtpath, "10 -> 9 -> 0 -> 7 -> 6");
    }
    fn format_path(path: &Vec<usize>) -> String {
        path.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}
