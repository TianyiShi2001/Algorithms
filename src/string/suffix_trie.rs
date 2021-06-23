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
    }

    /// Count the occurence of the substring, `query`.
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
}
