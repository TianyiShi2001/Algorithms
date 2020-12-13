//! Implementation of finding the Lowest Common Ancestor (LCA) of a tree. This impl first finds an
//! Euler tour from the root node which visits all the nodes in the tree. The node height values
//! obtained from the Euler tour can then be used in combination with a sparse table to find the LCA
//! in O(1).
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=sD1IoalFomA)

use super::rooting::TreeNode;

pub struct Tree {
    pub root: TreeNode,
    pub size: usize,
}

pub struct LcaSolver {
    sparse_table: MinSparseTable,
    node_order: Vec<usize>,
    // The last occurrence mapping. This mapping keeps track of the last occurrence of a TreeNode in
    // the Euler tour for easy indexing.
    last: Vec<usize>,
}

impl LcaSolver {
    pub fn new(tree: &Tree) -> Self {
        let mut node_depth = vec![0usize; tree.size * 2 - 1]; // Vec::<usize>::new();
        let mut node_order = vec![0usize; tree.size * 2 - 1]; // Vec::<usize>::new();
        let mut last = vec![0usize; tree.size];
        let mut tour_index = 0;

        let mut visit = |node: usize, depth: usize| {
            node_order[tour_index] = node;
            node_depth[tour_index] = depth;
            last[node] = tour_index;
            tour_index += 1;
        };

        //dfs
        let mut stack = vec![(&tree.root, 0usize)];
        let mut visited = vec![false; tree.size];
        while let Some((node, depth)) = stack.pop() {
            visit(node.id, depth);
            if !visited[node.id] {
                visited[node.id] = true;
                for child in &node.children {
                    stack.push((node, depth)); // revisit the current node after visiting each child
                    stack.push((child, depth + 1));
                }
            }
        }

        let sparse_table = MinSparseTable::new(&node_depth);
        Self {
            sparse_table,
            node_order,
            last,
        }
    }
    pub fn lca(&self, a: usize, b: usize) -> usize {
        let (a, b) = (self.last[a], self.last[b]);
        let (l, r) = if a < b { (a, b) } else { (b, a) };
        let idx = self.sparse_table.query_index(l, r);
        self.node_order[idx]
    }
}

pub struct MinSparseTable {
    // The sparse table values.
    min_depth: Vec<Vec<Option<usize>>>,
    // Index Table associated with the values in the sparse table.
    index: Vec<Vec<Option<usize>>>,
    log2: Vec<usize>,
}

impl MinSparseTable {
    pub fn new(node_depth: &[usize]) -> Self {
        let n = node_depth.len();
        let log2 = Self::build_log2(n);
        let m = log2[n];
        let mut min_depth = vec![vec![None; n]; m + 1];
        let mut index = vec![vec![None; n]; m + 1];
        for (i, &depth) in node_depth.iter().enumerate() {
            min_depth[0][i] = Some(depth);
            index[0][i] = Some(i);
        }
        // Build sparse table combining the values of the previous intervals.
        for i in 1..=m {
            for j in 0..=(n - (1 << i)) {
                let left_interval = min_depth[i - 1][j];
                let right_interval = min_depth[i - 1][j + (1 << (i - 1))];
                // Propagate the index of the best value
                if left_interval <= right_interval {
                    min_depth[i][j] = left_interval;
                    index[i][j] = index[i - 1][j];
                } else {
                    min_depth[i][j] = right_interval;
                    index[i][j] = index[i - 1][j + (1 << (i - 1))];
                }
            }
        }
        Self {
            min_depth,
            index,
            log2,
        }
    }
    fn build_log2(n: usize) -> Vec<usize> {
        let mut log2 = vec![0usize; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }
        log2
    }
    fn query_index(&self, l: usize, r: usize) -> usize {
        let len = r - l + 1;
        let i = self.log2[len];
        let left_interval = self.min_depth[i][l];
        let right_interval = self.min_depth[i][r - (1 << i) + 1];
        if left_interval <= right_interval {
            self.index[i][l]
        } else {
            self.index[i][r - (i << i) + 1]
        }
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::graph::UnweightedAdjacencyList;
    #[test]
    fn test_tree_lowest_commmon_ancestor() {
        let tree = UnweightedAdjacencyList::new_undirected(
            17,
            &[
                [0, 1],
                [0, 2],
                [1, 3],
                [1, 4],
                [2, 5],
                [2, 6],
                [2, 7],
                [3, 8],
                [3, 9],
                [5, 10],
                [5, 11],
                [7, 12],
                [7, 13],
                [11, 14],
                [11, 15],
                [11, 16],
            ],
        );
        let size = tree.vertices_count();
        let root = TreeNode::from_adjacency_list(&tree, 0);
        let tree = Tree { root, size };
        let lca_solver = LcaSolver::new(&tree);
        assert_eq!(lca_solver.lca(14, 13), 2);
        assert_eq!(lca_solver.lca(9, 11), 0);
        assert_eq!(lca_solver.lca(12, 12), 12);
    }
}
