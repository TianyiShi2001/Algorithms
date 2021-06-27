//! Consecutive edges in a trie (each representing a character) can be coalesced into a string,
//! which allows construction of a suffix tree, in which the number of edges and nodes is reduced.
//! Each node has at least two children. Remember that in a binary tree with `m` leaves there are
//! `m - 1` non-leaf nodes. In a suffix tree there are thus `m` leaves and less than `m - 1` non-
//! leaf nodes, where m is the length of the string. i.e. there are less than `2m - 1` nodes in
//! total. However, the total length of the substrings stored in the tree still grows with `m^2`
//! (coalescing a series of characters into a string really doesn't matter the total amount of
//! space needed to store these characters!) To really solve this problem, see `improved.rs`.
//!
//! - [25:28, in Ben Langmead's lecture on "Suffix tries and trees" (2013)](https://www.youtube.com/watch?v=hLsrPsFHPcQ)

use super::super::suffix_trie::single::{Node as TrieNode, Trie};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Tree {
    pub root: Node,
}

#[derive(Default, Debug)]
pub struct Node {
    pub children: HashMap<Vec<u8>, Box<Node>>,
}

impl Tree {
    #[allow(clippy::vec_init_then_push)]
    pub fn from_trie(trie: &Trie) -> Tree {
        fn process_node(
            trie_node: &TrieNode,
            tree_node_parent: &mut Node,
            buffer: &mut Vec<u8>,
        ) -> Vec<u8> {
            match trie_node.children.len() {
                0 => buffer.to_vec(),
                1 => {
                    let (c, trie_node_child) = trie_node.children.iter().next().unwrap();
                    buffer.push(*c);
                    process_node(trie_node_child, tree_node_parent, buffer)
                }
                _ => {
                    for (&c, child) in &trie_node.children {
                        let mut buffer = Vec::new();
                        buffer.push(c);
                        let mut child_tree_node = Node::default();
                        let link_to_child = process_node(child, &mut child_tree_node, &mut buffer);
                        tree_node_parent
                            .children
                            .insert(link_to_child, Box::new(child_tree_node));
                    }
                    buffer.to_vec()
                }
            }
        }
        let mut slf = Self::default();
        process_node(&trie.root, &mut slf.root, &mut Vec::new());
        slf
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref S1: &'static [u8] = b"abaaba";
        static ref ST1: Trie = Trie::from_str_naive(&S1);
        static ref S2: &'static [u8] = b"aaaaaa";
        static ref ST2: Trie = Trie::from_str_naive(&S2);
        // see ![visual representation of the suffix trie of `abracadabra`](https://i.imgur.com/oes5dxo.png)
    }
    #[test]
    fn from_trie() {
        let tree = Tree::from_trie(&*ST1);
        println!("{:?}", tree);
        let tree = Tree::from_trie(&*ST2);
        println!("{:?}", tree);
    }
}
