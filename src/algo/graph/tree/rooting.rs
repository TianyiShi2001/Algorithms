//! Often when working with trees we are given them as a graph with undirected edges, however
//! sometimes a better representation is a rooted tree. This module (and the [`with_parent`] submodule) contains
//! implementations to build a tree from its adjacency list representation with a given root id.
//!
//! - Time Complexity: O(V+E)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=2FFq2_je7Lg&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=9)

use super::Node;
use crate::algo::graph::UnweightedAdjacencyList;

impl Node {
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
        build_tree_recursive(graph, root, None)
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
        let root = Node::from_adjacency_list(&graph, 6);
        // Rooted at 6 the root should look like:
        //         6
        //      2  7  8
        //    1  3
        //  0   4 5
        println!("{:?}", &root);
        assert_eq!(
            root,
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
        let root = Node::from_adjacency_list(&graph, 3);
        // Rooted at 3 the root should look like:
        //       3
        //    2  4  5
        //  6  1
        // 7 8  0
        println!("{:?}", &root);
        assert_eq!(
            root,
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

pub mod with_parent {
    use crate::algo::graph::tree::with_parent::*;
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

    impl UnsafeTreeNode {
        pub fn from_adjacency_list(graph: &UnweightedAdjacencyList, root: usize) -> Box<Self> {
            fn build_tree_recursive(
                graph: &UnweightedAdjacencyList,
                mut node: Box<UnsafeTreeNode>,
            ) -> Box<UnsafeTreeNode> {
                for &child_id in &graph[node.id] {
                    if !node.parent.is_null() && unsafe { (*node.parent).id == child_id } {
                        continue;
                    }
                    let child_node = build_tree_recursive(
                        graph,
                        Box::new(UnsafeTreeNode::new(
                            child_id,
                            node.as_ref() as *const UnsafeTreeNode,
                        )),
                    );
                    node.children.push(child_node);
                }
                node
            }
            build_tree_recursive(graph, Box::new(UnsafeTreeNode::new(root, std::ptr::null())))
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
            // Rooted at 6 the root should look like:
            //         6
            //      2  7  8
            //    1  3
            //  0   4 5
            let _node0 = Node::new(0, None);
            let _node1 = Node::new(1, None);
            let _node2 = Node::new(2, None);
            let _node3 = Node::new(3, None);
            let _node4 = Node::new(4, None);
            let _node5 = Node::new(5, None);
            let _node5 = Node::new(5, None);
            let _node6 = Node::new(6, None);
            let _node7 = Node::new(7, None);
            let _node8 = Node::new(8, None);
            Node::add_child(&_node6, &_node2);
            Node::add_child(&_node6, &_node7);
            Node::add_child(&_node6, &_node8);
            Node::add_child(&_node2, &_node1);
            Node::add_child(&_node2, &_node3);
            Node::add_child(&_node1, &_node0);
            Node::add_child(&_node3, &_node4);
            Node::add_child(&_node3, &_node5);

            let root1 = _node6;

            assert_eq!(root, root1);
        }

        #[test]
        fn test_tree_rooting_unsafe() {
            let mut graph = UnweightedAdjacencyList::with_size(9);
            graph.add_undirected_edge(0, 1);
            graph.add_undirected_edge(2, 1);
            graph.add_undirected_edge(2, 3);
            graph.add_undirected_edge(3, 4);
            graph.add_undirected_edge(5, 3);
            graph.add_undirected_edge(2, 6);
            graph.add_undirected_edge(6, 7);
            graph.add_undirected_edge(6, 8);
            let tree = UnsafeTreeNode::from_adjacency_list(&graph, 6);
            // Rooted at 6 the tree should look like:
            //           6
            //      2    7     8
            //    1   3
            //  0    4 5
            println!("{:?}", &tree);
            // layer 1
            let UnsafeTreeNode {
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

            let tree = UnsafeTreeNode::from_adjacency_list(&graph, 3);
            // Rooted at 3 the tree should look like:
            //               3
            //     2         4        5
            //  6     1
            // 7 8    0
            let UnsafeTreeNode {
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
}
