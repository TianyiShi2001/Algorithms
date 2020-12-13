//! An improved implementation of Tree Rooting, in which each node has an pointer to its parent.

use crate::algo::graph::UnweightedAdjacencyList;

#[derive(Debug, Eq, PartialEq)]
#[allow(clippy::vec_box)]
pub struct TreeNode {
    id: usize,
    parent: *const TreeNode,
    children: Vec<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(id: usize, parent: *const TreeNode) -> Self {
        Self {
            id,
            parent,
            children: vec![],
        }
    }
    pub fn from_adjacency_list(graph: &UnweightedAdjacencyList, root: usize) -> Box<Self> {
        fn build_tree_recursive(
            graph: &UnweightedAdjacencyList,
            mut node: Box<TreeNode>,
        ) -> Box<TreeNode> {
            for &child_id in &graph[node.id] {
                if !node.parent.is_null() && unsafe { (*node.parent).id == child_id } {
                    continue;
                }
                let child_node = build_tree_recursive(
                    graph,
                    Box::new(TreeNode::new(child_id, node.as_ref() as *const TreeNode)),
                );
                node.children.push(child_node);
            }
            node
        }
        build_tree_recursive(graph, Box::new(TreeNode::new(root, std::ptr::null())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_rooting() {
        let mut graph = UnweightedAdjacencyList::with_size(9);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(2, 1);
        graph.add_undirected_edge(2, 3);
        graph.add_undirected_edge(3, 4);
        graph.add_undirected_edge(5, 3);
        graph.add_undirected_edge(2, 6);
        graph.add_undirected_edge(6, 7);
        graph.add_undirected_edge(6, 8);
        let tree = TreeNode::from_adjacency_list(&graph, 6);
        // Rooted at 6 the tree should look like:
        //           6
        //      2    7     8
        //    1   3
        //  0    4 5
        println!("{:?}", &tree);
        // layer 1
        let TreeNode {
            id,
            parent,
            children,
        } = *tree;
        assert_eq!(id, 6);
        assert!(parent.is_null());
        assert_eq!(children.len(), 3);
        let node2 = &children[0];
        assert_eq!(node2.id, 2);
        assert_eq!((unsafe { &*node2.parent }).id, 6);
        assert_eq!(node2.children.len(), 2);

        let tree = TreeNode::from_adjacency_list(&graph, 3);
        // Rooted at 3 the tree should look like:
        //               3
        //     2         4        5
        //  6     1
        // 7 8    0
        let TreeNode {
            id,
            parent,
            children,
        } = *tree;
        assert_eq!(id, 3);
        assert!(parent.is_null());
        assert_eq!(children.len(), 3);
        let node2 = &children[0];
        assert_eq!(node2.id, 2);
        assert_eq!((unsafe { &*node2.parent }).id, 3);
        assert_eq!(node2.children.len(), 2);
    }
}
