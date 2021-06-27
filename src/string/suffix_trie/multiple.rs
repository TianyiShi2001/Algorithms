use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub children: HashMap<u8, Box<Node>>,
    pub contained_in: Vec<bool>,
}

impl Node {
    fn new(n: usize) -> Self {
        Node {
            children: HashMap::default(),
            contained_in: vec![false; n],
        }
    }
    fn common_to(&self) -> usize {
        self.contained_in.iter().filter(|x| **x).count()
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Node,
    n: usize,
}

impl Trie {
    fn new(n: usize) -> Self {
        Self {
            root: Node::new(n),
            n,
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn from_ascii_alphabetic_strs(ss: &[&[u8]]) -> Self {
        let n = ss.len();
        let mut slf = Trie::new(n);
        const A_CODEPOINT: usize = 0x41;
        assert!(n < A_CODEPOINT);
        let mut sentinel = 0u8;
        for s in ss.iter() {
            slf.insert(s, sentinel);
            sentinel += 1;
        }
        slf
    }

    fn insert(&mut self, s: &[u8], sentinel: u8) {
        let n = s.len();
        for i in 0..n {
            let suffix = s[i..n].iter().chain(std::iter::once(&sentinel));
            let mut node = &mut self.root as *mut Node;
            for c in suffix {
                let nd = unsafe { &mut *node };
                nd.contained_in[sentinel as usize] = true;
                node = &mut **nd
                    .children
                    .entry(*c)
                    .or_insert_with(|| Box::new(Node::new(self.n)));
            }
        }
    }

    /// Essentially, the longest common substring can be translated to
    /// 'find the deepest node whose descendent leaves contain each of the sentinels at least once'
    pub fn longest_common_substring(&self, n: usize) -> Vec<u8> {
        fn dfs(
            node: &Node,
            longest_len: &mut usize,
            longest: &mut Vec<u8>,
            buffer: &mut Vec<u8>,
            n: usize,
        ) {
            if node.common_to() >= n {
                let b_len = buffer.len();
                if b_len > *longest_len {
                    *longest = buffer.clone();
                    *longest_len = b_len;
                }
                for (&c, child) in &node.children {
                    buffer.push(c);
                    dfs(child, longest_len, longest, buffer, n);
                    buffer.pop().unwrap();
                }
            }
        }
        let mut longest = Vec::new();
        dfs(&self.root, &mut 0, &mut longest, &mut Vec::new(), n);
        longest
    }

    pub fn longest_common_substring_of_all(&self) -> Vec<u8> {
        self.longest_common_substring(self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref SS: [&'static [u8]; 3] = [
            b"idfjiHELLOWORLDdfszd",
            b"qiodfHELLOWORLDzojgjs",
            b"jfiosiqpHELLOzvzxfrdf"
        ];
        static ref ST: Trie = Trie::from_ascii_alphabetic_strs(&*SS);
    }
    #[test]
    fn longest_common_substring() {
        assert_eq!(&ST.longest_common_substring(2), b"HELLOWORLD");
        assert_eq!(&ST.longest_common_substring_of_all(), b"HELLO");
    }
}
