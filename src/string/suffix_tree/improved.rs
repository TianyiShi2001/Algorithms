//! Our naive implementation of suffix tree reduces the total number of nodes compared to
//! the equivalent suffix trie, but did not really reduce overall memory usage.
//! Instead of using strings to represent edges, we can use two pointers to the original
//! string instead, so that each edge is now 2-word-long (2 * pointer size).
//! The idiomatic way to to this in Rust is to use a slice (instead of raw pointers),
//! which help to ensure validity of pointers (preventing dangling pointers).
//!
//! - [30:00, in Ben Langmead's lecture on "Suffix tries and trees" (2013)](https://www.youtube.com/watch?v=hLsrPsFHPcQ)

// use super::super::suffix_trie::single::{Node as TrieNode, Trie};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Tree<'a> {
    pub root: Node<'a>,
}

#[derive(PartialEq, Eq)]
pub enum Node<'a> {
    Branches(HashMap<&'a [u8], Box<Node<'a>>>),
    Leaf(usize), // offset
}

use std::fmt;
impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Branches(children) => {
                for (&edge, child) in children.iter() {
                    let mut edge = unsafe { std::str::from_utf8_unchecked(edge) };
                    if edge.is_empty() {
                        edge = "(empty}"
                    }
                    writeln!(f, "{}: {:?}", edge, child)?;
                }
            }
            Self::Leaf(_offset) => {
                write!(f, "$")?;
            }
        }
        Ok(())
    }
}

impl<'a> Default for Node<'a> {
    fn default() -> Self {
        Self::Branches(HashMap::new())
    }
}

impl<'a> Tree<'a> {
    /// A naive method to construct the suffix tree from a string.
    ///
    /// First, build a single-edge tree representing only the longest suffix, then
    /// augment to include the 2nd-longest, then augment to include 3rd-longest, etc.
    ///
    /// - Time complexity: $O(m^2)$
    /// - Space complexity: $O(m)$
    ///
    /// (There is another naive method, which is to build a suffix trie first, then
    /// coalesce non-branching paths and relabel edges. This is too inefficient that
    /// I won't bother implementing it.)
    pub fn from_str_naive(s: &'a [u8]) -> Self {
        let mut root = Node::default();
        for offset in 0..s.len() {
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
                        let branches = Box::new(Node::Branches(branches));
                        unsafe { &mut *children_ptr }.insert(upper, branches);
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
            if descendents_leaves >= n {
                let substr_len = buffer.iter().map(|substr| substr.len()).sum::<usize>();
                if substr_len > *longest_len {
                    *longest = buffer.clone();
                    *longest_len = substr_len;
                }
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
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;
    use lazy_static::lazy_static;
    use maplit::hashmap;
    lazy_static! {
        static ref S1: &'static [u8] = b"abracadabra";
        static ref ST1: Tree<'static> = Tree::from_str_naive(&S1);
        static ref ST1_EXPECTED: Tree<'static> = Tree {
            root: Node::Branches(hashmap!{
                &S1[0..1] => Box::new(Node::Branches( // a
                    hashmap! {
                        &S1[10..] => Box::new(Node::Leaf(10)),
                        &S1[4..] => Box::new(Node::Leaf(3)),
                        &S1[1..4] => Box::new(Node::Branches(
                            hashmap! {
                                &S1[10..] => Box::new(Node::Leaf(7)),
                                &S1[4..] => Box::new(Node::Leaf(0)), // cadabra ==> abracadabra
                            }
                        ))
                    }
                )),
                &S1[1..4] => Box::new(Node::Branches(hashmap!{
                   // &S1[..]
                }))
            })
        };
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
        assert_eq!(ST1.longest_repeated_substr(6), Vec::<u8>::new());
    }
}
