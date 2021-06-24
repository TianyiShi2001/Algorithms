//! Our naive implementation of suffix tree reduces the total number of nodes compared to
//! the equivalent suffix trie, but did not really reduce overall memory usage.
//! Instead of using strings to represent edges, we can use two pointers to the original
//! string instead, so that each edge is now 2-word-long (2 * pointer size).
//! The idiomatic way to to this in Rust is to use a slice (instead of raw pointers),
//! which help to ensure validity of pointers (preventing dangling pointers).
//!
//! - [30:00, in Ben Langmead's lecture on "Suffix tries and trees" (2013)](https://www.youtube.com/watch?v=hLsrPsFHPcQ)

// use super::super::suffix_trie::single::{Node as TrieNode, Trie};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Tree<'a> {
    pub root: Node<'a>,
}

#[derive(Debug)]
pub enum Node<'a> {
    Branches(HashMap<&'a [u8], Box<Node<'a>>>),
    Leaf(usize), // offset
}

impl<'a> Default for Node<'a> {
    fn default() -> Self {
        Self::Branches(HashMap::new())
    }
}

// impl<'a> Node<'a> {
//     fn dummy() -> Self {
//         Self::Leaf(0)
//     }
// }

// #[derive(Default, Debug)]
// pub struct Node<'a> {
//     pub children: HashMap<&'a [u8], Box<Node<'a>>>,
// }

impl<'a> Tree<'a> {
    /// A naive method to construct the suffix tree from a string.
    ///
    /// First, build a single-edge tree representing only the longest suffix, then
    /// augment to include the 2nd-longest, then augment to include 3rd-longest, etc.
    pub fn from_str_naive(s: &'a [u8]) -> Self {
        //-> Self {
        let mut root = Node::default();
        for offset in 0..s.len() {
            // let suffix_len = s.len() - offset;
            let mut suffix = &s[offset..];
            let mut node = &mut root;
            'outer: while let Node::Branches(children) = node {
                let children_ptr = children as *mut HashMap<&'a [u8], Box<Node<'a>>>;
                for (edge, child) in unsafe { &mut *children_ptr }.iter_mut() {
                    let mut n_match = 0;
                    for (&a, &b) in suffix.iter().zip(edge.iter()) {
                        if a == b {
                            n_match += 1;
                        } else {
                            break;
                        }
                    }
                    if n_match == 0 {
                        continue; // to search for the next edge
                    }
                    if n_match == edge.len() {
                        //      .            fully matches the edge
                        //      a                  <------>                     a
                        //      b                  <------>                     b
                        //      X  <--  remaining suffix continues to    <-----  e
                        //     c e      be matched against node X        <-----   g
                        //    d   f
                        //   .     .
                        //    tree                                             suffix
                        suffix = &suffix[n_match..];
                        node = child;
                        continue 'outer;
                        // there is no more than 1 edge that fully or partially matches the suffix, so no
                        // need to check the remaining edges.
                    } else if n_match > 0 {
                        let (upper, lower_original) = edge.split_at(n_match);
                        let lower_suffix = &suffix[n_match..];
                        let mut branches = HashMap::new();
                        //      o                            o
                        //      |                    upper   |
                        //      |                            |
                        // edge |       =======>             o
                        //      |                           / \
                        //      |        lower_original    /   \   lower_suffix
                        //      o  <- child_original ->   o     o <- Leaf(offset)
                        let child_original = unsafe { &mut *children_ptr }.remove(edge).unwrap();
                        branches.insert(lower_original, child_original);
                        branches.insert(lower_suffix, Box::new(Node::Leaf(offset)));
                        unsafe { &mut *children_ptr }
                            .insert(upper, Box::new(Node::Branches(branches)));
                        // we have finished inserting the suffix, so break the outer loop
                        break 'outer;
                    }
                }
                // we reach here when no edges at least partially matches the suffix,
                // so what we want to do is to insert the entire suffix as a child of the
                // parent
                //      o                            o
                //      |                            |\
                //      |                 edge       | \      newly inserted
                // edge |   =======>  (unmodified)   |  \     (suffix)
                //      |                            |   \
                //      |                            |    \
                //      o                            o     o
                children.insert(suffix, Box::new(Node::Leaf(offset)));
                break;
            }
        }
        Self { root }
    }

    pub fn longest_repeated_substr(&self, n: usize) -> Vec<u8> {
        fn dfs<'a>(
            node: &'a Node,
            buffer: &mut Vec<&'a [u8]>,
            longest: &mut Vec<&'a [u8]>,
            longest_len: &mut usize,
            n: usize,
        ) -> usize {
            let mut descendents_leaves = 0;
            match node {
                Node::Branches(children) => {
                    for (&c, child) in children.iter() {
                        buffer.push(c);
                        descendents_leaves += dfs(child, buffer, longest, longest_len, n);
                        buffer.pop().unwrap();
                    }
                }
                Node::Leaf(_) => descendents_leaves += 1,
            }
            if descendents_leaves >= n
                && buffer.iter().map(|substr| substr.len()).sum::<usize>() > *longest_len
            {
                *longest = buffer.clone();
                *longest_len = buffer.len();
            }
            descendents_leaves
        }
        let mut longest = Vec::new();
        dfs(&self.root, &mut Vec::new(), &mut longest, &mut 0, n);
        longest
            .into_iter()
            .flat_map(|x| x.into_iter().copied())
            .collect()
    }
    // pub fn from_trie(trie: &Trie) -> Tree {
    //     fn process_node(
    //         trie_node: &TrieNode,
    //         tree_node_parent: &mut Node,
    //         buffer: &mut Vec<u8>,
    //     ) -> Vec<u8> {
    //         match trie_node.children.len() {
    //             0 => buffer.to_vec(),
    //             1 => {
    //                 let (c, trie_node_child) = trie_node.children.iter().next().unwrap();
    //                 buffer.push(*c);
    //                 process_node(trie_node_child, tree_node_parent, buffer)
    //             }
    //             _ => {
    //                 for (&c, child) in &trie_node.children {
    //                     let mut buffer = Vec::new();
    //                     buffer.push(c);
    //                     let mut child_tree_node = Node::default();
    //                     let link_to_child = process_node(child, &mut child_tree_node, &mut buffer);
    //                     tree_node_parent
    //                         .children
    //                         .insert(link_to_child, Box::new(child_tree_node));
    //                 }
    //                 buffer.to_vec()
    //             }
    //         }
    //     }
    //     let mut slf = Self::default();
    //     process_node(&trie.root, &mut slf.root, &mut Vec::new());
    //     slf
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref S1: &'static [u8] = b"abracadabra";
        static ref ST1: Tree<'static> = Tree::from_str_naive(&S1);
        // see ![visual representation of the suffix trie of `abracadabra`](https://i.imgur.com/oes5dxo.png)
    }
    // #[test]
    // fn contains_substr_1() {
    //     assert!(ST1.contains_substr(b"abra"));
    //     assert!(ST1.contains_substr(b"brac"));
    //     assert!(ST1.contains_substr(b"abra"));
    //     assert!(!ST1.contains_substr(b"abrc"));
    //     assert!(!ST1.contains_substr(b"arac"));
    // }

    // #[test]
    // fn occurence_1() {
    //     assert_eq!(ST1.occurence(b"af"), 0);
    //     assert_eq!(ST1.occurence(b"abrac"), 1);
    //     assert_eq!(ST1.occurence(b"abra"), 2);
    //     assert_eq!(ST1.occurence(b"a"), 5);
    // }

    #[test]
    fn longest_repeated_substr_1() {
        assert_eq!(ST1.longest_repeated_substr(2), b"abra".to_vec());
        assert_eq!(ST1.longest_repeated_substr(3), vec![b'a']);
        assert_eq!(ST1.longest_repeated_substr(5), vec![b'a']);
        assert_eq!(ST1.longest_repeated_substr(6), vec![]);
    }
}
