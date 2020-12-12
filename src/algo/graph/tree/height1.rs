//! # Tree height Example
//!
//! - Time Complexity: O(n)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=0qgaIMqOEVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=8)

#[derive(Default)]
pub struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn height(&self) -> Option<usize> {
        fn height<T>(opt_node: &Option<Box<TreeNode<T>>>) -> i32 {
            match opt_node {
                None => -1,
                Some(node) => std::cmp::max(height(&node.left), height(&node.right)) + 1,
            }
        }
        match height(&self.root) {
            -1 => None,
            h => Some(h as usize),
        }
    }
}

pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
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
    fn test_tree_height1() {
        //        0
        //       / \
        //      1   2
        //     / \ / \
        //    3  4 5  6
        //   / \
        //  7   8
        let root = TreeNode {
            val: 0,
            left: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode {
                    val: 3,
                    left: Some(Box::new(TreeNode::new(7))),
                    right: Some(Box::new(TreeNode::new(8))),
                })),
                right: Some(Box::new(TreeNode::new(4))),
            })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode::new(5))),
                right: Some(Box::new(TreeNode::new(6))),
            })),
        };
        let tree = BinaryTree {
            root: Some(Box::new(root)),
        };

        let empty_tree_height = BinaryTree::<i32>::default().height();
        println!("Empty tree height: {:?}", empty_tree_height);
        assert_eq!(empty_tree_height, None);

        let singleton_height = BinaryTree {
            root: Some(Box::new(TreeNode::new(5))),
        }
        .height();
        println!("Singleton height: {:?}", singleton_height);
        assert_eq!(singleton_height, Some(0));

        let tree_height = tree.height();
        println!("Tree height: {:?}", tree.height());
        assert_eq!(tree_height, Some(3));
    }
}
