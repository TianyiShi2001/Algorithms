use std::cmp::Ordering;
use std::fmt::Debug;
use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;
pub trait BTreeItem = Ord + Debug + PartialEq + Eq + Clone;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node<T: BTreeItem> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: BTreeItem> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BTree<T: BTreeItem> {
    pub roots: Vec<Link<T>>,
}

impl<T: BTreeItem> BTree<T> {
    pub fn new() -> Self {
        Self { roots: vec![None] }
    }
    pub fn version(&self, version: usize) -> &Link<T> {
        &self.roots[version]
    }
    pub fn num_versions(&self) -> usize {
        self.roots.len()
    }
    pub fn latest(&self) -> &Link<T> {
        &self.roots[self.num_versions() - 1]
    }
    pub fn insert(&mut self, value: T) {
        fn _insert<T: BTreeItem>(parent: &Link<T>, value: T) -> Link<T> {
            let node = match parent {
                None => Node::new(value),
                Some(parent) => {
                    let mut node = Node::new(parent.value.clone());
                    if value <= parent.value {
                        node.left = _insert(&parent.left, value);
                        node.right = parent.right.clone();
                    } else {
                        node.right = _insert(&parent.right, value);
                        node.left = parent.left.clone();
                    }
                    node
                }
            };
            Some(Arc::new(node))
        }
        self.roots.push(_insert(self.latest(), value));
    }
    pub fn remove(&mut self, value: &T) -> bool {
        fn _insert_right_branch_to_left_branch<T: BTreeItem>(
            parent: &Link<T>,
            right: Link<T>,
        ) -> Link<T> {
            match parent {
                None => right,
                Some(parent) => {
                    let mut node = Node::new(parent.value.clone());
                    node.left = parent.left.clone();
                    node.right = _insert_right_branch_to_left_branch(&parent.right, right);
                    Some(Arc::new(node))
                }
            }
        }
        fn _remove<T: BTreeItem>(curr: &Link<T>, value: &T) -> Link<T> {
            match curr {
                None => return None,
                Some(curr) => {
                    if value == &curr.value {
                        // Use the left branch to replace the removed value,
                        // and insert the right branch to the nearest empty
                        // `right` field since the top value of the right
                        // branch is greater than every value in the left
                        // branch.
                        if curr.right.is_none() {
                            // if right is empty simply reuse (unmodified) left branch
                            curr.left.clone()
                        } else {
                            // else probe recursively to find the empty `field` to insert
                            _insert_right_branch_to_left_branch(&curr.left, curr.right.clone())
                        }
                    } else {
                        let mut node = Node::new(curr.value.clone());
                        if value < &curr.value {
                            node.left = _remove(&curr.left, value);
                            node.right = curr.right.clone();
                        } else {
                            node.right = _remove(&curr.right, value);
                            node.left = curr.left.clone();
                        }
                        Some(Arc::new(node))
                    }
                }
            }
        }
        match _remove(self.latest(), value) {
            None => false,
            new_root => {
                self.roots.push(new_root);
                true
            }
        }
    }
    pub fn find(&self, value: &T, version: usize) -> &Link<T> {
        let mut curr = &self.roots[version];
        while let Some(node) = curr {
            match value.cmp(&node.value) {
                Ordering::Less => curr = &node.left,
                Ordering::Greater => curr = &node.right,
                Ordering::Equal => return curr,
            }
        }
        &None
    }
    pub fn find_latest(&self, value: &T) -> &Link<T> {
        self.find(value, self.num_versions() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btree() {
        let mut bt = BTree::new();
        bt.insert(4);
        assert!(bt.find_latest(&4).is_some());
        bt.insert(2);
        bt.insert(3);
        bt.insert(5);
        bt.insert(1);
        bt.insert(2);
        assert!(bt.find_latest(&2).is_some());
        assert!(bt.find_latest(&3).is_some());
        assert!(bt.find_latest(&5).is_some());
        assert!(bt.find_latest(&1).is_some());
        assert!(bt.find_latest(&8).is_none());
        assert!(bt.remove(&2));
        assert!(bt.find_latest(&2).is_some());
        assert!(bt.remove(&2));
        assert!(bt.find_latest(&2).is_none());
        assert!(bt.remove(&3));
        assert!(bt.find_latest(&3).is_none());
    }
}
