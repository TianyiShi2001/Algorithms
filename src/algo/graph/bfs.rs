//! # Breadth First Search
//!
//! - Time Complexity: O(V + E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=oDqjPvD54Ss&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=5)
//! - [W. Fiset's video](https://www.youtube.com/watch?v=KiCBXu4P-2Y&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=6)

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
        let graph = UnweightedAdjacencyList::new_undirected(
            13,
            &[
                [0, 7],
                [0, 9],
                [0, 11],
                [7, 11],
                [7, 6],
                [7, 3],
                [6, 5],
                [3, 4],
                [2, 3],
                [2, 12],
                [12, 8],
                [8, 1],
                [1, 10],
                [10, 9],
                [9, 8],
            ],
        );

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

pub mod fast_queue {
    //! # Breadth First Search (Iterative Implementation)
    //!
    //! This implementation does not track the depth, and thus can make use of the faster fixed size queue.

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
            let graph = UnweightedAdjacencyList::new_undirected(
                13,
                &[
                    [0, 7],
                    [0, 9],
                    [0, 11],
                    [7, 11],
                    [7, 6],
                    [7, 3],
                    [6, 5],
                    [3, 4],
                    [2, 3],
                    [2, 12],
                    [12, 8],
                    [8, 1],
                    [1, 10],
                    [10, 9],
                    [9, 8],
                ],
            );

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
}
