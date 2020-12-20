//! UnionFind/Disjoint Set data structure implementation. This code was inspired by the union find
//! implementation found in 'Algorithms Fourth Edition' by Robert Sedgewick and Kevin Wayne.
//!
//! # Resources
//!
//! - [W. Fiset's video series](https://www.youtube.com/watch?v=ibjEGG7ylHk)

use std::cmp::Ordering::*;

/// Vector-based union-find representing a set of disjoint sets.
#[derive(Clone)]
pub struct UnionFind {
    /// `parents[i]` points to the parent of `i`.
    /// If `id[i] == i` then `i` is a root node.
    parents: Vec<usize>,
    /// Keeps track of the size (number of elements) in each component
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn with_size(size: usize) -> Self {
        UnionFind {
            // parents are initialised to invalid values
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }

    pub fn extend(&mut self, size: usize) {
        let n = self.len();
        for i in n..n + size {
            self.parents.push(i);
            self.sizes.push(1);
        }
    }

    /// Try to union two sets.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let rep_a = self.find(a);
        let rep_b = self.find(b);

        if rep_a == rep_b {
            return false;
        }

        let size_a = self.sizes[rep_a];
        let size_b = self.sizes[rep_b];

        match size_a.cmp(&size_b) {
            Greater => self.set_parent(rep_b, rep_a),
            Less | Equal => self.set_parent(rep_a, rep_b),
        }

        true
    }

    /// Finds the representative element for the given element’s set.
    pub fn find(&mut self, mut element: usize) -> usize {
        // Find the root of the component/set
        let mut root = element;
        while root != self.parents[root] {
            root = self.parents[root]
        }

        // Compress the path leading back to the root.
        // Doing this operation is called "path compression"
        // and is what gives us amortized time complexity.
        while element != root {
            let next = self.parents[element];
            self.parents[element] = root;
            element = next;
        }

        root
    }

    pub fn component_size(&mut self, element: usize) -> usize {
        let rep = self.find(element);
        self.sizes[rep]
    }

    pub fn in_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn parent(&self, element: usize) -> usize {
        self.parents[element]
    }

    pub fn set_parent(&mut self, element: usize, parent: usize) {
        self.sizes[parent] += self.sizes[element];
        self.parents[element] = parent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::with_size(7);
        uf.extend(1);
        assert_eq!(uf.len(), 8);
        assert!(!uf.is_empty());
        assert!(uf.union(0, 1));
        assert!(uf.union(1, 2));
        assert!(uf.union(4, 3));
        assert!(uf.union(3, 2));
        assert!(!uf.union(0, 3));

        assert!(uf.in_same_set(0, 1));
        assert!(uf.in_same_set(0, 2));
        assert!(uf.in_same_set(0, 3));
        assert!(uf.in_same_set(0, 4));
        assert!(!uf.in_same_set(0, 5));
        assert_eq!(uf.component_size(0), 5);
        assert_eq!(uf.component_size(1), 5);
        assert_eq!(uf.component_size(2), 5);
        assert_eq!(uf.component_size(3), 5);
        assert_eq!(uf.component_size(4), 5);

        assert_eq!(uf.component_size(5), 1);
        assert_eq!(uf.component_size(6), 1);

        uf.union(5, 3);
        assert!(uf.in_same_set(0, 5));

        uf.union(6, 7);
        assert!(uf.in_same_set(6, 7));
        assert!(!uf.in_same_set(5, 7));

        uf.union(0, 7);
        assert!(uf.in_same_set(5, 7));
    }
}
