//! A k-dimensional tree data structure.
//! A knn algorithm implemented with k-d tree can be found in [`crate::ml::knn`]
//!
//! # Recommended Prerequisites
//!
//! - quadtree

use num_traits::Float;
use std::fmt::Debug;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point<T: Clone + Float, const DIM: usize>(pub [T; DIM]);

impl<T: Clone + Float, const DIM: usize> std::ops::Index<usize> for Point<T, DIM> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T: Clone + Float, const DIM: usize> std::ops::IndexMut<usize> for Point<T, DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Clone + Float, const DIM: usize> From<[T; DIM]> for Point<T, DIM> {
    fn from(input: [T; DIM]) -> Self {
        Self(input)
    }
}

#[derive(Debug)]
pub struct Node<T: Clone + Float, const DIM: usize> {
    pub pivot: Point<T, DIM>,
    pub left: Option<Box<Node<T, DIM>>>,
    pub right: Option<Box<Node<T, DIM>>>,
}

impl<T: Clone + Float, const DIM: usize> Node<T, DIM> {
    pub fn new<P: Into<Point<T, DIM>>>(point: P) -> Self {
        Self {
            pivot: point.into(),
            left: None,
            right: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct KdTree<T: Clone + Float, const DIM: usize> {
    pub root: Option<Box<Node<T, DIM>>>,
}

impl<T: Clone + Float + Debug, const DIM: usize> KdTree<T, DIM> {
    pub fn dim() -> usize {
        DIM
    }
    pub fn from_slice(points: &mut [Point<T, DIM>]) -> Self {
        fn build_node<T: Clone + Float, const DIM: usize>(
            points: &mut [Point<T, DIM>],
            depth: usize,
        ) -> Option<Box<Node<T, DIM>>> {
            let d = depth % DIM;
            points.sort_unstable_by(|a, b| a[d].partial_cmp(&b[d]).unwrap());
            let mut mid = points.len() / 2;
            let val = &points[mid][d];
            // ensure that points to the right of the pivot are strictly greater
            for i in mid + 1..points.len() {
                if points[i][d] != *val {
                    break;
                } else {
                    mid = i;
                }
            }
            let pivot = points[mid].clone();
            let (l, r) = points.split_at_mut(mid);

            Some(Box::new(Node {
                pivot,
                left: if l.is_empty() {
                    None
                } else {
                    build_node(l, depth + 1)
                },
                right: if r.len() == 1 {
                    None
                } else {
                    build_node(&mut r[1..], depth + 1)
                },
            }))
        }
        let root = build_node(points, 0);
        Self { root }
    }

    pub fn contains(&self, point: &Point<T, DIM>) -> bool {
        let mut depth = 0;
        let mut next = self.root.as_ref();
        while let Some(curr) = next {
            let curr_point = &curr.pivot;
            if curr_point == point {
                return true;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                curr.left.as_ref()
            } else {
                curr.right.as_ref()
            };
            depth += 1;
        }
        false
    }
    /// Insert a point into the tree.
    ///
    /// Inserting elements one by one is likely to cause the tree to become inbalanced.
    /// Prefer using `from_slice` to construct the tree.
    pub fn insert(&mut self, point: Point<T, DIM>) -> bool {
        let mut depth = 0;
        let mut next = &mut self.root;
        while let Some(curr) = next {
            let curr_point = &curr.pivot;
            if *curr_point == point {
                return false;
            }
            let d = depth % DIM;
            next = if point[d] <= curr_point[d] {
                &mut curr.left
            } else {
                &mut curr.right
            };
            depth += 1;
        }
        *next = Some(Box::new(Node::new(point)));
        true
    }
}
