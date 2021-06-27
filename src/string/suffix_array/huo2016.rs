use std::usize;

const EMPTY: usize = usize::MAX;
const UNIQUE: usize = usize::MAX - 1;
const MULTI: usize = usize::MAX - 2;

pub struct Huo2016 {
    pub s: Vec<u8>,
    pub sa: Vec<usize>,
    pub n: usize,
    pub sigma: usize,
}

impl Huo2016 {
    pub fn new(s: Vec<u8>, sigma: Option<usize>) -> Self {
        let n = s.len();
        let mut slf = Self {
            s,
            sa: vec![0; n],
            n,
            sigma: sigma.unwrap_or(u8::MAX as usize),
        };
        slf.rename();
        slf.sort_all_lms_chars();
        slf.induced_sort_lms_substrs();
        slf.induced_sort_all_suffixes();
        slf
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

        // count occurence
        for &c in &self.s {
            self.sa[c as usize] += 1;
        }

        // compute head indices
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
        self.reset_sa(self.sigma);
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
        let mut s_ip1_is_s = true; // the last character (sentinel) is always S
        let mut s_ip1 = 0; // the sentinel character is always 0
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = &mut self.s[i];
            if *s_i < s_ip1 || (*s_i == s_ip1 && s_ip1_is_s) {
                *s_i = self.sa[*s_i as usize] as u8;
                s_ip1_is_s = true;
            } else {
                s_ip1_is_s = false;
            }
            s_ip1 = *s_i;
        }
    }

    /// Sort all LMS characters of `s`, i.e. place the indices of the LMS characters in
    /// the tail of their corresponding buckets in `sa`.
    /// Unlike Nong et al. (2009), we do not use a bucket array, meaning we do not have extra
    /// space to store the LF/RF pointers/counters for each bucket to inidcate the position of
    /// the free entries in the process. For this purpose, we develop the "inferior counter
    /// trick", which allows us to carefully use the space in `sa` to store the information of
    /// both the indices and the pointers.
    fn sort_all_lms_chars(&mut self) {
        self.fill_sa(EMPTY);
        // Clear `sa`, then scan `s` from right to left. For every `s[i]` which is an LMS
        // character (which can be determined in constant time),  do the following:
        //   - if `sa[s[i]] = EMPTY`, let `sa[s[i]] = UNIQUE`, meaning it is the unique LMS
        //     character in this bucket
        //   - if `sa[s[i]] = UNIQUE`, let `sa[s[i]] = MULTI`, meaning the number of LMS
        //     characters in this bucket is at least 2
        //   - otherwise, do nothing

        // `s[0]` must not be LMS by definition
        // starting from `s[n - 2]` i.e. the second last character
        let mut s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
        let mut s_im1_is_s;
        let mut s_i = self.s[self.n - 2];
        let mut s_im1;
        // `i_minus_1` ranges from `n-3` to `0` inclusive, meaning `i` ranges from `n-2` to `1` inclusive.
        // `s[0]` must not be an LMS character by definition so it is fine that `i` does not include `0`.
        // `s[n-1]` is the sentinel character which is dealt with as a special case later.
        for i_minus_1 in (0..self.n - 2).rev() {
            s_im1 = self.s[i_minus_1];
            s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
            if !s_im1_is_s && s_i_is_s {
                // `s[i]` is LMS
                let sa_s_i = &mut self.sa[s_i as usize];
                match *sa_s_i {
                    EMPTY => *sa_s_i = UNIQUE,
                    UNIQUE => *sa_s_i = MULTI,
                    _ => (),
                }
            }
            s_i = s_im1;
            s_i_is_s = s_im1_is_s;
        }
        // sentinel is LMS character by definition, and it always uniquely occupies bucket 0
        self.sa[0] = UNIQUE;

        // We scan `s` from right to left. For every `s[i]` which is an LMS character,
        // we distinguish the following cases:
        //   - `sa[s[i]] == UNIQUE`: let `sa[s[i]] = i`, i.e. `s[i]` is the unique LMS
        //     character in its bucket, so we just put its index into its bucket (at the tail)
        //   - `sa[s[i]] == MULTI` and ``sa`

        let mut s_i_is_s = false;
        let mut s_im1_is_s;
        let mut s_i = self.s[self.n - 2];
        let mut s_im1;
        let mut i = self.n - 2;
        let sa = &mut self.sa as *mut Vec<usize>;
        unsafe {
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if !s_im1_is_s && s_i_is_s {
                    // `s[i]` is LMS
                    let sa_s_i = &mut (*sa)[s_i as usize];
                    match *sa_s_i {
                        // Case 1: `sa[s[i]] == UNIQUE`: let `sa[s[i]] = i`, i.e. `s[i]` is the unique LMS
                        // character in its bucket, so we just put its index into its bucket (at the tail)
                        UNIQUE => *sa_s_i = i,

                        MULTI => {
                            let sa_sim1 = &mut (*sa)[s_i as usize - 1];
                            if *sa_sim1 == EMPTY {
                                // Case 2: `sa[s[i]] == MULTI` and `sa[s[i]-1] == EMPTY`
                                // In this case, `s[i]` is the first (i.e. largest index, since we scan `s` from right
                                // to left) LMS-character in its bucket. So if `sa[s[i] − 2] = EMPTY`, we let
                                // `sa[s[i]−2] = i` and `sa[s[i]−1] = 1` (i.e., we use `sa[s[i]−1]` as the counter for
                                // the number of LMS characters which has been added to this bucket so far).
                                // Otherwise, `sa[s[i]−2] != EMPTY` (i.e., `sa[s[i] - 2]` is in a differentbucket, which
                                // implies that this bucket has only two LMS characters). Then we let `sa[s[i]] = i`
                                // and `sa[s[i]−1] = EMPTY`. (We do not need a counter in this case and the last LMS
                                // character belonging to this bucket will be dealt with in the later process)
                                let sa_sim2 = &mut (*sa)[s_i as usize - 2];
                                match *sa_sim2 {
                                    EMPTY => {
                                        *sa_sim2 = i;
                                        *sa_sim1 = 1;
                                    }
                                    _ => {
                                        *sa_s_i = i;
                                        *sa_sim1 = EMPTY;
                                    }
                                }
                            } else {
                                // Case 3: `sa[s[i]] =MULTI` and `sa[s[i]−1] !=EMPTY`
                                // In this case,`sa[s[i]−1]` is maintained as the counter. Let `c = sa[s[i]−1]`.
                                // We check whether the position (`sa[s[i]−c−2]`), i.e. `c + 2` positions before
                                // its tail, is `EMPTY` or not. If `sa[s[i]−c−2] == EMPTY`, let `sa[s[i]−c−2] =i`
                                // and increase `sa[s[i]−1]` by one (i.e., update the counter number). Otherwise
                                // `sa[s[i]−c−2] != EMPTY` (i.e., reaching another bucket), we need to shift
                                // these `c` indices to the right by two positions (i.e., move `sa[s[i]−c−1..=s[i]−2]`
                                // to `sa[s[i]−c+ 1..=s[i]])`, and let `sa[s[i]−c] = i` and `sa[s[i]−c−1] =EMPTY`.
                                // After this, only one LMS-character needs to be added into this bucket in the later
                                // process.
                                let x = &mut (*sa)[s_i as usize - *sa_sim1 - 2] as *mut usize;
                                if *x == EMPTY {
                                    *x = i;
                                    *sa_sim1 += 1;
                                } else {
                                    for j in (s_i as usize - *sa_sim1 + 1..=s_i as usize).rev() {
                                        (*sa)[j] = (*sa)[j - 2];
                                    }
                                    (*sa)[s_i as usize - *sa_sim1] = i;
                                    (*sa)[s_i as usize - *sa_sim1 - 1] = EMPTY;
                                }
                            }
                        }
                        _ => {
                            // Case 4: `sa[s[i]]` is an index
                            // From case (2) and (3), we know the current `s[i]` must be the last LMS-character in
                            // its bucket. So we scan `sa` from right to left, starting with `sa[s[i]]`, to find
                            // the first position `j` such that `sa[j] = EMPTY`. Then we let `sa[j] = i`. Now, we
                            // have filled the entire bucket. However, we note that not every bucket is fully filled
                            // as we have only processed LMS-characters so far.
                            let mut j = s_i as usize;
                            while self.sa[j] != EMPTY {
                                j -= 1;
                            }
                            self.sa[j] = i;
                        }
                    }
                }
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
                i = i_minus_1;
            }
            (*sa)[0] = UNIQUE;
        }
        // After the above Step 1 and 2, there may be still some special symbols `MULTI` and the counters
        // (because the bucket is not fully filled, so we have not shifted these indices to the right in
        // the bucket). We need to free these positions. We scan `sa` once more from right to left. If
        // `sa[i] == MULTI`, we shift the indices of LMS-characters in this bucket to the right by two
        // positions (i.e., `sa[i−c−1..=i−2]` to `sa[i−c+ 1..=i]`) and let `sa[i−c−1] = sa[i−c] = EMPTY`,
        // where `c=sa[i−1]` denotes the counter.
        for i in (0..self.n).rev() {
            if self.sa[i] == MULTI {
                let c = self.sa[self.s[i] as usize - 1];
                for j in (i - c + 1..=i).rev() {
                    self.sa[j] = self.sa[j - 2];
                }
                self.sa[i - c - 1] = EMPTY;
                self.sa[i - c] = EMPTY;
            }
        }
        self.sa[0] = self.n - 1; // sentinel as a special case
    }

    /// Sort all LMS substrings from the sorted LMS characters using induced sorting.
    fn induced_sort_lms_substrs(&mut self) {
        // sort the LMS prefix of all suffixes from the sorted LMS characters
    }

    fn induced_sort_all_suffixes(&mut self) {
        // induced sort of L suffixes from sorted LMS suffixes
        let mut s_ip1_is_l = false;
        let mut s_ip1 = 0;
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = self.s[i];
            if s_i > s_ip1 || (s_i == s_ip1 && s_ip1_is_l) {
                // `s[i]` is L
                let sa_si = &mut self.sa[s_i as usize];
                if *sa_si == EMPTY {
                    *sa_si = UNIQUE;
                } else if *sa_si == UNIQUE {
                    *sa_si = MULTI;
                }
                s_ip1_is_l = true;
            } else {
                s_ip1_is_l = false;
            }
            s_ip1 = s_i;
        }
        // TODO: sentinel case?
        // scan `sa` from left to right to sort all L suffixes
        let mut i = 0;
        while i <= self.n {
            let sa_i = self.sa[i];
            if sa_i == MULTI {
                i += 2;
                continue;
            }
            if sa_i != UNIQUE && sa_i != EMPTY {
                let j = sa_i - 1;
                // TODO: implement suf(j) is L
                let mut suf_j_is_l = true;
                if suf_j_is_l {
                    // place suf_j into the LF entry of the bucket and increase counter by one
                }
            }
            i += 1;
        }
        // Now all L-suffixes are sorted
        // TODO: free MULTI and counters

        // Remove LMS suffixes from `sa`
        // TODO

        // * step 3: induced sort of S suffixes from sorted L suffixes
        let mut s_ip1_is_s = false;
        let mut s_ip1 = 0;
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = self.s[i];
            if s_i < s_ip1 || (s_i == s_ip1 && s_ip1_is_s) {
                // `s[i]` is S
                let sa_si = &mut self.sa[s_i as usize];
                if *sa_si == EMPTY {
                    *sa_si = UNIQUE;
                } else if *sa_si == UNIQUE {
                    *sa_si = MULTI;
                }
                s_ip1_is_s = true;
            } else {
                s_ip1_is_s = false;
            }
            s_ip1 = s_i;
        }
        // TODO: sentinel case?
        // scan `sa` from left to right to sort all L suffixes
        let mut i = 0;
        while i <= self.n {
            let sa_i = self.sa[i];
            if sa_i == MULTI {
                i += 2;
                continue;
            }
            if sa_i != UNIQUE && sa_i != EMPTY {
                let j = sa_i - 1;
                // TODO: implement suf(j) is S
                let mut suf_j_is_s = true;
                if suf_j_is_s {
                    // place suf_j into the RF entry of the bucket and increase counter by one
                }
            }
            i += 1;
        }
    }

    fn reset_sa(&mut self, n: usize) {
        for i in 0..n {
            self.sa[i] = 0;
        }
    }
    fn fill_sa(&mut self, val: usize) {
        for i in 0..self.n {
            self.sa[i] = val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    const EXAMPLE_HUO: [u8; 13] = [2, 1, 1, 3, 3, 1, 1, 3, 3, 1, 2, 1, 0];
    const EXAMPLE_HUO_STEP_1_S: [u8; 13] = [7, 6, 6, 9, 9, 6, 6, 9, 9, 6, 7, 1, 0];
    const EXAMPLE_HUO_STEP_2_SA: [usize; 13] = [
        12, EMPTY, EMPTY, EMPTY, 1, 5, 9, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
    ];
    // lazy_static! {
    //     static ref EXAMPLE_HUO:
    // }
    #[test]
    fn test_step_1() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::new(s, Some(3));
        solver.rename();
        assert_eq!(&solver.s, &EXAMPLE_HUO_STEP_1_S);
    }

    #[test]
    fn test_step_2() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::new(s, Some(3));
        solver.rename();
        solver.sort_all_lms_chars();
        assert_eq!(&solver.sa, &EXAMPLE_HUO_STEP_2_SA);
    }
}
