use std::collections::HashMap;

pub struct NaiveSuffixTrie {
    pub root: NaiveSuffixTrieNode,
}

#[derive(Default)]
pub struct NaiveSuffixTrieNode {
    pub children: HashMap<u8, Box<NaiveSuffixTrieNode>>,
}

impl NaiveSuffixTrie {
    fn from_str_naive(s: &[u8]) -> Self {
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

    fn contains_substr(&self, query: &[u8]) -> bool {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_naive_suffix_trie() {
        let s = b"abracadabra";
        let st = NaiveSuffixTrie::from_str_naive(s);
        assert!(st.contains_substr(b"abra"));
        assert!(st.contains_substr(b"brac"));
        assert!(st.contains_substr(b"abra"));
        assert!(!st.contains_substr(b"abrc"));
        assert!(!st.contains_substr(b"arac"));
    }
}
