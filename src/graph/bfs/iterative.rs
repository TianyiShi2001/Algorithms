use crate::data_structures::queue::{FixedCapacityQueue, Queue};
use crate::graph::AdjacencyList;
use std::collections::VecDeque;
use std::marker::PhantomData;

#[derive(Default)]
pub struct BfsIterativeSolver<T: Queue<usize>> {
    phantom: PhantomData<T>,
}

impl<T: Queue<usize>> BfsIterativeSolver<T> {
    /// Perform a breadth first search on a graph a starting node `start`.
    pub fn bfs(graph: &AdjacencyList, start: usize) -> Vec<Option<usize>> {
        let n = graph.len();
        // tracks who the parent of `i` was
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut queue = T::with_capacity(n);

        // Start by visiting the `start` node and push it to the queue.
        queue.push_back(start);
        visited[start] = true;

        // Continue until the BFS is donw.
        while let Some(node) = queue.pop_front() {
            let neighbours = &graph.edges[node];

            // Loop through all edges attached to this node. Mark nodes as visited once they`re
            // in the queue. This will prevent having duplicate nodes in the queue and speedup the BFS.
            for &edge in neighbours {
                if !visited[edge.to] {
                    visited[edge.to] = true;
                    prev[edge.to] = Some(node);
                    queue.push_back(edge.to);
                }
            }
        }

        prev
    }

    pub fn reconstruct_path(graph: &AdjacencyList, start: usize, end: usize) -> Vec<usize> {
        let prev = Self::bfs(graph, start);
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

fn format_path(path: &Vec<usize>) -> String {
    path.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_adjacency_list_iterative() {
        const N: usize = 13;
        let mut graph = AdjacencyList::with_size(N);
        graph.add_unweighted_undirected_edge(0, 7);
        graph.add_unweighted_undirected_edge(0, 9);
        graph.add_unweighted_undirected_edge(0, 11);
        graph.add_unweighted_undirected_edge(7, 11);
        graph.add_unweighted_undirected_edge(7, 6);
        graph.add_unweighted_undirected_edge(7, 3);
        graph.add_unweighted_undirected_edge(6, 5);
        graph.add_unweighted_undirected_edge(3, 4);
        graph.add_unweighted_undirected_edge(2, 3);
        graph.add_unweighted_undirected_edge(2, 12);
        graph.add_unweighted_undirected_edge(12, 8);
        graph.add_unweighted_undirected_edge(8, 1);
        graph.add_unweighted_undirected_edge(1, 10);
        graph.add_unweighted_undirected_edge(10, 9);
        graph.add_unweighted_undirected_edge(9, 8);

        let (start, end) = (10, 5);

        let path = BfsIterativeSolver::<VecDeque<_>>::reconstruct_path(&graph, start, end);
        let fmtpath = format_path(&path);
        println!(
            "The shortest path from {} to {} is: {}\n",
            start, end, fmtpath
        );
        assert_eq!(&fmtpath, "10 -> 9 -> 0 -> 7 -> 6");

        let path =
            BfsIterativeSolver::<FixedCapacityQueue<_>>::reconstruct_path(&graph, start, end);
        let fmtpath = format_path(&path);
        println!(
            "The shortest path from {} to {} is: {}\n",
            start, end, fmtpath
        );
        assert_eq!(&fmtpath, "10 -> 9 -> 0 -> 7 -> 6");
    }
}
