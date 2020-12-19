//! This algorithm finds the center(s) of an undirected tree represented by an adjacency list.
//!
//! - Time Complexity: $O(V+E)$
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=nzF_9bjDzdc&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=10)

use crate::algo::graph::UnweightedAdjacencyList;

// A tree can either have one or two center(s)
#[derive(Debug, Eq, PartialEq)]
pub enum Center {
    One(usize),
    Two(usize, usize),
}

impl UnweightedAdjacencyList {
    /// Finds the center(s) of an undirected tree.
    /// The adjacency list must be build with undirected edges, and does not contain cycles, so that
    /// it qualifies the definition for a tree.
    pub fn center(&self) -> Center {
        let n = self.node_count();
        // Tracks the degree of each node
        // the degee of a node is the number of its neighbours (i.e. nodes that it points to)
        let mut degrees = vec![0; n];
        let mut leaves = Vec::new();
        // compute degrees and identify all leaves (i.e. nodes that are connected to only one neighbour and thus
        // with a degree of 1)
        self.nodes().for_each(|(i, neighbours)| {
            let degree = neighbours.len();
            // this also processes singleton nodes with a degree of zero
            // (but you can treat it as `degree == 1` for the sake of simplicity, in which case the
            // algorithm only works if the graph is not disconnected and contains only one tree)
            if degree <= 1 {
                leaves.push(i);
            }
            degrees[i] = degree;
        });
        let mut processed_leaves = leaves.len();
        // Pruning leaf nodes by decreasing the degree of its neighbours.
        // If the degree drops to 1, the node becomes a new leaf node and is added to `new_leaves`
        // which are processed in the next round of iteration
        // The process repeats until only the centers remain.
        while processed_leaves < n {
            let mut new_leaves = Vec::new();
            for &leaf in &leaves {
                for &neighbour in &self[leaf] {
                    degrees[neighbour] = degrees[neighbour].saturating_sub(1);
                    if degrees[neighbour] == 1 {
                        new_leaves.push(neighbour);
                    }
                }
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
