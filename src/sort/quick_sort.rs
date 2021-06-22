// TODO: implement more variants; add debug output to help explain clearly

use rand::{thread_rng, Rng};

pub fn quick_sort<T: Ord>(v: &mut [T]) {
    let mut rng = thread_rng();
    _quick_sort(v, &mut rng);
}

fn _quick_sort<T: Ord, R: Rng>(v: &mut [T], rng: &mut R) {
    let n = v.len();
    if n <= 1 {
        return;
    }
    let pivot_idx = rng.gen_range(0..n);
    // move the pivot to the end
    v.swap(pivot_idx, n - 1);
    let pivot_v = &v[n - 1] as *const T;
    // Look for item from left that is larger than the pivot
    // and item from right that is smaller than the pivot, then
    // swap them, repeat until the left pointer proceeds beyond
    // the right pointer.
    // Finally, swap the pivot (which has previously been moved
    // to the end) with `left`, so that all items to the left are
    // smaller than the pivot and all items to the right are larger
    // than the pivot.
    // This is referred to as the Hoare partitioning.
    let mut l = 0;
    let mut r = n - 2;
    loop {
        loop {
            // SAFETY: pivot_v will not be changed until we
            // break the outer loop
            if l == n - 1 || &v[l] > unsafe { &*pivot_v } {
                break;
            }
            l += 1;
        }
        loop {
            if r == 0 || &v[r] < unsafe { &*pivot_v } {
                break;
            }
            r -= 1;
        }
        if l < r {
            v.swap(l, r);
        } else {
            // swap the pivot value back to the new pivot index, which is l
            v.swap(l, n - 1);
            break;
        }
    }
    // Now, we can sort items to the left of the pivot (which are all smaller
    // than the pivot) and items to the right of the pivot (which are all
    // greater than the pivot) separately.

    let (left, right) = v.split_at_mut(l);
    _quick_sort(left, rng);
    if l != n - 1 {
        // [1..] to skip the pivot at index 0;
        // out of bound if `l == n - 1` i.e. `right.len() == 0` so need to check
        _quick_sort(&mut right[1..], rng);
    }
}
