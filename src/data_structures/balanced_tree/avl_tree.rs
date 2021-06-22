//! This mod contains an implementation of an AVL tree. An AVL tree is a special type of binary tree
//! which self balances itself to keep operations logarithmic.
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=q4fnJZr8ztY)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=1QSYxIKXXP4)
//! - [W. Fiset's video 3](https://www.youtube.com/watch?v=g4y2h70D6Nk)
//! - [W. Fiset's video 4](https://www.youtube.com/watch?v=tqFZzXkbbGY)
//! - [Wikipedia](https://www.wikiwand.com/en/AVL_tree)

use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node<T: Ord + Debug + PartialEq + Eq + Clone> {
    value: T,
    height: i32,
    balance_factor: i8,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Debug + PartialEq + Eq + Clone> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            height: 0,
            balance_factor: 0,
            left: None,
            right: None,
        }
    }
    /// Updates a node's height and balance factor.
    fn update(&mut self) {
        let left_node_height = self.left.as_ref().map_or(-1, |node| node.height);
        let right_node_height = self.right.as_ref().map_or(-1, |node| node.height);
        // update this node's height
        self.height = std::cmp::max(left_node_height, right_node_height) + 1;
        // update balance factor
        self.balance_factor = (right_node_height - left_node_height) as i8;
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct AvlTree<T: Ord + Debug + PartialEq + Eq + Clone> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: Ord + Debug + PartialEq + Eq + Clone> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
    // the height of a rooted tree is the number of edges between the tree's
    // root and its furthest leaf. This means that a tree containing a single
    // node has a height of 0
    pub fn height(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.height)
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn contains(&self, value: &T) -> bool {
        fn _contains<T: Ord + Debug + Clone>(node: &Option<Box<Node<T>>>, value: &T) -> bool {
            node.as_ref().map_or(false, |node| {
                // compare the current value to the value of the node.
                match value.cmp(&node.value) {
                    // dig into the left subtree
                    Ordering::Less => _contains(&node.left, value),
                    // dig into the right subtree
                    Ordering::Greater => _contains(&node.right, value),
                    Ordering::Equal => true,
                }
            })
        }
        _contains(&self.root, value)
    }
    /// If the value is not found in the AVL tree, insert it and return `true`.
    /// Otherwise, do not insert and return `false`.
    pub fn insert(&mut self, value: T) -> bool {
        fn _insert<T: Ord + Debug + Clone>(node: &mut Option<Box<Node<T>>>, value: T) -> bool {
            let success = match node.as_mut() {
                None => {
                    *node = Some(Box::new(Node::new(value)));
                    return true;
                }
                Some(node) => {
                    // compare the current value to the value of the node.
                    match value.cmp(&node.value) {
                        // insert into the left subtree
                        Ordering::Less => _insert(&mut node.left, value),
                        // insert into the right subtree
                        Ordering::Greater => _insert(&mut node.right, value),
                        Ordering::Equal => false,
                    }
                }
            };
            let node = node.as_mut().unwrap();
            node.update();
            AvlTree::balance(node);

            success
        }
        let success = _insert(&mut self.root, value);
        if success {
            self.len += 1;
        }
        success
    }

    /// re-balance a node if its balance factor is +2 or -2
    #[allow(clippy::branches_sharing_code)]
    fn balance(node: &mut Box<Node<T>>) {
        // left heavy
        match node.balance_factor {
            -2 => {
                // left-left case
                if node.left.as_ref().unwrap().balance_factor < 0 {
                    Self::rotate_right(node);
                } else {
                    // left-right case
                    Self::rotate_left(&mut node.left.as_mut().unwrap());
                    Self::rotate_right(node);
                }
            }
            2 => {
                // right-right case
                if node.right.as_ref().unwrap().balance_factor > 0 {
                    Self::rotate_left(node);
                } else {
                    // right-left case
                    Self::rotate_right(&mut node.right.as_mut().unwrap());
                    Self::rotate_left(node);
                }
            }
            _ => {}
        }
    }

    fn rotate_left(node: &mut Box<Node<T>>) {
        let right_left = node.right.as_mut().unwrap().left.take();
        let new_parent = mem::replace(&mut node.right, right_left).unwrap();
        let new_left_child = mem::replace(node, new_parent);
        node.left = Some(new_left_child);
        node.left.as_mut().unwrap().update();
        node.update();
    }

    fn rotate_right(node: &mut Box<Node<T>>) {
        let left_right = node.left.as_mut().unwrap().right.take();
        let new_parent = mem::replace(&mut node.left, left_right).unwrap();
        let new_right_child = mem::replace(node, new_parent);
        node.right = Some(new_right_child);
        node.right.as_mut().unwrap().update();
        node.update();
    }

    // pub fn remove(&mut self, elem: &T) {
    //     fn _remove<T: Ord + Debug + Clone>(
    //         node: Option<Box<Node<T>>>,
    //         elem: &T,
    //     ) -> Option<Box<Node<T>>> {
    //         match node {
    //             None => None,
    //             Some(mut node) => {
    //                 // compare the current value to the value of the node.
    //                 match elem.cmp(&node.value) {
    //                     // Dig into left subtree, the value we're looking
    //                     // for is smaller than the current value.
    //                     Ordering::Less => node.left = _remove(node.left, elem),
    //                     // Dig into right subtree, the value we're looking
    //                     // for is greater than the current value.
    //                     Ordering::Greater => node.right = _remove(node.right, elem),
    //                     Ordering::Equal => {
    //                         // This is the case with only a right subtree or no subtree at all.
    //                         // In this situation just swap the node we wish to remove
    //                         // with its right child.
    //                         if node.left.is_none() {
    //                             return node.right;
    //                         }
    //                         // This is the case with only a left subtree or
    //                         // no subtree at all. In this situation just
    //                         // swap the node we wish to remove with its left child.
    //                         else if node.right.is_none() {
    //                             return node.left;
    //                         }
    //                         // When removing a node from a binary tree with two links the
    //                         // successor of the node being removed can either be the largest
    //                         // value in the left subtree or the smallest value in the right
    //                         // subtree. As a heuristic, I will remove from the subtree with
    //                         // the greatest hieght in hopes that this may help with balancing.
    //                         else {
    //                             let left = node.left.as_ref().unwrap();
    //                             let right = node.right.as_ref().unwrap();

    //                             // Choose to remove from left subtree
    //                             if left.height >= right.height {
    //                                 // Swap the value of the successor into the node.
    //                                 let successor_value = AvlTree::find_max(&left).clone();
    //                                 node.value = successor_value.clone();

    //                                 // Find the largest node in the left subtree.
    //                                 node.left = _remove(node.left, &successor_value);
    //                             } else {
    //                                 // Swap the value of the successor into the node.
    //                                 let successor_value = AvlTree::find_min(&right).clone();
    //                                 node.value = successor_value.clone();

    //                                 // Go into the right subtree and remove the leftmost node we
    //                                 // found and swapped data with. This prevents us from having
    //                                 // two nodes in our tree with the same value.
    //                                 node.right = _remove(node.right, &successor_value);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 node.update();
    //                 AvlTree::balance(&mut node);
    //                 Some(node)
    //             }
    //         }
    //     }
    //     let root = mem::replace(&mut self.root, None);
    //     self.root = _remove(root, elem);
    // }

    // fn find_min(mut node: &Node<T>) -> &T {
    //     while let Some(next_node) = node.left.as_ref() {
    //         node = &next_node;
    //     }
    //     &node.value
    // }
    // fn find_max(mut node: &Node<T>) -> &T {
    //     while let Some(next_node) = node.right.as_ref() {
    //         node = &next_node;
    //     }
    //     &node.value
    // }
    pub fn remove(&mut self, elem: &T) -> bool {
        fn _remove<T: Ord + Debug + Clone>(
            _node: &mut Option<Box<Node<T>>>,
            elem: &T,
            success: &mut bool,
        ) {
            match _node {
                None => {}
                Some(node) => {
                    match elem.cmp(&node.value) {
                        Ordering::Less => {
                            _remove(&mut node.left, elem, success);
                        }
                        Ordering::Greater => {
                            _remove(&mut node.right, elem, success);
                        }
                        Ordering::Equal => {
                            *success = true;
                            // if the target is found, replace this node with a successor
                            *_node = match (node.left.take(), node.right.take()) {
                                (None, None) => None,
                                (None, Some(right)) => Some(right),
                                (Some(left), None) => Some(left),
                                (Some(left), Some(right)) => {
                                    if left.height >= right.height {
                                        let mut x = AvlTree::remove_max(left);
                                        x.right = Some(right);
                                        Some(x)
                                    } else {
                                        let mut x = AvlTree::remove_min(right);
                                        x.left = Some(left);
                                        Some(x)
                                    }
                                }
                            };
                        }
                    }
                    let mut node = _node.as_mut().unwrap();
                    node.update();
                    AvlTree::balance(&mut node);
                }
            }
        }
        let mut success = false;
        _remove(&mut self.root, elem, &mut success);
        if success {
            self.len -= 1;
        }
        success
    }

    fn remove_min(mut node: Box<Node<T>>) -> Box<Node<T>> {
        fn _remove_min<T: Ord + Debug + PartialEq + Eq + Clone>(
            node: &mut Node<T>,
        ) -> Option<Box<Node<T>>> {
            if let Some(next_node) = node.left.as_mut() {
                let res = _remove_min(next_node);
                if res.is_none() {
                    node.left.take()
                } else {
                    res
                }
            } else {
                None
            }
        }
        _remove_min(&mut node).unwrap_or(node)
    }
    fn remove_max(mut node: Box<Node<T>>) -> Box<Node<T>> {
        fn _remove_max<T: Ord + Debug + PartialEq + Eq + Clone>(
            node: &mut Node<T>,
        ) -> Option<Box<Node<T>>> {
            if let Some(next_node) = node.right.as_mut() {
                let res = _remove_max(next_node);
                if res.is_none() {
                    node.right.take()
                } else {
                    res
                }
            } else {
                None
            }
        }
        _remove_max(&mut node).unwrap_or(node)
    }

    pub fn iter(&self) -> AvlIter<T> {
        if let Some(trav) = self.root.as_ref() {
            AvlIter {
                stack: Some(vec![trav]),
                trav: Some(trav),
            }
        } else {
            AvlIter {
                stack: None,
                trav: None,
            }
        }
    }
}

// TODO: better ergonomics?
pub struct AvlIter<'a, T: 'a + Ord + Debug + PartialEq + Eq + Clone> {
    stack: Option<Vec<&'a Node<T>>>,
    trav: Option<&'a Node<T>>,
}

impl<'a, T: 'a + Ord + Debug + PartialEq + Eq + Clone> Iterator for AvlIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(stack), Some(trav)) = (self.stack.as_mut(), self.trav.as_mut()) {
            while let Some(left) = trav.left.as_ref() {
                stack.push(left);
                *trav = left;
            }

            stack.pop().map(|curr| {
                if let Some(right) = curr.right.as_ref() {
                    stack.push(right);
                    *trav = right;
                }
                &curr.value
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref AVL: AvlTree<i32> = {
            //     5
            //   2   10
            //      7  15
            let mut avl = AvlTree::new();
            assert!(avl.is_empty());
            avl.insert(2);
            avl.insert(5);
            avl.insert(7);
            avl.insert(10);
            avl.insert(15);
            assert_eq!(avl.len(), 5);
            avl
        };
    }

    #[test]
    fn test_avl() {
        let mut avl = AVL.clone();
        assert_eq!(avl.height().unwrap(), 2);
        assert!(avl.contains(&2));
        assert!(avl.contains(&5));
        assert!(avl.contains(&7));
        assert!(avl.contains(&10));
        assert!(avl.contains(&15));
        //     5
        //   2   10
        //      7  15
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.value, 5);
        let n2 = root.left.as_ref().unwrap();
        let n10 = root.right.as_ref().unwrap();
        assert_eq!(n2.value, 2);
        assert_eq!(n10.value, 10);
        assert_eq!(n10.left.as_ref().unwrap().value, 7);
        assert_eq!(n10.right.as_ref().unwrap().value, 15);
        AvlTree::rotate_left(avl.root.as_mut().unwrap());
        //     10
        //   5    15
        // 2   7
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.value, 10);
        let n5 = root.left.as_ref().unwrap();
        let n15 = root.right.as_ref().unwrap();
        assert_eq!(n5.value, 5);
        assert_eq!(n15.value, 15);
        assert_eq!(n5.left.as_ref().unwrap().value, 2);
        assert_eq!(n5.right.as_ref().unwrap().value, 7);
        //     10
        //   2    15
        //     7
        avl.remove(&5);
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.value, 10);
        let n2 = root.left.as_ref().unwrap();
        let n15 = root.right.as_ref().unwrap();
        assert_eq!(n2.value, 2);
        assert_eq!(n15.value, 15);
        assert!(n2.left.as_ref().is_none());
        assert_eq!(n2.right.as_ref().unwrap().value, 7);

        avl.insert(5);
        //     10
        //   5    15
        // 2   7
        AvlTree::rotate_right(avl.root.as_mut().unwrap());
        //     5
        //   2   10
        //      7  15
        assert_eq!(&avl, &*AVL);

        // will not insert an element that's already in the tree
        assert!(!avl.insert(5));
        // will not remove an element that's not in the tree
        assert!(!avl.remove(&100));
    }

    #[test]
    fn test_avl_iter() {
        //     5
        //   2   10
        //      7  15
        let v = AVL.iter().cloned().collect::<Vec<_>>();
        assert_eq!(&v, &[2, 5, 7, 10, 15]);
    }
}
