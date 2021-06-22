//! An implementation of the binary search algorithm
//!
//! - Time Complexity: O(log(high-low))

use crate::utils::EPS;

pub fn binary_search<F>(mut lo: f64, mut hi: f64, target: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    assert!(hi > lo);
    let mut mid = f64::NAN;
    while hi - lo > EPS {
        mid = (hi + lo) / 2.;
        // Compute the value of our function for the middle point
        // Note that f can be any function not just the square root function
        let value = f(mid);
        if value > target {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    mid
}
#[cfg(test)]
mod tests {
    use super::*;

    /// # EXAMPLE 1
    ///
    /// Suppose we want to know what the square root of 875 is and
    /// we have no knowledge of the wonderful `.sqrt()` function.
    /// One approach is to use a binary search because we know that
    /// the square root of 875 is bounded in the region: [0, 875].
    ///
    /// We can define our function to be f(x) = x*x and our target
    /// value to be 875. As we binary search on f(x) approaching
    /// successively closer values of 875 we get better and better
    /// values of x (the square root of 875)
    #[test]
    fn sqrt() {
        let square = |x| x * x;
        let res = binary_search(0., 875., 875., square);
        assert!((res - 875f64.sqrt()).abs() < EPS)
    }

    // # EXAMPLE #2
    //
    // Suppose we want to find the radius of a sphere with volume 100 m^3 using
    // a binary search. We know that for a sphere the volume is given by
    // V = (4/3)*pi*r^3, so all we have to do is binary search on the radius.
    //
    // Note: this is a silly example because you could just solve for r, but it
    // shows how binary search can be a powerful technique.
    #[test]
    fn radius_of_sphere() {
        let sphere_volume = |r: f64| 4. / 3. * std::f64::consts::PI * r.powi(3);
        let res = binary_search(0., 1000., 100., sphere_volume);
        // direct calculation
        let radius_from_volume = |v: f64| (v * 3. / 4. / std::f64::consts::PI).powf(1. / 3.);
        assert!((res - radius_from_volume(100.) < EPS));
    }
}
