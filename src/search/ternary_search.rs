//! Ternary search is similar to binary search except that it works on a function which decreases and
//! then increases. This implementation of ternary search returns the input value corresponding with
//! the minimum output value of the function you're searching on.
//!
//!  - Time Complexity: O(log(high - low)).
//!
//!  NOTE: You can also work with a function which increases and then decreases, simply negate your

use crate::utils::EPS;

/// Perform a ternary search on the interval low to high.
/// Remember that your function must be a continuous unimodal
/// function, this means a function which decreases then increases (U shape)
pub fn ternary_search<F: Fn(f64) -> f64>(mut low: f64, mut high: f64, function: F) -> f64 {
    let mut best = f64::NAN;
    loop {
        let mid1 = (2. * low + high) / 3.;
        let mid2 = (low + 2. * high) / 3.;
        let res1 = function(mid1);
        let res2 = function(mid2);
        if res1 > res2 {
            low = mid1;
        } else {
            high = mid2;
        }
        if !best.is_nan() && (best - mid1).abs() < EPS {
            break;
        }
        best = mid1;
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ternary_search() {
        // Search for the lowest point on the function x^2 + 3x + 5
        // using a ternary search on the interval [-100, +100]
        let quadratic = |x| x * x + 3. * x + 5.;
        // solve for frist derivate `2x + 3 == 0`; should be -1.5
        let minimum = ternary_search(-100., 100., quadratic);
        assert!((-1.5 - minimum).abs() < EPS);
    }
}

pub mod discrete {
    use crate::utils::EPS;
    /// Find the index at which the value is the miminum in `values`. The `values` must
    /// be a discrete and unimodal function with one and only one minimum.
    pub fn ternary_search(values: &[f64], mut lo: usize, mut hi: usize) -> usize {
        assert!(hi >= lo, "hi must be no less than lo");
        loop {
            match hi - lo {
                0 => return lo,
                1 => return if values[lo] < values[hi] { lo } else { hi },
                2 => {
                    let (mut min_idx, mut min) = (lo, values[lo]);
                    let mut v = values[lo + 1];
                    if v < min {
                        min_idx = lo + 1;
                        min = v;
                    }
                    v = values[hi];
                    if v < min {
                        min_idx = hi;
                    }
                    return min_idx;
                }
                _ => {
                    let mid1 = (2 * lo + hi) / 3;
                    let mid2 = (lo + 2 * hi) / 3;
                    let res1 = values[mid1];
                    let res2 = values[mid2];
                    if (res1 - res2).abs() < EPS {
                        lo = mid1;
                        hi = mid2;
                    } else if res1 > res2 {
                        lo = mid1;
                    } else {
                        hi = mid2;
                    }
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_ternary_search_discrete() {
            let values = [16., 12., 10., 3., 6., 7., 9., 10., 11., 12., 13., 17.];
            let min_index = ternary_search(&values, 0, values.len() - 1);
            let min_value = values[min_index];
            assert_eq!(min_index, 3);
            assert!((min_value - values[min_index]).abs() < EPS);
        }
    }
}
