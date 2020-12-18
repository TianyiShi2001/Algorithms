//! Often when working with trees we are given them as a graph with undirected edges, however
//! sometimes a better representation is a rooted tree.
//!
//! - Time Complexity: O(V+E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=2FFq2_je7Lg&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=9)

use super::{Node, Tree};
use crate::algo::graph::UnweightedAdjacencyList;

impl Tree {
    pub fn from_adjacency_list(graph: &UnweightedAdjacencyList, root: usize) -> Self {
        fn build_tree_recursive(
            graph: &UnweightedAdjacencyList,
            node_id: usize,
            parent_id: Option<usize>,
        ) -> Node {
            let mut node = Node::new(node_id);
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
        let root = build_tree_recursive(graph, root, None);
        Self {
            root,
            size: graph.node_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_rooting() {
        let graph = UnweightedAdjacencyList::new_undirected(
            9,
            &[
                [0, 1],
                [2, 1],
                [2, 3],
                [3, 4],
                [5, 3],
                [2, 6],
                [6, 7],
                [6, 8],
            ],
        );
        let tree = Tree::from_adjacency_list(&graph, 6).root;
        // Rooted at 6 the tree should look like:
        //         6
        //      2  7  8
        //    1  3
        //  0   4 5
        println!("{:?}", &tree);
        assert_eq!(
            tree,
            Node {
                id: 6,
                children: vec![
                    Node {
                        id: 2,
                        children: vec![
                            Node {
                                id: 1,
                                children: vec![Node::new(0)]
                            },
                            Node {
                                id: 3,
                                children: vec![Node::new(4), Node::new(5)]
                            }
                        ]
                    },
                    Node::new(7),
                    Node::new(8)
                ]
            }
        );
        let tree = Tree::from_adjacency_list(&graph, 3).root;
        // Rooted at 3 the tree should look like:
        //       3
        //    2  4  5
        //  6  1
        // 7 8  0
        println!("{:?}", &tree);
        assert_eq!(
            tree,
            Node {
                id: 3,
                children: vec![
                    Node {
                        id: 2,
                        children: vec![
                            Node {
                                id: 1,
                                children: vec![Node::new(0)]
                            },
                            Node {
                                id: 6,
                                children: vec![Node::new(7), Node::new(8)]
                            }
                        ]
                    },
                    Node::new(4),
                    Node::new(5)
                ]
            }
        );
    }
}

pub mod rc {
    use crate::algo::graph::tree::rc::*;
    use crate::algo::graph::UnweightedAdjacencyList;

    impl Node {
        pub fn from_adjacency_list(
            graph: &UnweightedAdjacencyList,
            root: usize,
        ) -> Rc<RefCell<Node>> {
            fn build_tree_recursive(
                graph: &UnweightedAdjacencyList,
                node_id: usize,
                parent: Option<&Rc<RefCell<Node>>>,
            ) -> Rc<RefCell<Node>> {
                let node = Node::new(node_id, parent);
                for &child_id in &graph[node_id] {
                    if let Some(parent_node) = parent {
                        if parent_node.borrow().id == child_id {
                            continue;
                        }
                    }
                    let child_node = build_tree_recursive(graph, child_id, Some(&node));
                    node.borrow_mut().children.push(child_node);
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
        fn test_tree_rooting_rc() {
            let graph = UnweightedAdjacencyList::new_undirected(
                9,
                &[
                    [0, 1],
                    [2, 1],
                    [2, 3],
                    [3, 4],
                    [5, 3],
                    [2, 6],
                    [6, 7],
                    [6, 8],
                ],
            );
            let root = Node::from_adjacency_list(&graph, 6);
            // Rooted at 6 the tree should look like:
            //         6
            //      2  7  8
            //    1  3
            //  0   4 5
            let node0 = Node::new(0, None);
            let node1 = Node::new(1, None);
            let node2 = Node::new(2, None);
            let node3 = Node::new(3, None);
            let node4 = Node::new(4, None);
            let node5 = Node::new(5, None);
            let node5 = Node::new(5, None);
            let node6 = Node::new(6, None);
            let node7 = Node::new(7, None);
            let node8 = Node::new(8, None);
            Node::add_child(&node6, &node2);
            Node::add_child(&node6, &node7);
            Node::add_child(&node6, &node8);
            Node::add_child(&node2, &node1);
            Node::add_child(&node2, &node3);
            Node::add_child(&node1, &node0);
            Node::add_child(&node3, &node4);
            Node::add_child(&node3, &node5);

            let root1 = node6;

            assert_eq!(root, root1);
        }
    }
}
