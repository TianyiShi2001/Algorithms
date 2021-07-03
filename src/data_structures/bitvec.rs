use super::bit::BitOpts;
use std::fmt::{Binary, Debug, Display};

pub struct BitVec<B: BitOpts> {
    inner: Vec<B>,
    word_size: usize,
    len: usize,
}

impl<B: BitOpts> BitVec<B> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            word_size: std::mem::size_of::<B>() * 8,
            len: 0,
        }
    }
    pub fn push(&mut self, v: bool) {
        let (i, j) = self.calculate_indices(self.len);
        if i + 1 > self.inner.len() {
            self.inner.push(B::zero());
        }
        if v {
            self.inner[i].set_bit(j);
        }
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<bool> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let (i, j) = self.calculate_indices(self.len);
            let res = self.inner[i].get_bit(j);
            if j == 0 {
                self.inner.pop().unwrap();
            } else {
                if res {
                    self.inner[i].clear_bit(j);
                }
            }
            Some(res)
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx >= self.len {
            panic!("Out of bound")
        }
        let (i, j) = self.calculate_indices(idx);
        self.inner[i].get_bit(j)
    }
    pub fn set(&mut self, idx: usize, v: bool) {
        if idx >= self.len {
            panic!("Out of bound")
        }
        let (i, j) = self.calculate_indices(idx);
        if v {
            self.inner[i].set_bit(j);
        } else {
            self.inner[i].clear_bit(j);
        }
    }
    fn calculate_indices(&self, idx: usize) -> (usize, usize) {
        (self.len / self.word_size, self.len % self.word_size)
    }
}

impl<B: BitOpts + Binary> Display for BitVec<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for word in &self.inner {
            write!(f, "{:0b} ", word)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitvec() {
        let mut bv = BitVec::<u16>::new();
        for _ in 0..10 {
            bv.push(false);
            bv.push(true);
        }
        assert_eq!(bv.pop(), Some(true));
        assert_eq!(bv.len, 19);
        assert_eq!(&bv.inner, &[0b1010101010101010, 0b0010]);
    }
}
