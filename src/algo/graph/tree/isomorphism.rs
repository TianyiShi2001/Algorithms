//! Determines if two unrooted trees are isomorphic. This algorithm can easily be modified to support
//! checking if two rooted trees are isomorphic.
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=OCKvEMF0Xac&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=11)

use crate::algo::graph::tree::center::Center;
use crate::algo::graph::tree::Node;
use crate::algo::graph::UnweightedAdjacencyList;

impl From<Center> for Vec<usize> {
    fn from(center: Center) -> Self {
        match center {
            Center::One(x) => vec![x],
            Center::Two(x, y) => vec![x, y],
        }
    }
}

impl Node {
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
            let tree1 = Node::from_adjacency_list(&self, c1);
            for &c2 in &other_centers {
                let tree2 = Node::from_adjacency_list(&self, c2);
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
        let adj = UnweightedAdjacencyList::new_undirected(
            10,
            &[
                [0, 2],
                [0, 1],
                [0, 3],
                [2, 6],
                [2, 7],
                [1, 4],
                [1, 5],
                [5, 9],
                [3, 8],
            ],
        );
        let root = Node::from_adjacency_list(&adj, 0);
        let encoded = root.encode();
        let encoded = String::from_utf8(encoded).unwrap();
        assert_eq!(&encoded, "(((())())(()())(()))")
    }

    #[test]
    fn test_tree_isomorphism() {
        let tree1 = UnweightedAdjacencyList::new_undirected(5, &[[2, 0], [3, 4], [2, 1], [2, 3]]);
        let tree2 = UnweightedAdjacencyList::new_undirected(5, &[[1, 0], [2, 4], [1, 3], [1, 2]]);
        assert!(tree1.is_isomorphic_with(&tree2));
    }
}
