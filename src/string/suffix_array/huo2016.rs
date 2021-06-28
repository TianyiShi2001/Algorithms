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
        let mut slf = Self::init(s, sigma);
        slf.rename();
        slf.sort_all_lms_chars();
        slf.induced_sort_lms_substrs();
        slf.induced_sort_all_suffixes();
        slf
    }
    pub fn init(s: Vec<u8>, sigma: Option<usize>) -> Self {
        let n = s.len();
        Self {
            s,
            sa: vec![0; n],
            n,
            sigma: sigma.unwrap_or(u8::MAX as usize),
        }
    }
    fn _from_inner(s: Vec<u8>, sa: Vec<usize>, sigma: usize) -> Self {
        let n = s.len();
        Self { s, sa, n, sigma }
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

        // println!("Rename part 1 (sa) before accumulation: {:?}", self.sa);

        // compute head indices
        let mut prev = 1; // or self.sa[0]; the sentinel always occurs once
        let mut curr;
        for i in 1..self.sigma {
            curr = &mut self.sa[i];

                *curr += prev;
                prev = *curr;
            
        }
        // Rename
        for c in &mut self.s[..self.n - 1] {
            // `s[n - 1]` is the sentinel which always remains to be 0 no matter how
            // the string is transformed
            *c = self.sa[*c as usize - 1] as u8
        }


        // println!("Rename part 1 (s) : {:?}", self.s);
        // println!("Rename part 1 (sa): {:?}", self.sa);

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
        let mut prev = 0; // tail of bucket 0 is always 0 because there is one
                          // and only one sentinel
        let mut curr;
        for i in 1..self.n {

            curr = &mut self.sa[i];

                *curr += prev;
                prev = *curr;
            
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
        // clear SA
        self.fill_sa(EMPTY);
    }

    unsafe fn place_i_into_sa_ti_right_to_left(sa: *mut Vec<usize>, i: usize, s_i: u8) -> bool {
        let mut shifted = false;
        let sa_s_i = &mut (*sa)[s_i as usize];
        match *sa_s_i {
            // Case 1: `sa[s[i]] == UNIQUE`: let `sa[s[i]] = i`, i.e. `s[i]` is the unique LMS
            // character in its bucket, so we just put its index into its bucket (at the tail)
            UNIQUE => *sa_s_i = i,

            MULTI => {
                let counter = &mut (*sa)[s_i as usize - 1];
                if *counter == EMPTY {
                    // Case 2: `sa[s[i]] == MULTI` and `sa[s[i]-1] == EMPTY`
                    // In this case, `s[i]` is the first (i.e. largest index, since we scan `s` from right
                    // to left) LMS-character in its bucket. So if `sa[s[i] − 2] = EMPTY`, we let
                    // `sa[s[i]−2] = i` and `sa[s[i]−1] = 1` (i.e., we use `sa[s[i]−1]` as the counter for
                    // the number of LMS characters which has been added to this bucket so far).
                    // Otherwise, `sa[s[i]−2] != EMPTY` (i.e., `sa[s[i] - 2]` is in a differentbucket, which
                    // implies that this bucket has only two LMS characters). Then we let `sa[s[i]] = i`
                    // and `sa[s[i]−1] = EMPTY`. (We do not need a counter in this case and the last LMS
                    // character belonging to this bucket will be dealt with in the later process)
                    if s_i >= 2 {
                        let sa_sim2 = &mut (*sa)[s_i as usize - 2];
                        if *sa_sim2 == EMPTY {
                            *sa_sim2 = i;
                            *counter = 1;
                            return false;
                        }
                    }
                    // reach left end, or reach the bucket to the left
                    *sa_s_i = i;
                    *counter = EMPTY;
                } else {
                    // Case 3: `sa[s[i]] == MULTI` and `sa[s[i]−1] != EMPTY`
                    // In this case,`sa[s[i]−1]` is maintained as the counter. Let `c = sa[s[i]−1]`.
                    // We check whether the position (`sa[s[i]−c−2]`), i.e. `c + 2` positions before
                    // its tail, is `EMPTY` or not. If `sa[s[i]−c−2] == EMPTY`, let `sa[s[i]−c−2] =i`
                    // and increase `sa[s[i]−1]` by one (i.e., update the counter number). Otherwise
                    // `sa[s[i]−c−2] != EMPTY` (i.e., reaching another bucket), we need to shift
                    // these `c` indices to the right by two positions (i.e., move `sa[s[i]−c−1..=s[i]−2]`
                    // to `sa[s[i]−c+1..=s[i]])`, and let `sa[s[i]−c] = i` and `sa[s[i]−c−1] =EMPTY`.
                    // After this, only one LMS-character needs to be added into this bucket in the later
                    // process.
                    if s_i as usize >= *counter + 2 {
                        let x = &mut (*sa)[s_i as usize - *counter - 2] as *mut usize;
                        if *x == EMPTY {
                            *x = i;
                            *counter += 1;
                            return false;
                        }
                    }

                    let counter = *counter;
                    // will mutate, so copy the counter value out
                    for j in (s_i as usize - counter + 1..=s_i as usize).rev() {
                        (*sa)[j] = (*sa)[j - 2];
                    }
                    (*sa)[s_i as usize - counter] = i;
                    (*sa)[s_i as usize - counter - 1] = EMPTY;
                    shifted = true;
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
                while (*sa)[j] != EMPTY {
                    j -= 1;
                }
                (*sa)[j] = i;
            }
        }
        shifted
    }
    unsafe fn place_i_into_sa_ti_left_to_right(sa: *mut Vec<usize>, i: usize, s_i: u8) -> bool {
        let mut shifted = false;
        let sa_s_i = &mut (*sa)[s_i as usize];
        match *sa_s_i {
            UNIQUE => *sa_s_i = i,
            MULTI => {
                let counter = &mut (*sa)[s_i as usize + 1];
                if *counter == EMPTY {
                    let j = s_i as usize + 2;
                    if j < (*sa).len() {
                        let sa_sip2 = &mut (*sa)[j];
                        if *sa_sip2 == EMPTY {
                            *sa_sip2 = i;
                            *counter = 1;
                            return false;
                        }
                    }
                    *sa_s_i = i;
                    *counter = EMPTY;
                } else {
                    let j = s_i as usize + *counter + 2;
                    if j < (*sa).len() {
                        let x = &mut (*sa)[s_i as usize + *counter + 2] as *mut usize;
                        if *x == EMPTY {
                            *x = i;
                            *counter += 1;
                            return false;
                        }
                    }

                    let counter = *counter;
                    for j in s_i as usize..s_i as usize + counter {
                        (*sa)[j] = (*sa)[j + 2];
                    }
                    (*sa)[s_i as usize + counter] = i;
                    (*sa)[s_i as usize + counter + 1] = EMPTY;
                    shifted = true;
                }
            }
            _ => {
                let mut j = s_i as usize;
                while (*sa)[j] != EMPTY {
                    j += 1;
                }
                (*sa)[j] = i;
            }
        }
        shifted
    }

    /// Sort all LMS characters of `s`, i.e. place the indices of the LMS characters in
    /// the tail of their corresponding buckets in `sa`.
    /// Unlike Nong et al. (2009), we do not use a bucket array, meaning we do not have extra
    /// space to store the LF/RF pointers/counters for each bucket to inidcate the position of
    /// the free entries in the process. For this purpose, we develop the "inferior counter
    /// trick", which allows us to carefully use the space in `sa` to store the information of
    /// both the indices and the pointers.
    fn sort_all_lms_chars(&mut self) {
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
                    // println!("{} is LMS, tail is {}", i, s_i);
                    Self::place_i_into_sa_ti_right_to_left(sa, i, s_i);
                    // println!("{:?}", self.sa);
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
        let mut i = self.n - 1;
        while i != 0 {
            if self.sa[i] == MULTI {
                let c = self.sa[i - 1];
                for j in (i - c + 1..=i).rev() {
                    self.sa[j] = self.sa[j - 2];
                }
                i -= c;
                self.sa[i] = EMPTY;
                i -= 1;
                self.sa[i] = EMPTY;
                // self.sa[i - c - 1] = EMPTY;
                // self.sa[i - c] = EMPTY;
                // i = i - c - 2;
                // continue;
            }
            i -= 1;
        }
        self.sa[0] = self.n - 1; // sentinel as a special case
        // println!("End of step 2: {:?}", self.sa);
    }

    fn remove_all_lms_chars(&mut self) {
        let mut s_i_is_s = false;
        let mut s_im1_is_s;
        let mut s_i = self.s[self.n - 2];
        let mut s_im1;
        for i_minus_1 in (0..self.n - 2).rev() {
            s_im1 = self.s[i_minus_1];
            s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
            if !s_im1_is_s && s_i_is_s {
                // `s[i]` is LMS
                let sa_s_i = &mut self.sa[s_i as usize];
                match *sa_s_i {
                    MULTI => {
                        self.sa[s_i as usize - 1] += 1;
                    }
                    UNIQUE => {
                        *sa_s_i = MULTI;
                        self.sa[s_i as usize - 1] = 2; // set counter
                    }
                    _ => *sa_s_i = UNIQUE,
                }
            }
            s_i = s_im1;
            s_i_is_s = s_im1_is_s;
        }
        // don't touch sentinel

        // TODO: scan from right to left to achieve actual linear time
        for i in 1..self.n {
            let sa_i = self.sa[i];
            match sa_i {
                UNIQUE => self.sa[i] = EMPTY,
                MULTI => {
                    let c = self.sa[i - 1];
                    for j in i  + 1 - c..=i {
                        self.sa[j] = EMPTY;
                    }
                }
                _ => (),
            }
        }
    }

    /// Sort all LMS substrings from the sorted LMS characters using induced sorting.
    fn induced_sort_lms_substrs(&mut self) {
        // sort the LMS prefix of all suffixes from the sorted LMS characters
    }

    fn induced_sort_all_suffixes(&mut self) {
        // Step 1. Induced sort all L-suffixes from the sorted LMS-suffixes:
        // (1) First initializeSA: We scan `s` from right to left. For every `s[i]`
        // which is L-type, do the following:
        //   (a) If `sa[s[i]] = EMPTY`, let `sa[s[i]] = UNIQUE` (unique L-type character
        //      in this bucket).
        //   (b) If `sa[s[i]] = UNIQUE`, let `sa[s[i]] = MULTI` (the number of L-type
        //       characters in this bucket is at least 2)
        //   (c) Otherwise do nothing.
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
        assert!(self.sa[0] == self.n - 1); // sentinel should not change // TODO: remove this assertion

        // (2) Then we scan `sa` from left to right to sort all the L-suffixes.
        //   (a) If `sa[i] == EMPTY`, do nothing.
        //   (b) If `sa[i]` is an index, we let `j = sa[i]−1`. Then, if suf(j)
        //       is L-suffix (this can be identified inconstant time from the
        //       following Lemma 9), we place suf(j) into the LF-entry (recall
        //       that LF-entry denotes the leftmost free entry in its bucket) of
        //       its bucket and increase the counter by one.
        //   (c) If `sa[i] == MULTI`, which means `sa[i]` is the head of its
        //       bucket, and this bucket has at least two L-suffixes which are
        //       not sorted, we use `sa[i]` and `sa[i+ 1]` as the bucket head
        //       (the symbol `MULTI`)and the counter of this bucket, respectively.
        //       Then we skip these two entries and continue to scan `sa[i+ 2]`.
        let mut i = 0;
        let mut shifted_bucket_head = None;
        while i < self.n {
            let sa_i = self.sa[i];
            if sa_i == MULTI {
                shifted_bucket_head = Some(i);
                i += 2;
                continue;
            }
            if sa_i != UNIQUE && sa_i != EMPTY && sa_i > 0 {
                // why didn't the author mention that sa_i should be larger than 0 and not unique????
                let j = sa_i - 1;
                let s_j = self.s[j];
                let suf_j_is_l = s_j > self.s[sa_i]
                    || (s_j == self.s[sa_i] && {
                        // if all L-suffixes of `s` belonging to bucket `s[sa[i]]` are not already
                        // sorted, then suf(`sa[i] - 1`) is an L-suffix
                        // we can distinguish whether all L-suffixes of `s` belong to the current
                        // bucket `s[sa[i]]` are already sorted or not by scanning the current
                        // bucket once, when we reach a new bucket.
                        // ???
                        // if `s_j == s[sa_i]`
                        //   if `s[sa_i]` is L, `s[j]` must also be L, because they are both LF pointers
                        //   if `s[sa_i]` is LMS, `s[j]`, which is left to `s[sa_i]`, must be L by definition.
                        //      this case actually is impossible because `s[sa_i]` is an RF pointer, while
                        //      `s[j]` is an LF pointer
                        true
                        // TODO: is this correct? (also below)
                    });
                if suf_j_is_l {
                    unsafe {
                        // println!("{} is L, place into SA[{}]", j, s_j);
                        if Self::place_i_into_sa_ti_left_to_right(&mut self.sa, j, s_j) {
                            // if shifted, need to shift `i` back
                            if let Some(idx) = shifted_bucket_head {
                                if idx == s_j as usize {
                                    // if shifted bucket is the one that is shifted back
                                    i -= 1;
                            println!("shift {} to {}", i +1, i);
                            continue;
                                }
                            }
                            
                        }
                    }
                }
            }
            i += 1;
        }
        // Now all L-suffixes are sorted. Note that we still need to scan `sa` once more
        // to free these positions occupied by `MULTI` and counters. After this, the
        // indices of all L-suffixes are intheir final positions in `sa`.

        // Do we really need to do so? If there are remaining MULTI and counters,
        // the L suffixes must not be resting at their final positions
        // Yes we need. For example, input [10, 2, 6, 8, 10, 1, 6, 7, 9, 6, 1, 10, 10, 6, 2, 0]
        // will produce [15, 5, 10, 14, 1, M, 2, 9, 13, E, E, 8, 4, 0, 12, 11] at this stage
        // (final SA should be [15, 5, 10, 14, 1, 9, 13, 6, 2, 7, 3, 8, 4, 0, 12, 11])
        let mut i = 1;
        // don't touch sentinel at `sa[0]`
        while i < self.n {
            if self.sa[i] == MULTI {
                let c = self.sa[i + 1];
                for j in (i..i + c ).rev() {
                    self.sa[j] = self.sa[j + 2];
                }
                i += c;
                self.sa[i] = EMPTY;
                i += 1;
                self.sa[i] = EMPTY;
            }
            i += 1;
        }
        self.sa[0] = self.n - 1; // sentinel as a special case

        // Step 2. Remove LMS-Suffixes from `sa`
        // We can use a trick similar to the previous Step 2 in Section 3.3, i.e., placing
        // the indices of LMS-characters into `sa`. The difference is that instead of placing
        // the actual LMS-characters, we place the `EMPTY` symbol instead. Also note that we
        // do not delete the sentinel since it must be in the final position. Now, `sa` contains
        // only all L-suffixes and the sentinel, and all of them are intheir final positions in `sa`.
        // TODO: is this correct? Section 3.3 is to place all LMS characters, but here we are removing the
        //       so-called "LMS suffixes". Are these two terms the same? If they are the same, what is the
        //       purpose of section 3.4, 3.5, and 3.6? Why is the end result of section 3.6 the same as
        //       3.3? Is this just because they chose a bad example and they turn out to be the same by coincidence?
        // println!("After sorting L: {:?}", self.sa);
        // println!("Before removing lms chars: {:?}", self.sa);
        self.remove_all_lms_chars();

       // println!("After removing LMS: {:?}", self.sa);

        // Step 3. Induced sort all S-suffixes from the sorted L-suffixes
        // Now, this step is completely symmetrical to the above Step 1 (Sort all L-suffixes using
        // induced sorting). We use S-type and RF-entry instead of L-typeand LF-entry, and scan `sa`
        // from right to left instead of left to right.
        let mut s_ip1_is_s = true; // sentinel
        let mut s_ip1 = 0;
        let mut s_i;
        for i in (1..self.n - 1).rev() {
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
        // println!("After init S: {:?}", self.sa);
        // sentinel skipped, so `sa[0]` should not change
        assert!(self.sa[0] == self.n - 1);
        // TODO: sentinel case?
        // scan `sa` from left to right to sort all L suffixes
        let mut i = self.n - 1;
        let mut shifted_bucket_tail = None;
        while i != 0 {
            let sa_i = self.sa[i];
            if sa_i == MULTI {
                shifted_bucket_tail = Some(i);
                i -= 2;
                continue;
            }
            if sa_i != UNIQUE && sa_i != EMPTY && sa_i > 0 {
                let j = sa_i - 1;
                let s_j = self.s[j];
                // TODO: if correct, simplify
                let suf_j_is_s = s_j < self.s[sa_i]
                    || (s_j == self.s[sa_i]
                        && {
                            s_j as usize > i || 
                        // we know `s[j]` and `s[sa[i]]` (i.e. `s[j + 1`) are both in the current bucket (which contains index `i`)
                        // if `s_j > i`, then `s_j` must be a tail pointer of this bucket which is used to sort S suffixes
                        // if `s_j < i`, then `s_j` must be a head pointer which is used to sort L suffixes
                        // // if `s_j == i`, then `s_j` also must be a head pointer. Why? Suppose `s_j = i` is a tail pointer, meaning
                        // // we have just entered this bucket from the right hand side. We can reach here only if `sa[i]` has been 
                        // // correctly filled (i.e. containing an index of a S-suffix, not `MULTI`, `EMPTY` or `UNIQUE`, which would
                        // // otherwise be skipped).
                        // // If this bucket contains a unique (i.e. only one) S-suffix, then it is impossible that the bucket contains
                        // // two S-suffixes `j` and `j+1` (i.e. `sa[i]`).
                        // // If this bucket contains multiple S-suffixes, then:
                        // //   - if this bucket has not been shifted, `sa[i]` contains `MULTI` and we cannot reach here (skipped)
                        // //   - if this bucket has been shifted, we still need to check?
                        
                        (s_j as usize == i && {i != self.n - 1 && i + 1 == self.s[self.sa[i + 1]] as usize})
                        });
                // println!("i: {}, sa_i: {}, j: {}, is_s: {}", i, sa_i, j, suf_j_is_s);
                if suf_j_is_s {
                    unsafe {
                        if Self::place_i_into_sa_ti_right_to_left(&mut self.sa, j, s_j) {
                            if let Some(idx) = shifted_bucket_tail {
                                if idx == s_j as usize {
                                    i += 1;
                                    continue;
                                }
                            }
                            
                        }
                    }
                }
            }
            i -= 1;
        }
    }

    fn reset_sa(&mut self, n: usize) {
        for i in 0..=n {
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
    use crate::_test_utils::random_uniform_vec;
    const EXAMPLE_HUO: [u8; 13] = [2, 1, 1, 3, 3, 1, 1, 3, 3, 1, 2, 1, 0];
    const EXAMPLE_HUO_STEP_1_S: [u8; 13] = [7, 6, 6, 9, 9, 6, 6, 9, 9, 6, 7, 1, 0];
    const EXAMPLE_HUO_STEP_2_SA: [usize; 13] = [
        12, EMPTY, EMPTY, EMPTY, 1, 5, 9, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
    ];
    const EXAMPLE_HUO_FINAL_SA: [usize; 13] = [12, 11, 1, 5, 9, 2, 6, 10, 0, 4, 8, 3, 7];

    #[test]
    fn test_step_1() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::init(s, Some(3));
        solver.rename();
        assert_eq!(&solver.s, &EXAMPLE_HUO_STEP_1_S);
    }

    #[test]
    fn test_step_2() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::init(s, Some(3));
        solver.rename();
        solver.sort_all_lms_chars();
        assert_eq!(&solver.sa, &EXAMPLE_HUO_STEP_2_SA);
    }

    #[test]
    fn test_step_6() {
        let s = EXAMPLE_HUO.iter().copied().collect();
        let mut solver = Huo2016::init(s, Some(3));
        solver.rename();
        solver.sort_all_lms_chars();
        solver.induced_sort_all_suffixes();
        // println!("{:?}", solver.sa);
        assert_eq!(&solver.sa, &EXAMPLE_HUO_FINAL_SA);
    }

    use super::super::SuffixArray;

    #[test]
    fn test_manual() {
        let s = vec![
            1, 6, 4, 2, 7, 4, 5, 8, 6, 2, 2, 4, 5, 6, 3, 4, 1, 6, 4, 5, 6, 2, 1, 3, 4, 0,
        ];
        let expected = SuffixArray::from_str_very_naive(&s);
        println!("Expected: {:?}", expected.sa);
        let mut solver = Huo2016::init(s.clone(), Some(8));
        solver.rename();
        // println!("After rename T: {:?}", solver.s);
        solver.sort_all_lms_chars();
        solver.induced_sort_all_suffixes();
        println!("Computed: {:?}", solver.sa);
        assert_eq!(&expected.sa, &solver.sa);
    }

    #[test]
    fn test_rand() {
        let sigma = 10;
        //  let mut s = random_uniform_vec(1, sigma, 15);
        //  s.push(0);
       let mut s = vec![7, 8, 2, 4, 8, 2, 2, 5, 9, 4, 9, 1, 1, 5, 2, 0];
        println!("Input: {:?}", &s);
        let expected = SuffixArray::from_str_very_naive(&s);
        println!("Expected: {:?}", expected.sa);
        let mut solver = Huo2016::init(s.clone(), Some(sigma as usize));
        solver.rename();
        println!("After rename T: {:?}", solver.s);
        solver.sort_all_lms_chars();
        println!("After sorting LMS chars: {:?}", solver.sa);
        solver.induced_sort_all_suffixes();
        println!("Computed: {:?}", solver.sa);
        assert_eq!(&expected.sa, &solver.sa);
    }
}