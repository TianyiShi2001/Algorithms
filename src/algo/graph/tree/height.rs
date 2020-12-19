//! # Tree height Example
//!
//! - Time Complexity: O(n)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=0qgaIMqOEVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=8)

use super::{BinaryTreeNode, Node};

impl Node {
    pub fn height(&self) -> usize {
        self.children
            .iter()
            .fold(0, |height, child| std::cmp::max(height, child.height() + 1))
    }
}

impl BinaryTreeNode {
    pub fn height(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 0,
            (Some(node), None) | (None, Some(node)) => node.height() + 1,
            (Some(l), Some(r)) => std::cmp::max(l.height(), r.height()) + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_height() {
        //         6
        //      2  7  8
        //    1  3
        //  0   4 5
        let root = Node {
            id: 6,
            children: vec![
                Node {
                    id: 2,
                    children: vec![
                        Node {
                            id: 1,
                            children: vec![Node::new(0)],
                        },
                        Node {
                            id: 3,
                            children: vec![Node::new(4), Node::new(5)],
                        },
                    ],
                },
                Node::new(7),
                Node::new(8),
            ],
        };
        assert_eq!(root.height(), 3);
        let node_2 = &root.children[0];
        assert_eq!(node_2.height(), 2);
        let node_7 = &root.children[1];
        assert_eq!(node_7.height(), 0);
    }
    #[test]
    fn test_binary_tree_height() {
        //        0
        //       / \
        //      1   2
        //     / \ / \
        //    3  4 5  6
        //   / \
        //  7   8
        let tree = BinaryTreeNode {
            id: 0,
            left: Some(Box::new(BinaryTreeNode {
                id: 1,
                left: Some(Box::new(BinaryTreeNode {
                    id: 3,
                    left: Some(Box::new(BinaryTreeNode::new(7))),
                    right: Some(Box::new(BinaryTreeNode::new(8))),
                })),
                right: Some(Box::new(BinaryTreeNode::new(4))),
            })),
            right: Some(Box::new(BinaryTreeNode {
                id: 2,
                left: Some(Box::new(BinaryTreeNode::new(5))),
                right: Some(Box::new(BinaryTreeNode::new(6))),
            })),
        };

        let singleton_height = BinaryTreeNode::new(5).height();
        println!("Singleton height: {}", singleton_height);
        assert_eq!(singleton_height, 0);

        let tree_height = tree.height();
        println!("Tree height: {}", tree.height());
        assert_eq!(tree_height, 3);
    }
}
