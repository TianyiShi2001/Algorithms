//! # Tree Sum Example
//!
//! - Time Complexity: O(n)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=0qgaIMqOEVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=8)

// making the tree generic over all summable types, e.g. `f64`, `i32`, or your own
// `Complex` type (as long as it implements these traits)
pub trait Summable: std::ops::AddAssign<Self> + Copy + num_traits::Zero {}
impl<T: std::ops::AddAssign<Self> + Copy + num_traits::Zero> Summable for T {}

pub struct TreeNode<T: Summable> {
    val: T,
    children: Vec<Box<TreeNode<T>>>,
}

impl<T: Summable> TreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(Box::new(child));
    }

    pub fn sum(&self) -> T {
        self.children
            .iter()
            .fold(T::zero(), |sum, child| sum + child.sum())
            + self.val
    }
    pub fn leaf_sum(&self) -> T {
        // a leaf has no children
        if self.children.is_empty() {
            self.val
        } else {
            self.children
                .iter()
                .fold(T::zero(), |sum, child| sum + child.leaf_sum())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sum() {
        let mut root = TreeNode::new(5);
        let mut node4 = TreeNode::new(4);
        let mut node3 = TreeNode::new(3);
        let mut node1 = TreeNode::new(1);
        let mut node7 = TreeNode::new(7);

        let nodem6 = TreeNode::new(-6);
        let node0 = TreeNode::new(0);
        let nodem4 = TreeNode::new(-4);
        let node2 = TreeNode::new(2);
        let node9 = TreeNode::new(9);
        let node8 = TreeNode::new(8);

        node1.add_child(node2);
        node1.add_child(node9);
        node4.add_child(node1);
        node4.add_child(nodem6);
        root.add_child(node4);
        node3.add_child(node0);
        node7.add_child(node8);
        node3.add_child(node7);
        node3.add_child(nodem4);
        root.add_child(node3);

        let sum = root.sum();
        println!("Tree sum: {}", sum);
        assert_eq!(sum, 29);

        let leaf_sum = root.leaf_sum();
        println!("Leaf sum: {}", leaf_sum);
        assert_eq!(leaf_sum, 9);
    }
}
