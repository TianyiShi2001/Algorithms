use num_traits::{PrimInt, Unsigned};
use std::fmt::{Debug, Display};
use std::iter::Step;
use std::usize;

pub trait UnsignedInt = PrimInt + Unsigned + Display + Debug + Step;

const EMPTY: usize = usize::MAX;
const UNIQUE: usize = usize::MAX - 1;
const MULTI: usize = usize::MAX - 2;

pub struct Li2016Ro<'a, T: UnsignedInt> {
    s: &'a [T],
    sa: &'a mut [usize],
    sigma: T,
    n: usize,
    d: T,
}

impl<'a, T> Li2016Ro<'a, T>
where
    T: UnsignedInt,
{
    fn init(s: &'a [T], sa: &'a mut [usize], sigma: T) -> Self {
        let n = s.len();
        let d = T::from((sigma.to_f32().unwrap() / n as f32).ceil()).unwrap();
        Self { s, sa, sigma, n, d }
    }

    fn count_l_s_lms_lml(&self) -> (bool, usize, usize, usize) {
        let mut l = 1; // the char before sentinel must be L (index is n-2)
        let mut s = 1; // sentinel at n-1
        let mut lms = 1; // sentinel
        let mut lml = 0;
        let mut s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
        let mut s_im1_is_s;
        let mut s_i = self.s[self.n - 2];
        let mut s_im1;
        // `i_minus_1` ranges from `n-3` to `0` inclusive, meaning `i` ranges from `n-2` to `1` inclusive.
        // `s[0]` must not be an LMS character by definition so it is fine that `i` does not include `0`.
        // `s[n-1]` is the sentinel character which is dealt with as a special case later
        for i_minus_1 in (0..self.n - 2).rev() {
            s_im1 = self.s[i_minus_1];
            s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
            if s_im1_is_s {
                s += 1;
                if !s_i_is_s {
                    // `s[i]` is LML
                    lml += 1;
                }
            } else {
                // `s[i-1]` is L
                l += 1;
                if s_i_is_s {
                    // `s[i]` is LMS
                    lms += 1;
                }
            }

            s_i = s_im1;
            s_i_is_s = s_im1_is_s;
        }
        if l <= s {
            (true, l, s, lms)
        } else {
            (false, l, s, lml)
        }
    }

    /// STEP 1: use counting sort to sort LMS chars
    fn sort_lms_chars(&mut self, n1: usize) {
        let mut lo_char = T::one(); // sentinel dealt with as a special case
        let mut hi_char;
        let mut output_curr_head = self.n - n1 + 1; // plus one to skip sentinel
                                                    // println!("sigma: {}|half_n: {}", self.sigma, self.n / 2 + 1);
        let mut half_n = T::from(self.n / 2).unwrap() + T::one();
        // * since the sentinel is dealt with as a special case at the end, we can use one more slot
        if self.n % 2 == 1 {
            // * when n is odd, we can use one more slot.
            // * for example, when n = 9, there will be a maximum of 4 LMS chars,
            // * one of which is sentinel, so there are 3 LMS chars need to be sorted
            // * by counting sort, leaving 6 slots for counting array
            half_n = half_n + T::one();
        }
        loop {
            hi_char = lo_char + half_n;
            let range = (hi_char - lo_char).to_usize().unwrap();
            let mut s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
            let mut s_im1_is_s;
            let mut s_i = self.s[self.n - 2];
            let mut s_im1;
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if !s_im1_is_s && s_i_is_s {
                    if lo_char <= s_i && s_i < hi_char {
                        let idx_in_counting_arr = (s_i - lo_char).to_usize().unwrap();
                        self.sa[idx_in_counting_arr] += 1;
                    }
                }
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
            }
            // accumulation
            let mut prev = self.sa[0];
            let mut curr;
            for i in 1..range {
                curr = &mut self.sa[i];
                *curr += prev;
                prev = *curr;
            }
            // `prev` is the total number of LMS chars in this interval, and
            // will be added to `output_curr_head` after processing this interval

            // Scan S again to place LMS chars
            s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
            s_i = self.s[self.n - 2];
            // `i_minus_1` ranges from `n-3` to `0` inclusive, meaning `i` ranges from `n-2` to `1` inclusive.
            // `s[0]` must not be an LMS character by definition so it is fine that `i` does not include `0`.
            // `s[n-1]` is the sentinel character which is dealt with as a special case later
            let mut i = self.n - 2;
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if !s_im1_is_s && s_i_is_s {
                    // `s[i]` is LMS
                    if lo_char <= s_i && s_i < hi_char {
                        // println!("{} {} {}", lo_char, s_i, hi_char);
                        // if in current interval
                        let idx_in_counting_arr = (s_i - lo_char).to_usize().unwrap();
                        let idx_in_output_without_offset = &mut self.sa[idx_in_counting_arr];
                        *idx_in_output_without_offset -= 1;
                        self.sa[output_curr_head + *idx_in_output_without_offset] = i;
                    }
                }
                i = i_minus_1;
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
            }
            self.sa[0..range].fill(0); // clear the counting array
            if hi_char > self.sigma {
                break;
            }
            lo_char = hi_char;
            output_curr_head += prev; // recall that prev is the number of LMS chars in this interval
        }
        self.sa[self.n - n1] = self.n - 1; // sentinel as a special case
    }

    fn sort_lml_chars(&mut self, n1: usize) {
        // * Observation: when `n` is even, `nLML <= n/2-1`, and this occurs when `nL==nS`
        // *   e.g. `n=10`, `type=LSLSLSLSLS` or `type=SLSLSLSLLS`, in which case this function
        // *   will not be called (LMS chars are sorted instead), so the actual maximum of `nLML`
        // *   is practically `n/2-2`
        // *   When `n` is odd, the practical maximum of `nLML` is `n/2-1`.
        // *   e.g. `n=9`, `type=LSLSLSLLS`
        let mut lo_char = T::one(); // sentinel dealt with as a special case
        let mut hi_char;
        let mut output_curr_head = self.n - n1;
        let mut half_n = T::from(self.n / 2).unwrap() + T::one();
        if self.n % 2 == 0 {
            half_n = half_n + T::one();
        }
        loop {
            hi_char = lo_char + half_n;
            let range = (hi_char - lo_char).to_usize().unwrap();
            let mut s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
            let mut s_im1_is_s;
            let mut s_i = self.s[self.n - 2];
            let mut s_im1;
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if s_im1_is_s && !s_i_is_s {
                    // is LML
                    if lo_char <= s_i && s_i < hi_char {
                        let idx_in_counting_arr = (s_i - lo_char).to_usize().unwrap();
                        self.sa[idx_in_counting_arr] += 1;
                    }
                }
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
            }
            // accumulation
            let mut prev = self.sa[0];
            let mut curr;
            for i in 1..range {
                curr = &mut self.sa[i];
                *curr += prev;
                prev = *curr;
            }
            // `prev` is the total number of LML chars in this interval, and
            // will be added to `output_curr_head` after processing this interval

            // Scan `s` again to place LML chars
            s_i_is_s = false; // `s[n - 2]` must be L, because it is greater than the sentinel at `s[n - 1]`
            s_i = self.s[self.n - 2];
            // `i_minus_1` ranges from `n-3` to `0` inclusive, meaning `i` ranges from `n-2` to `1` inclusive.
            // `s[0]` must not be an LML character by definition so it is fine that `i` does not include `0`.
            let mut i = self.n - 2;
            for i_minus_1 in (0..self.n - 2).rev() {
                s_im1 = self.s[i_minus_1];
                s_im1_is_s = s_im1 < s_i || (s_im1 == s_i && s_i_is_s);
                if s_im1_is_s && !s_i_is_s {
                    // `s[i]` is LML
                    if lo_char <= s_i && s_i < hi_char {
                        let idx_in_counting_arr = (s_i - lo_char).to_usize().unwrap();
                        let idx_in_output_without_offset = &mut self.sa[idx_in_counting_arr];
                        *idx_in_output_without_offset -= 1;
                        self.sa[output_curr_head + *idx_in_output_without_offset] = i;
                    }
                }
                i = i_minus_1;
                s_i = s_im1;
                s_i_is_s = s_im1_is_s;
            }
            self.sa[0..range].fill(0); // clear the counting array
            if hi_char > self.sigma {
                break;
            }
            lo_char = hi_char;
            output_curr_head += prev; // recall that prev is the number of LML chars in this interval
        }
    }

    fn induced_sort_s_suffixes(&mut self) {
        //     let mut n_parts = T::from(4).unwrap() * self.d;
        //     let mut lo_char = T::one();
        //     let mut hi_char;
        //     // println!("sigma: {}|half_n: {}", self.sigma, self.n / 2 + 1);
        //     if false
        //     // self.sigma.to_usize().unwrap() <= self.n / 2 + 1 {
        //     // e.g. when n = 9 we can use the first 5 slots as the counting array
        //     //hi_char = self.sigma + T::one(); // lo <= c < hi, so hi = sigma + 1
        //     //counting_sort(self, lo_char, hi_char, output_curr_head);
        //     {
        //     } else {
        //         for i in T::one()..=n_parts {
        //             hi_char = i * self.sigma / n_parts + T::one();
        //             // println!("i: {}, lo: {}, hi: {}", i, lo_char, hi_char);
        //             output_curr_head += counting_sort(self, lo_char, hi_char, output_curr_head);
        //             lo_char = hi_char;
        //         }
        //     }
        //     self.sa[self.n - n1] = self.n - 1; // sentinel as a special case
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_test_utils::random_uniform_vec;
    const EXAMPLE_LI: [u8; 13] = [2, 1, 1, 3, 3, 1, 1, 3, 3, 1, 2, 1, 0];
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
        let mut sa = vec![0; 13];
        let mut solver = Li2016Ro::init(&EXAMPLE_LI, &mut sa, 3);
        solver.sort_lms_chars(4);
        println!("{:?}", sa);
    }

    #[test]
    fn test_step_1_rand() {
        let sigma = 500u32;
        for _ in 0..100 {
            let mut s = random_uniform_vec(1, sigma, 9);
            s.push(0);
            //let s = vec![148u32, 467, 426, 464, 156, 290, 314, 338, 226, 235, 0];
            let mut sa = vec![0; s.len()];
            // // println!("Input: {:?}", &s);
            // let expected = SuffixArray::from_str_very_naive(&s).sa.clone();
            let mut solver = Li2016Ro::init(&s, &mut sa, sigma);

            let (use_lms, _nl, _ns, n1) = solver.count_l_s_lms_lml();
            if use_lms {
                solver.sort_lms_chars(n1);
                println!("LMS");
                println!("{:?}", sa);
                println!("{:?}\n", s);
            } else {
                solver.sort_lml_chars(n1);
                println!("LML");
                println!("{:?}", sa);
                println!("{:?}\n", s);
            }
            // assert_eq!(&expected, &solver.sa);
        }
    }
}
// vec![148, 467, 426, 464, 156, 290, 314, 338, 226, 235, 0]
