/// # Resources
///
/// - [Ben Langmead's lecture on "Suffix tries and trees" (2013)](https://www.youtube.com/watch?v=hLsrPsFHPcQ)
use std::collections::HashMap;

pub struct NaiveSuffixTrie {
    pub root: NaiveSuffixTrieNode,
}

#[derive(Default)]
pub struct NaiveSuffixTrieNode {
    pub children: HashMap<u8, Box<NaiveSuffixTrieNode>>,
}

impl NaiveSuffixTrie {
    pub fn from_str_naive(s: &[u8]) -> Self {
        let n = s.len();
        let mut root = NaiveSuffixTrieNode::default();
        for i in 0..n {
            let suffix = s[i..n].iter().chain(std::iter::once(&b'$'));
            let mut node = &mut root as *mut NaiveSuffixTrieNode;
            for c in suffix {
                node = &mut **(unsafe { &mut *node }
                    .children
                    .entry(*c)
                    .or_insert(Box::new(NaiveSuffixTrieNode::default())));
            }
        }
        Self { root }
    }

    /// Checks whether a substring, `query`, is contained in the string.
    pub fn contains_substr(&self, query: &[u8]) -> bool {
        let mut node = &self.root;
        for c in query {
            if let Some(child) = node.children.get(c) {
                node = child;
            } else {
                return false;
            }
        }
        true
        // to check if the query is a SUFFIX, check if the final `node`
        // contains a child with `$`
    }

    /// Counts the occurence of the substring, `query`.
    pub fn occurence(&self, query: &[u8]) -> usize {
        // this part is essentially the same as `contains_substr`.
        // if we fail to reconstruct the query by going down the trie,
        // the string does not contain `query` i.e. we return 0
        let mut node = &self.root;
        for c in query {
            if let Some(child) = node.children.get(c) {
                node = child;
            } else {
                return 0;
            }
        }
        // if `query` can be reconstructed, all the descendent leaves of
        // `node` represent difference occurences of the query
        fn dfs(node: &NaiveSuffixTrieNode, count: &mut usize) {
            for (c, child) in &node.children {
                if *c == b'$' {
                    *count += 1;
                } else {
                    dfs(child, count);
                }
            }
        }
        let mut count = 0;
        dfs(node, &mut count);
        count
    }

    /// Finds (one of) the longest substring that repeats at least n times.
    /// Essentially, this means to find the deepest node with at least `n` children.
    pub fn longest_repeated_substr(&self, n: usize) -> Vec<u8> {
        fn dfs(
            node: &NaiveSuffixTrieNode,
            buffer: &mut Vec<u8>,
            longest: &mut Vec<u8>,
            longest_len: &mut usize,
            n: usize,
        ) -> usize {
            let mut descendents_leaves = 0;
            for (c, child) in &node.children {
                if *c == b'$' {
                    // finds a leave
                    descendents_leaves += 1;
                } else {
                    // else add up descendent leaves of all branches
                    buffer.push(*c);
                    descendents_leaves += dfs(child, buffer, longest, longest_len, n);
                    buffer.pop().unwrap();
                }
            }
            if descendents_leaves >= n && buffer.len() > *longest_len {
                *longest = buffer.clone();
                *longest_len = buffer.len();
            }
            // unsafe {
            //     println!(
            //         "{} has {} descendents",
            //         std::str::from_utf8_unchecked(buffer),
            //         descendents_leaves
            //     );
            // }
            descendents_leaves
        }
        let mut longest = Vec::new();
        dfs(&self.root, &mut Vec::new(), &mut longest, &mut 0, n);
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref S1: &'static [u8] = b"abracadabra";
        static ref ST1: NaiveSuffixTrie = NaiveSuffixTrie::from_str_naive(&S1);
    }
    #[test]
    fn contains_substr_1() {
        assert!(ST1.contains_substr(b"abra"));
        assert!(ST1.contains_substr(b"brac"));
        assert!(ST1.contains_substr(b"abra"));
        assert!(!ST1.contains_substr(b"abrc"));
        assert!(!ST1.contains_substr(b"arac"));
    }

    #[test]
    fn occurence_1() {
        assert_eq!(ST1.occurence(b"af"), 0);
        assert_eq!(ST1.occurence(b"abrac"), 1);
        assert_eq!(ST1.occurence(b"abra"), 2);
        assert_eq!(ST1.occurence(b"a"), 5);
    }

    #[test]
    fn longest_repeated_substr_1() {
        assert_eq!(ST1.longest_repeated_substr(2), b"abra".to_vec());
        assert_eq!(ST1.longest_repeated_substr(3), vec![b'a']);
        assert_eq!(ST1.longest_repeated_substr(5), vec![b'a']);
        assert_eq!(ST1.longest_repeated_substr(6), vec![]);
    }
}
