//! Determines if two unrooted trees are isomorphic. This algorithm can easily be modified to support
//! checking if two rooted trees are isomorphic.
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=OCKvEMF0Xac&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=11)

use crate::graph::tree::center::{Center, TreeCenter};
use crate::graph::tree::rooting::TreeNode;
use crate::graph::UnweightedAdjacencyList;

impl From<Center> for Vec<usize> {
    fn from(center: Center) -> Self {
        match center {
            Center::One(x) => vec![x],
            Center::Two(x, y) => vec![x, y],
        }
    }
}

impl TreeNode {
    pub fn encode(&self) -> Vec<u8> {
        let mut labels: Vec<_> = self.children.iter().map(|node| node.encode()).collect();
        labels.sort();
        let mut res = Vec::new();
        res.push(b'(');
        for label in &labels {
            res.extend_from_slice(label);
        }
        res.push(b')');
        res
    }
}

impl UnweightedAdjacencyList {
    pub fn is_isomorphic_with(&self, other: &UnweightedAdjacencyList) -> bool {
        let this_centers: Vec<usize> = self.center().into();
        let other_centers: Vec<usize> = other.center().into();
        for &c1 in &this_centers {
            let tree1 = TreeNode::from_adjacency_list(&self, c1);
            for &c2 in &other_centers {
                let tree2 = TreeNode::from_adjacency_list(&self, c2);
                if tree1.encode() == tree2.encode() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_encoding() {
        let mut adj = UnweightedAdjacencyList::with_size(10);
        adj.add_undirected_edge(0, 2);
        adj.add_undirected_edge(0, 1);
        adj.add_undirected_edge(0, 3);
        adj.add_undirected_edge(2, 6);
        adj.add_undirected_edge(2, 7);
        adj.add_undirected_edge(1, 4);
        adj.add_undirected_edge(1, 5);
        adj.add_undirected_edge(5, 9);
        adj.add_undirected_edge(3, 8);
        let tree = TreeNode::from_adjacency_list(&adj, 0);
        let encoded = tree.encode();
        let encoded = String::from_utf8(encoded).unwrap();
        assert_eq!(&encoded, "(((())())(()())(()))")
    }

    #[test]
    fn test_tree_isomorphism() {
        let mut tree1 = UnweightedAdjacencyList::with_size(5);
        tree1.add_undirected_edge(2, 0);
        tree1.add_undirected_edge(3, 4);
        tree1.add_undirected_edge(2, 1);
        tree1.add_undirected_edge(2, 3);
        let mut tree2 = UnweightedAdjacencyList::with_size(5);
        tree2.add_undirected_edge(1, 0);
        tree2.add_undirected_edge(2, 4);
        tree2.add_undirected_edge(1, 3);
        tree2.add_undirected_edge(1, 2);
        assert!(tree1.is_isomorphic_with(&tree2));
    }
}
