//! Often when working with trees we are given them as a graph with undirected edges, however
//! sometimes a better representation is a rooted tree.
//!
//! - Time Complexity: O(V+E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=2FFq2_je7Lg&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=9)

use crate::algo::graph::UnweightedAdjacencyList;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub id: usize,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: vec![],
        }
    }
    pub fn from_adjacency_list(graph: &UnweightedAdjacencyList, root: usize) -> Self {
        fn build_tree_recursive(
            graph: &UnweightedAdjacencyList,
            node_id: usize,
            parent_id: Option<usize>,
        ) -> TreeNode {
            let mut node = TreeNode::new(node_id);
            for &child_id in &graph[node_id] {
                if let Some(id) = parent_id {
                    if id == child_id {
                        continue;
                    }
                }
                let child_node = build_tree_recursive(graph, child_id, Some(node_id));
                node.children.push(child_node);
            }
            node
        }
        build_tree_recursive(graph, root, None)
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
        //         6
        //      2  7  8
        //    1  3
        //  0   4 5
        println!("{:?}", &tree);
        assert_eq!(
            tree,
            TreeNode {
                id: 6,
                children: vec![
                    TreeNode {
                        id: 2,
                        children: vec![
                            TreeNode {
                                id: 1,
                                children: vec![TreeNode::new(0)]
                            },
                            TreeNode {
                                id: 3,
                                children: vec![TreeNode::new(4), TreeNode::new(5)]
                            }
                        ]
                    },
                    TreeNode::new(7),
                    TreeNode::new(8)
                ]
            }
        );
        let tree = TreeNode::from_adjacency_list(&graph, 3);
        // Rooted at 3 the tree should look like:
        //       3
        //    2  4  5
        //  6  1
        // 7 8  0
        println!("{:?}", &tree);
        assert_eq!(
            tree,
            TreeNode {
                id: 3,
                children: vec![
                    TreeNode {
                        id: 2,
                        children: vec![
                            TreeNode {
                                id: 1,
                                children: vec![TreeNode::new(0)]
                            },
                            TreeNode {
                                id: 6,
                                children: vec![TreeNode::new(7), TreeNode::new(8)]
                            }
                        ]
                    },
                    TreeNode::new(4),
                    TreeNode::new(5)
                ]
            }
        );
    }
}
