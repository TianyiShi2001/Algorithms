use num_traits::{PrimInt, Unsigned};
use std::fmt::{Debug, Display};
use std::usize;

// trait UnsignedInt = PrimInt + Unsigned;

const EMPTY: usize = usize::MAX;
const UNIQUE: usize = usize::MAX - 1;
const MULTI: usize = usize::MAX - 2;

pub struct Li2016<'a, T>
where
    //  S: PrimInt + Unsigned,
    T: PrimInt + Unsigned + Display + Debug,
{
    pub s: &'a mut [T],
    pub sa: &'a mut [usize],
    pub n: usize,
    pub sigma: usize,
}

impl<'a, T> Li2016<'a, T>
where
    // S: PrimInt + Unsigned,
    T: PrimInt + Unsigned + Display + Debug,
{
    pub fn new(s: &'a mut [T], sa: &'a mut [usize], sigma: Option<usize>) -> Self {
        let mut slf = Self::init(s, sa, sigma);
        slf.solve(true);
        slf
    }
    pub fn init(s: &'a mut [T], sa: &'a mut [usize], sigma: Option<usize>) -> Self {
        let n = s.len();
        Self {
            s,
            sa,
            n,
            sigma: sigma.unwrap_or(T::max_value().to_usize().unwrap()),
        }
    }
    fn _from_inner(s: &'a mut [T], sa: &'a mut [usize], sigma: usize) -> Self {
        let n = s.len();
        Self { s, sa, n, sigma }
    }
    pub fn solve(&mut self, recursive: bool) {
        // println!("original string: {:?}", self.s);
        println!("sigma: {:?}", self.sigma);
        self.rename();
        // println!("renamed string: {:?}", self.s);
        let n1 = self.sort_all_lms_chars();
        println!("n1: {}", n1);
        if n1 == 1 {
            // if there is only one LMS character i.e. the sentinel we can solve without ambiguity
            self.induced_sort_all_suffixes();
        } else {
            println!("Induced sorting all LMS substrs from chars...");
            self.induced_sort_lms_substrs();
            if !recursive {
                println!("Retaining LMSs...");
                self.retain_sorted_lms_substrs();
                println!("Induced sorting all suffixes (bottom of recusion)...");
                self.induced_sort_all_suffixes();
                println!("Finished sorting all suffixes (bottom of recusion)...");
                return;
            }
            let e = self.move_sorted_lms_substrs_to_the_end();
            println!("Constructing T1...");
            let (max_rank, has_duplicate) = self.construct_t1(e);
            println!(
                "T1 max rank: {}; has duplicate: {}",
                max_rank, has_duplicate
            );
            let (mut s1, sa1) = self.sa.split_at_mut(self.n - n1);
            sa1.fill(0); // prepare for renaming
            s1 = &mut s1[..n1]; // s1 from 0 to n1-1; sa1 from n-n1 to n-1; both have length n1
            let mut subproblem = Li2016::init(
                // &mut self.sa[..n1],
                // &mut self.sa[n1 + 1..],
                s1,
                sa1,
                Some(max_rank),
            );
            subproblem.solve(has_duplicate);
            println!("Moving T1 result from SA1 to the head");
            let sa = s1; // Just for readability
            for i in 0..n1 {
                sa[i] = sa1[i];
            }

            println!("Putting all LMS characters (unsorted) to the end");
            let lms = sa1; // for readability
                           // place unsorted LMS to the end
            let mut j = n1 - 1; // tail pointer
            lms[j] = self.n - 1; // sentinel as a special case
            j -= 1;
            let mut s_i_is_s = false;
            let mut s_im1_is_s;
            let mut s_i = self.s[self.n - 2];
            let mut s_im1;
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if !s_im1_is_s && s_i_is_s {
                    // `s[i]` is LMS
                    // println!("LMS {} is placed into lms[{}]", i_minus_1 + 1, j);
                    lms[j] = i_minus_1 + 1;
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
            }

            println!("Sorting LMS substrs in SA[0..=n1-1], using `sa[i] = lms[sa[i]]`...");
            // LMS substrs finally sorted in `SA[0..=n1-1]`
            let mut sa_i;
            for i in 0..n1 {
                sa_i = &mut sa[i];
                *sa_i = lms[*sa_i];
            }
            lms.fill(EMPTY);
            println!("Placing sorted LMS substrs back to corresponding buckets...");
            // place sorted LMS substrs back to corresponding buckets
            let sa = self.sa as *mut [usize];
            unsafe {
                let mut sa_i;
                let mut sa_i_val;
                let mut j;
                let mut curr_tail = 0; // dummy
                let mut offset = 0;
                for i in (1..n1).rev() {
                    sa_i = &mut (*sa)[i];
                    sa_i_val = *sa_i;
                    *sa_i = EMPTY;
                    j = self.s[sa_i_val].to_usize().unwrap(); // start scanning at the tail to the left
                    if j == curr_tail {
                        offset += 1;
                    } else {
                        curr_tail = j;
                        offset = 0;
                    }
                    // println!("tail: {}; offset: {}", curr_tail, offset);
                    (*sa)[curr_tail - offset] = sa_i_val;
                }
            }
            // then we can finally solve!
            println!("Induced sorting all suffixes...");
            self.induced_sort_all_suffixes();
            println!("Finished sorting (sigma={})", self.sigma);
        }
    }

    /// Rename each L-type character of `s` to be the index of its bucket head and each
    /// S-type chaeacter of `s` to be the index of its bucket tail. This does not change
    /// the lexicographical order of all suffixes of `s`
    ///
    /// - Time compexity: O(n)
    /// - Space complexity: O(1)
    fn rename(&mut self) {
        self.sa[0..=self.sigma].fill(0);
        // count occurence
        for &c in self.s.iter() {
            self.sa[c.to_usize().unwrap()] += 1;
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
            *c = T::from(self.sa[c.to_usize().unwrap() - 1]).unwrap()
        }

        // println!("Rename part 1 (s) : {:?}", self.s);
        // println!("Rename part 1 (sa): {:?}", self.sa);

        // clear `sa`
        self.sa[0..=self.sigma].fill(0);
        // count occurence
        for &c in self.s.iter() {
            self.sa[c.to_usize().unwrap()] += 1;
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
        let mut s_ip1 = T::zero(); // the sentinel character is always 0
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = &mut self.s[i];
            s_ip1_is_s = *s_i < s_ip1 || (*s_i == s_ip1 && s_ip1_is_s);
            if s_ip1_is_s {
                *s_i = T::from(self.sa[s_i.to_usize().unwrap()]).unwrap();
            }
            s_ip1 = *s_i;
        }
        // clear SA
        self.sa.fill(EMPTY);
    }

    unsafe fn place_i_into_sa_ti_right_to_left(sa: *mut [usize], i: usize, s_i: T) -> bool {
        let s_i = s_i.to_usize().unwrap();
        let mut shifted = false;
        let sa_s_i = &mut (*sa)[s_i];
        match *sa_s_i {
            // Case 1
            UNIQUE => *sa_s_i = i,
            MULTI => {
                let counter = &mut (*sa)[s_i - 1];
                if *counter == EMPTY {
                    // Case 2: `sa[s[i]] == MULTI` and `sa[s[i]-1] == EMPTY`
                    if s_i >= 2 {
                        let sa_sim2 = &mut (*sa)[s_i - 2];
                        if *sa_sim2 == EMPTY {
                            *sa_sim2 = i;
                            *counter = 1;
                            return false;
                        }
                    }
                    // reach left end, or reach the bucket to the left
                    // i.e. this bucket contains only 2 indexes/suffixes
                    *sa_s_i = i;
                    *counter = EMPTY;
                } else {
                    // Case 3: `sa[s[i]] == MULTI` and `sa[s[i]−1] != EMPTY`
                    if s_i >= *counter + 2 {
                        let x = &mut (*sa)[s_i - *counter - 2] as *mut usize;
                        if *x == EMPTY {
                            *x = i;
                            *counter += 1;
                            return false;
                        }
                    }
                    // reach left end, or reach the bucket to the left
                    let counter = *counter;
                    // will mutate, so copy the counter value out
                    for j in (s_i - counter + 1..=s_i).rev() {
                        (*sa)[j] = (*sa)[j - 2];
                    }
                    (*sa)[s_i - counter] = i;
                    (*sa)[s_i - counter - 1] = EMPTY;
                    shifted = true;
                }
            }
            _ => {
                // Case 4: `sa[s[i]]` is an index
                // `i` must be placed into the EMPTY slot left by case 2/3
                let mut j = s_i;
                while (*sa)[j] != EMPTY {
                    j -= 1;
                }
                (*sa)[j] = i;
            }
        }
        shifted
    }
    unsafe fn place_i_into_sa_ti_left_to_right(sa: *mut [usize], i: usize, s_i: T) -> bool {
        let s_i = s_i.to_usize().unwrap();
        let mut shifted = false;
        let sa_s_i = &mut (*sa)[s_i];
        match *sa_s_i {
            UNIQUE => *sa_s_i = i,
            MULTI => {
                let counter = &mut (*sa)[s_i + 1];
                if *counter == EMPTY {
                    let j = s_i + 2;
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
                    let j = s_i + *counter + 2;
                    if j < (*sa).len() {
                        let x = &mut (*sa)[j] as *mut usize;
                        if *x == EMPTY {
                            *x = i;
                            *counter += 1;
                            return false;
                        }
                    }

                    let counter = *counter;
                    for j in s_i..s_i + counter {
                        (*sa)[j] = (*sa)[j + 2];
                    }
                    (*sa)[s_i + counter] = i;
                    (*sa)[s_i + counter + 1] = EMPTY;
                    shifted = true;
                }
            }
            _ => {
                let mut j = s_i;
                while (*sa)[j] != EMPTY {
                    j += 1;
                }
                (*sa)[j] = i;
            }
        }
        shifted
    }

    /// Returns the number of LMS characters
    fn sort_all_lms_chars(&mut self) -> usize {
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
                let sa_s_i = &mut self.sa[s_i.to_usize().unwrap()];
                match *sa_s_i {
                    EMPTY => *sa_s_i = UNIQUE,
                    UNIQUE => *sa_s_i = MULTI,
                    _ => (),
                }
            }
            s_i = s_im1;
            s_i_is_s = s_im1_is_s;
        }
        self.sa[0] = self.n - 1; // sentinel as a special case

        let mut lms_char_count_excluding_sentinel = 0;
        let mut s_i_is_s = false;
        let mut s_im1_is_s;
        let mut s_i = self.s[self.n - 2];
        let mut s_im1;
        let mut i = self.n - 2;
        let sa = self.sa as *mut [usize];
        unsafe {
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if !s_im1_is_s && s_i_is_s {
                    // `s[i]` is LMS
                    // println!("{} is LMS, tail is {}", i, s_i);
                    Self::place_i_into_sa_ti_right_to_left(sa, i, s_i);
                    lms_char_count_excluding_sentinel += 1;
                    // println!("{:?}", self.sa);
                }
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
                i = i_minus_1;
            }
        }

        // Remove `MULTI` and counters (some may remain because SA is not fully filled i.e. not always
        // hitting end or an adjacent bucket)
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
                                 // println!("End of step 2: {:?}", self.sa);
        lms_char_count_excluding_sentinel + 1
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
                let sa_s_i = &mut self.sa[s_i.to_usize().unwrap()];
                match *sa_s_i {
                    MULTI => {
                        self.sa[s_i.to_usize().unwrap() - 1] += 1;
                    }
                    UNIQUE => {
                        *sa_s_i = MULTI;
                        self.sa[s_i.to_usize().unwrap() - 1] = 2; // set counter
                    }
                    _ => *sa_s_i = UNIQUE,
                }
            }
            s_i = s_im1;
            s_i_is_s = s_im1_is_s;
        }
        // don't touch sentinel
        let mut i = self.n - 1;
        while i != 0 {
            let sa_i = self.sa[i];
            i -= match sa_i {
                UNIQUE => {
                    self.sa[i] = EMPTY;
                    1
                }
                MULTI => {
                    let c = self.sa[i - 1];
                    for j in i + 1 - c..=i {
                        self.sa[j] = EMPTY;
                    }
                    c
                }
                _ => 1,
            }
        }
    }

    /// Sort all LMS substrings from the sorted LMS characters using induced sorting.
    fn induced_sort_lms_substrs(&mut self) {
        self.induced_sort_all_suffixes(); // same as section 3.7
                                          // sort the LMS prefix of all suffixes from the sorted LMS characters
    }

    /// Section 3.4 part 2
    ///
    /// Returns the left-most lms char (i.e. sentinel, with value `n - 1`) idx, i.e. `n - n1`
    fn move_sorted_lms_substrs_to_the_end(&mut self) -> usize {
        // println!("Before moving lms substrs to end: {:?}", self.sa);

        let mut i = self.n - 1;
        let s = self.s as *mut [T];
        let is_s_type_bucket_tail =
            |sa_i: usize| -> bool { unsafe { (*s)[sa_i] < (*s)[sa_i + 1] } };
        // let mut count;
        let mut tail;
        let mut sa_i;
        let mut end_ptr = self.n - 1;
        'outer: while i > 0 {
            sa_i = self.sa[i];
            if is_s_type_bucket_tail(sa_i) {
                // println!(
                //     "i = {:>2}, sa[i] = {:>2} is S-type bucket tail, and {} < {}",
                //     i,
                //     sa_i,
                //     unsafe { (*s)[sa_i] },
                //     unsafe { (*s)[sa_i + 1] }
                // );
                tail = i; // i.e. `s[sa[i]]`
                          // count number of S characters in this bucket
                loop {
                    // s[0] cannot be LMS by definition; skip `sa_i == 0`
                    if sa_i != 0 && self.s[sa_i - 1] > self.s[sa_i] {
                        // println!("{} is LMS", sa_i);
                        // is LMS
                        self.sa[end_ptr] = sa_i;
                        end_ptr -= 1;
                    }

                    i -= 1;
                    if i == 0 {
                        // sentinel dealt with separately
                        break 'outer;
                    }
                    sa_i = self.sa[i];
                    if self.s[sa_i].to_usize().unwrap() != tail {
                        // not an S char in the same bucket
                        // println!(
                        //     "{} is not s in the current bucket with tail/head {} instead of {}",
                        //     sa_i, self.s[sa_i], tail
                        // );
                        if is_s_type_bucket_tail(sa_i) {
                            tail = i;
                            // println!(
                            //     "i = {:>2}, sa[i] = {:>2} is S-type bucket tail, and {} < {}",
                            //     i,
                            //     sa_i,
                            //     unsafe { (*s)[sa_i] },
                            //     unsafe { (*s)[sa_i + 1] }
                            // );
                            continue;
                        } else {
                            break;
                        }
                    }
                    // println!(
                    //     "{} is also s in the current bucket with tail {}",
                    //     sa_i, tail
                    // );
                }
            }
            i -= 1;
        }
        // sentinel as a special case because cannot compare `sa[i] == n - 1` and cannot compare
        // `s[n-1]` to `s[n]`, the latter doesn't exist
        self.sa[end_ptr] = self.n - 1;
        self.sa[0..end_ptr].fill(EMPTY);
        end_ptr
    }

    fn retain_sorted_lms_substrs(&mut self) {
        let mut i = self.n - 1;
        let s = self.s as *mut [T];
        let is_s_type_bucket_tail =
            |sa_i: usize| -> bool { unsafe { (*s)[sa_i] < (*s)[sa_i + 1] } };
        let mut tail;
        let mut sa_i;
        'outer: while i > 0 {
            sa_i = self.sa[i];
            if is_s_type_bucket_tail(sa_i) {
                tail = i;
                loop {
                    if sa_i != 0 && self.s[sa_i - 1] > self.s[sa_i] {
                        // println!("{} is LMS", sa_i);
                        // is LMS
                        // self.sa[end_ptr] = sa_i;
                        // end_ptr -= 1;
                    } else {
                        // println!("{} is not LMS, empty sa[{}]", sa_i, i);
                        self.sa[i] = EMPTY;
                    }

                    i -= 1;
                    if i == 0 {
                        return;
                    }
                    sa_i = self.sa[i];
                    if self.s[sa_i].to_usize().unwrap() != tail {
                        if is_s_type_bucket_tail(sa_i) {
                            tail = i;
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
            // println!("{} is not LMS, empty sa[{}]", sa_i, i);
            self.sa[i] = EMPTY;

            i -= 1;
        }
    }

    /// Section 3.5: Construct the reduced problem T1
    /// Returns the max rank and if there are duplicated ranks
    fn construct_t1(&mut self, end_ptr: usize) -> (usize, bool) {
        let sa = self.sa as *mut [usize];
        let length_of_lms_string = |k: usize| -> usize {
            let mut prev = self.s[k];
            let mut curr;
            let mut next_lms_index = 0; // dummy; 0
            let mut i = k + 1;
            while i != self.n {
                curr = self.s[i];
                if prev > curr {
                    next_lms_index = i;
                } else if prev < curr && next_lms_index != 0 {
                    return next_lms_index - k + 1;
                }
                prev = curr;
                i += 1;
            }
            self.n - k
        };
        let mut prev_lms_len = 0; // sentinel actually has length 1, but it is always smaller than the next LMS
        let mut curr_lms_len;
        // skip sentinel at end_ptr
        let mut curr_lms_index;
        let mut prev_lms_index = 0; // dummy
        let mut rank = 0;
        let mut has_duplicated_ranks = false;
        for i in end_ptr + 1..self.n {
            curr_lms_index = self.sa[i];
            curr_lms_len = length_of_lms_string(curr_lms_index);
            // println!(
            //     "Prev: LMS index={}, len={}; Curr: LMS index={}, len={}",
            //     prev_lms_index, prev_lms_len, curr_lms_index, curr_lms_len
            // );
            if curr_lms_len != prev_lms_len {
                rank += 1
            } else {
                if self.s[curr_lms_index..curr_lms_index + curr_lms_len]
                    == self.s[prev_lms_index..prev_lms_index + prev_lms_len]
                {
                    // same rank
                    has_duplicated_ranks = true;
                } else {
                    rank += 1;
                }
            };
            unsafe {
                (*sa)[curr_lms_index / 2] = rank;
            }
            prev_lms_len = curr_lms_len;
            prev_lms_index = curr_lms_index;
        }
        // println!("{:?}", self.sa);
        // move `s1` scattered in `sa[0..n-n1-1]` to `sa[0..n1-1]`
        let mut sa_i;
        let mut j = 0;
        unsafe {
            for i in 0..end_ptr {
                sa_i = (*sa)[i];
                if sa_i != EMPTY {
                    (*sa)[j] = sa_i;
                    j += 1;
                }
            }
        }
        self.sa[j] = 0; // sentinel
        self.sa[j + 1..end_ptr].fill(EMPTY);
        (rank, has_duplicated_ranks)
    }

    fn induced_sort_all_suffixes(&mut self) {
        // Step 1. Induced sort all L-suffixes from the sorted LMS-suffixes:
        // initialise SA; scan S from right to left
        println!("Initilising SA for sorting L-type...");
        let mut s_ip1_is_l = false;
        let mut s_ip1 = T::zero();
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = self.s[i];
            s_ip1_is_l = s_i > s_ip1 || (s_i == s_ip1 && s_ip1_is_l);
            if s_ip1_is_l {
                // `s[i]` is L
                let sa_si = &mut self.sa[s_i.to_usize().unwrap()];
                if *sa_si == EMPTY {
                    *sa_si = UNIQUE;
                } else if *sa_si == UNIQUE {
                    *sa_si = MULTI;
                }
            }
            s_ip1 = s_i;
        }
        // sa[0] == n - 1 (sentinel unchanged)
        // println!("After init L {:?}", self.sa);

        // scan SA from left to right to sort all L-suffixes
        println!("Induced-sorting L-type...");
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
                let suf_j_is_l = s_j >= self.s[sa_i];
                // if `s_j == s[sa_i]`
                //   if `s[sa_i]` is L, `s[j]` must also be L, because they are both LF pointers
                //   if `s[sa_i]` is LMS, `s[j]`, which is left to `s[sa_i]`, must be L by definition.
                //      this case actually is impossible because `s[sa_i]` is a tail pointer, while
                //      `s[j]` is an head pointer

                if suf_j_is_l {
                    unsafe {
                        // println!("SA: {:?}", self.sa);
                        // println!("S:  {:?}", self.s);
                        // println!("{} is L, place into SA[{}]", j, s_j);

                        if Self::place_i_into_sa_ti_left_to_right(self.sa, j, s_j) {
                            // if shifted, need to shift `i` back
                            if let Some(idx) = shifted_bucket_head {
                                if idx == s_j.to_usize().unwrap() {
                                    // if shifted bucket is the one that is shifted back
                                    i -= 1;
                                    // println!("shift {} to {}", i + 1, i);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        println!("Removing MULTI and counters...");
        // Now all L-suffixes are sorted. Scan `sa` once more to empty `MULTI` and counters.
        // For example, input [10, 2, 6, 8, 10, 1, 6, 7, 9, 6, 1, 10, 10, 6, 2, 0]
        // will produce [15, 5, 10, 14, 1, M, 2, 9, 13, E, E, 8, 4, 0, 12, 11] at this stage
        // (final SA should be [15, 5, 10, 14, 1, 9, 13, 6, 2, 7, 3, 8, 4, 0, 12, 11])
        let mut i = 1;
        // don't touch sentinel at `sa[0]`
        while i < self.n {
            if self.sa[i] == MULTI {
                let c = self.sa[i + 1];
                for j in i..i + c {
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

        println!("Removing LMS indexes...");
        // Step 2. Remove LMS-Suffixes from `sa`
        self.remove_all_lms_chars();
        // println!("After removing LMS: {:?}", self.sa);

        println!("Initilising SA for sorting S-type...");
        // Step 3. Induced sort all S-suffixes from the sorted L-suffixes
        // Symmetrical to sorting L-suffixes; scan from right to left, look for S-type char and use RF-entry
        let mut s_ip1_is_s = true; // sentinel
        let mut s_ip1 = T::zero();
        let mut s_i;
        for i in (0..self.n - 1).rev() {
            s_i = self.s[i];
            s_ip1_is_s = s_i < s_ip1 || (s_i == s_ip1 && s_ip1_is_s);
            if s_ip1_is_s {
                // `s[i]` is S
                let sa_si = &mut self.sa[s_i.to_usize().unwrap()];
                if *sa_si == EMPTY {
                    *sa_si = UNIQUE;
                } else if *sa_si == UNIQUE {
                    *sa_si = MULTI;
                }
            }
            s_ip1 = s_i;
        }
        // println!("After init S: {:?}", self.sa);
        // sentinel skipped, so `sa[0]` should not change

        // scan `sa` from right to left to sort all S suffixes
        println!("Induced-sorting S-type...");
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
                let suf_j_is_s = s_j < self.s[sa_i]
                    || (s_j == self.s[sa_i]
                        && {
                            s_j .to_usize().unwrap() > i ||
                        // we know `s[j]` and `s[sa[i]]` (i.e. `s[j + 1]`) are both in the current bucket (which contains index `i`)
                        // if `s_j > i`, then `s_j` must be a tail pointer of this bucket which is used to sort S suffixes
                        // if `s_j < i`, then `s_j` must be a head pointer which is used to sort L suffixes
                        // if `s_j == i`, then need further check
                        {
                            let suspected_tail = s_j .to_usize().unwrap();
                            let sa_sj = self.sa[suspected_tail];
                            sa_sj == MULTI || suspected_tail < self.s[self.sa[suspected_tail + 1]] .to_usize().unwrap()
                        }
                        });
                // println!("{:?}", self.sa);
                // println!("i: {}, sa_i: {}, j: {}, is_s: {}", i, sa_i, j, suf_j_is_s);

                if suf_j_is_s {
                    // println!("place {} into {}", j, s_j);
                    // println!();
                    unsafe {
                        if Self::place_i_into_sa_ti_right_to_left(self.sa, j, s_j) {
                            if let Some(idx) = shifted_bucket_tail {
                                if idx == s_j.to_usize().unwrap() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_test_utils::random_uniform_vec;
    const EXAMPLE_LI: [u8; 13] = [2, 1, 1, 3, 3, 1, 1, 3, 3, 1, 2, 1, 0];
    const EXAMPLE_LI_RENAMED_S: [u8; 13] = [7, 6, 6, 9, 9, 6, 6, 9, 9, 6, 7, 1, 0];
    const EXAMPLE_LI_STEP_2_SA: [usize; 13] = [
        12, EMPTY, EMPTY, EMPTY, 1, 5, 9, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
    ];
    const EXAMPLE_LI_STEP_3_SA: [usize; 13] = [
        EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, 12, 1, 5, 9,
    ];
    const EXAMPLE_LI_STEP_4_SA: [usize; 13] =
        [1, 1, 2, 0, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, 12, 1, 5, 9];
    const EXAMPLE_LI_FINAL_SA: [usize; 13] = [12, 11, 1, 5, 9, 2, 6, 10, 0, 4, 8, 3, 7];

    #[test]
    fn test_step_1() {
        let mut s: Vec<u8> = EXAMPLE_LI.iter().copied().collect();
        let mut sa = vec![0; s.len()];
        let mut solver = Li2016::init(&mut s, &mut sa, Some(3));
        solver.rename();
        assert_eq!(&solver.s, &EXAMPLE_LI_RENAMED_S);
    }

    #[test]
    fn test_step_2() {
        let mut s: Vec<u8> = EXAMPLE_LI.iter().copied().collect();
        let mut sa = vec![0; s.len()];
        let mut solver = Li2016::init(&mut s, &mut sa, Some(3));
        solver.rename();
        solver.sort_all_lms_chars();
        assert_eq!(&solver.sa, &EXAMPLE_LI_STEP_2_SA);
    }

    #[test]
    fn test_step_3() {
        let mut s: Vec<u8> = EXAMPLE_LI_RENAMED_S.iter().copied().collect();
        let mut sa: Vec<usize> = EXAMPLE_LI_STEP_2_SA.iter().copied().collect();
        let mut solver = Li2016::init(&mut s, &mut sa, Some(3));
        solver.induced_sort_all_suffixes();
        let end_ptr = solver.move_sorted_lms_substrs_to_the_end();
        assert_eq!(&solver.sa, &EXAMPLE_LI_STEP_3_SA);
        assert_eq!(end_ptr, 9);
    }

    #[test]
    fn test_step_4() {
        let mut s: Vec<u8> = EXAMPLE_LI.iter().copied().collect();
        let mut sa: Vec<usize> = EXAMPLE_LI_STEP_3_SA.iter().copied().collect();
        let mut solver = Li2016::init(&mut s, &mut sa, Some(3));

        solver.construct_t1(9);
        assert_eq!(&solver.sa, &EXAMPLE_LI_STEP_4_SA);
    }

    #[test]
    fn test_solve() {
        let mut s: Vec<u8> = EXAMPLE_LI.iter().copied().collect();
        let mut sa = vec![0; s.len()];
        let mut solver = Li2016::init(&mut s, &mut sa, Some(3));
        solver.solve(true);
        assert_eq!(&solver.sa, &EXAMPLE_LI_FINAL_SA);
    }

    use super::super::SuffixArray;

    #[test]
    fn test_manual() {
        let mut s = vec![
            9u8, 1, 10, 6, 4, 4, 4, 5, 3, 5, 2, 3, 10, 4, 3, 4, 10, 3, 1, 0
        ];
        let mut sa = vec![0; s.len()];
        let sc = s.clone();
        let expected = SuffixArray::from_str_very_naive(&sc);
        // println!("Expected: {:?}", expected.sa);
        let mut solver = Li2016::init(&mut s, &mut sa, Some(10));
        solver.rename();
        // println!("After rename T: {:?}", solver.s);
        solver.sort_all_lms_chars();
        solver.induced_sort_all_suffixes();
         println!("Computed: {:?}", solver.sa);
        assert_eq!(&expected.sa, &solver.sa);
    }

    #[test]
    fn test_rand() {
        let sigma = 200u32;
        for _ in 0..100 {
            let mut s = random_uniform_vec(1, sigma, 1000);
            s.push(0);
            let mut sa = vec![0; s.len()];
            // println!("Input: {:?}", &s);
            let expected = SuffixArray::from_str_very_naive(&s).sa.clone();
            let mut solver = Li2016::init(&mut s, &mut sa, Some(sigma as usize));
            solver.solve(true);
            // println!("Computed: {:?}", solver.sa);
            // println!("Expected: {:?}", expected);
            assert_eq!(&expected, &solver.sa);
        }
    }
}
