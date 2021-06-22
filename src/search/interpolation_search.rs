//! An implementation of interpolation search
//!
//! - Time Complexity: O(log(log(n))) if data is uniform; O(n) in worst case
//!
//!  A fast alternative to a binary search when the elements are uniformly distributed. This
//!  algorithm runs in a time complexity of ~ O(log(log(n))).
//!
//! # Params
//!
//! `nums`  - an ordered list containing uniformly distributed values.
//! `target` - the value we're looking for in `items`

use std::cmp::Ordering;

pub fn interpolation_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = nums.len() - 1;
    let mut mid;
    while hi > lo {
        mid =
            lo + (((target - nums[lo]) / (nums[hi] - nums[lo])) as f64 * (hi - lo) as f64) as usize;
        match nums[mid].cmp(&target) {
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }
    // hi == lo
    if nums[lo] == target {
        Some(lo)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpolation_search() {
        let values = vec![10, 20, 25, 35, 50, 70, 85, 100, 110, 120, 125];
        // Since 25 exists in the values array the interpolation search
        // returns that it has found 25 at the index 2
        assert_eq!(interpolation_search(&values, 25), Some(2));
        // 111 does not exist
        assert_eq!(interpolation_search(&values, 111), None);
    }
}
