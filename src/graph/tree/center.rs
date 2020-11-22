//! This algorithm finds the center(s) of a tree.
//!
//! - Time Complexity: $O(V+E)$
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=nzF_9bjDzdc&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=10)

use crate::graph::UnweightedAdjacencyList;

#[derive(Debug, Eq, PartialEq)]
pub enum Center {
    One(usize),
    Two(usize, usize),
}

pub trait TreeCenter {
    fn center(&self) -> Center;
}

impl TreeCenter for UnweightedAdjacencyList {
    fn center(&self) -> Center {
        let n = self.len();
        let mut degrees = vec![0; n];
        let mut leaves = Vec::new();
        // identify all leaves
        self.edges.iter().enumerate().for_each(|(i, neighbours)| {
            let degree = neighbours.len();
            if degree <= 1 {
                leaves.push(i);
            }
            degrees[i] = degree;
        });
        let mut processed_leaves = leaves.len();
        // Remove leaf nodes and decrease the degree of each node adding new leaf nodes progressively
        // until only the centers remain.
        while processed_leaves < n {
            let mut new_leaves = Vec::new();
            for &leaf in &leaves {
                for &neighbour in &self[leaf] {
                    degrees[neighbour] = degrees[neighbour].wrapping_sub(1);
                    if degrees[neighbour] == 1 {
                        new_leaves.push(neighbour);
                    }
                }
                // degrees[leaf] = 0; // prune this leaf (not necessary?)
            }
            processed_leaves += new_leaves.len();
            leaves = new_leaves;
        }
        match leaves.len() {
            1 => Center::One(leaves[0]),
            2 => Center::Two(leaves[0], leaves[1]),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_center() {
        let graph = UnweightedAdjacencyList::with_size(1);
        assert_eq!(graph.center(), Center::One(0));

        let mut graph = UnweightedAdjacencyList::with_size(2);
        graph.add_undirected_edge(0, 1);
        assert_eq!(graph.center(), Center::Two(0, 1));

        let mut graph = UnweightedAdjacencyList::with_size(3);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(1, 2);
        assert_eq!(graph.center(), Center::One(1));

        let mut graph = UnweightedAdjacencyList::with_size(9);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(2, 1);
        graph.add_undirected_edge(2, 3);
        graph.add_undirected_edge(3, 4);
        graph.add_undirected_edge(5, 3);
        graph.add_undirected_edge(2, 6);
        graph.add_undirected_edge(6, 7);
        graph.add_undirected_edge(6, 8);
        assert_eq!(graph.center(), Center::One(2));
    }
}
