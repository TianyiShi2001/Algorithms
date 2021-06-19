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
