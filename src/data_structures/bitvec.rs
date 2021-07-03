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
    pub fn from_inner(inner: Vec<B>, len: usize) -> Self {
        Self {
            inner,
            word_size: std::mem::size_of::<B>() * 8,
            len,
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
    /// Counts the number of `1` bits from position `0` to `pos` inclusive.
    ///
    /// ```text
    /// idx  = 01234567
    /// BV   = 00101101
    /// rank = 00112334
    /// ```
    ///
    /// This is a naive O(n) implementation.
    pub fn rank(&self, pos: usize) -> usize {
        let (i, j) = self.calculate_indices(pos);
        let mut res = 0;
        for k in 0..i {
            res += self.inner[k].count_ones();
        }
        for k in 0..=j {
            res += self.inner[i].get_bit(k) as u32;
        }
        res as usize
    }
    /// Finds the position of the i-th `1` in the bitvec.
    /// In other words, it can be considered as the inverse of `rank`.
    /// `rank(select(k)) == k`.
    ///
    /// ```text
    /// idx  = 01234567
    /// BV   = 00101101
    /// sel  = 02457
    /// rank = 00112334
    /// ```
    ///
    /// This is a naive O(n) implementation.
    pub fn select(&self, n: u32) -> usize {
        let mut rank = 0;
        let mut w;
        for i in 0..self.inner.len() {
            w = self.inner[i];
            rank += w.count_ones();
            if rank >= n {
                for j in (0..self.word_size).rev() {
                    if w.get_bit(j) {
                        rank -= 1;
                    }
                    if rank < n {
                        return self.word_size * i + j;
                    }
                }
            }
        }
        panic!("Out of bound")
    }
    fn calculate_indices(&self, idx: usize) -> (usize, usize) {
        (idx / self.word_size, idx % self.word_size)
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

    #[test]
    fn test_rank_select() {
        let bv = BitVec::from_inner(vec![0b10100110u8, 0b1010], 8);

        assert_eq!(bv.rank(0), 0);
        assert_eq!(bv.rank(1), 1);
        assert_eq!(bv.rank(2), 2);
        assert_eq!(bv.rank(3), 2);
        assert_eq!(bv.rank(4), 2);
        assert_eq!(bv.rank(5), 3);
        assert_eq!(bv.rank(6), 3);
        assert_eq!(bv.rank(7), 4);
        assert_eq!(bv.rank(8), 4);
        assert_eq!(bv.rank(9), 5);
        assert_eq!(bv.rank(10), 5);
        assert_eq!(bv.rank(11), 6);

        assert_eq!(bv.select(1), 1);
        assert_eq!(bv.select(2), 2);
        assert_eq!(bv.select(3), 5);
        assert_eq!(bv.select(4), 7);
        assert_eq!(bv.select(5), 9);
        assert_eq!(bv.select(6), 11);
    }
}
