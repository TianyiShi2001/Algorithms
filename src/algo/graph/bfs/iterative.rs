//! # Breadth First Search (Iterative Implementation)
//!
//! - Time Complexity: O(V + E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=oDqjPvD54Ss&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=5)

use crate::algo::graph::UnweightedAdjacencyList;
use std::collections::VecDeque;

impl UnweightedAdjacencyList {
    /// Perform a breadth first search on a graph a starting node `start`.
    pub fn bfs(&self, start: usize) -> BfsResult {
        // Each breadth first search layer gets separated by a DEPTH_TOKEN.
        // DEPTH_TOKENs help count the distance from one node to another because
        // we can increment the depth counter each time a DEPTH_TOKEN is encountered
        const DEPTH_TOKEN: usize = usize::MAX;
        // number of nodes
        let n = self.node_count();
        // tracks who the parent of `i` was
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut queue = VecDeque::with_capacity(n);

        // Start by visiting the `start` node and push it to the queue.
        queue.push_back(start);
        queue.push_back(DEPTH_TOKEN);
        visited[start] = true;

        let mut depth = 0;

        // Continue until the BFS is done.
        while let Some(node) = queue.pop_front() {
            if queue.is_empty() {
                break;
            }
            if node == DEPTH_TOKEN {
                queue.push_back(DEPTH_TOKEN);
                depth += 1;
                continue;
            }
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

        BfsResult { prev, depth }
    }
}
pub struct BfsResult {
    prev: Vec<Option<usize>>,
    pub depth: usize,
}

impl BfsResult {
    pub fn path_to(&self, end: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut at = end;
        while let Some(prev_parent) = self.prev[at] {
            at = prev_parent;
            path.push(at);
        }
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let bfs_result = graph.bfs(start);
        let depth = bfs_result.depth;
        assert_eq!(depth, 5);
        let path = bfs_result.path_to(end);
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
