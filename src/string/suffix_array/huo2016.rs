pub struct Huo2016 {
    pub s: Vec<u8>,
    pub sa: Vec<usize>,
    pub n: usize,
    pub sigma: usize,
}

impl Huo2016 {
    fn new(s: Vec<u8>, sigma: Option<usize>) -> Self {
        let n = s.len();
        Self {
            s,
            sa: vec![0; n],
            n,
            sigma: sigma.unwrap_or(u8::MAX as usize),
        }
    }

    /// Rename each L-type character of `s` to be the index of its bucket head and each
    /// S-type chaeacter of `s` to be the index of its bucket tail. This does not change
    /// the lexicographical order of all suffixes of `s`
    ///
    /// - Time compexity: O(n)
    /// - Space complexity: O(1)
    fn rename(&mut self) {
        // scan `s` once to computer the number of times each character occurs in `s` and
        // store them in `sa`. Then we perform a prefix sum computation to determine the
        // starting position of each character (i.e. bucket head) in `sa`. Finally we scan
        // `s` once again to rename each character as the index of its bucket head.
        for &c in &self.s {
            self.sa[c as usize] += 1;
        }
        let mut prev = 1; // or self.sa[0]; the sentinel always occurs once
        let mut curr;
        for i in 1..=self.sigma {
            curr = &mut self.sa[i];
            if *curr == 0 {
                break;
            } else {
                *curr += prev;
                prev = *curr;
            }
        }
        // Rename
        for c in &mut self.s[..self.n - 1] {
            // `s[n - 1]` is the sentinel which always remains to be 0 no matter how
            // the string is transformed
            *c = self.sa[*c as usize - 1] as u8
        }

        // Now we need to let the S-type characters to be the index of its bucket tail.
        // Again we count the occurence of each character, store them and compute the
        // tail positions. Then we scan `s` again from right to left. For each S-type
        // `s[i]`, we let it be the index of its bucket tail.
        // Note that if we scan `s` from right to left, for each `s[i]`, we can know
        // its type if L or S in O(1) time. There are 2 cases:
        //     - if `s[i] != s[i+1]`, we can know its type immediately by definition
        //     - if `s[i] == s[i+1]`, then its type is the same as `s[i+1]`. We only
        //       need to maintain one boolean variable which represent the type of the
        //       previous scanned character `s[i+1]`

        // clear `sa`
        for i in 0..=self.sigma {
            self.sa[i] = 0;
        }
        // count occurence
        for &c in &self.s {
            self.sa[c as usize] += 1;
        }
        // compute tail indices (inclusive)
        let mut i = 0;
        let mut prev = 0; // tail of bucket 0 is always 0 because there is one
                          // and only one sentinel
        let mut curr;
        loop {
            i += 1;
            curr = &mut self.sa[i];
            if *curr == 0 {
                break;
            } else {
                *curr += prev;
                prev = *curr;
            }
        }
        let mut prev_is_s = true; // the last character (sentinel) is always S
        let mut prev_val = 0; // the sentinel character is always 0
        let mut curr_val;
        for i in (0..self.n - 1).rev() {
            curr_val = &mut self.s[i];
            if *curr_val < prev_val || (*curr_val == prev_val && prev_is_s) {
                *curr_val = self.sa[*curr_val as usize] as u8;
                prev_is_s = true;
            } else {
                prev_is_s = false;
            }
            prev_val = *curr_val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    const EXAMPLE_HUO: [u8; 13] = [2, 1, 1, 3, 3, 1, 1, 3, 3, 1, 2, 1, 0];
    // lazy_static! {
    //     static ref EXAMPLE_HUO:
    // }
    #[test]
    fn test_rename() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::new(s, Some(3));
        solver.rename();
        assert_eq!(&solver.s, &[7, 6, 6, 9, 9, 6, 6, 9, 9, 6, 7, 1, 0]);
    }
}
