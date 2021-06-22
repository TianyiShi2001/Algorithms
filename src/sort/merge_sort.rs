pub fn merge_sort<T: Ord + Default + Copy>(v: &[T]) -> Vec<T> {
    let n = v.len();
    if n <= 1 {
        return v.to_vec();
    }
    let mid = n / 2;
    let l = merge_sort(&v[..mid]);
    let r = merge_sort(&v[mid..]);
    merge(&l, &r)
}

/// Merge two sorted arrays into a larger sorted array
#[allow(clippy::collapsible_else_if)]
fn merge<T: Ord + Default + Copy>(v1: &[T], v2: &[T]) -> Vec<T> {
    let n1 = v1.len();
    let n2 = v2.len();
    let n = n1 + n2;
    let mut i1 = 0;
    let mut i2 = 0;

    let mut merged = vec![T::default(); n];
    for m in &mut merged {
        if i1 == n1 {
            *m = v2[i2];
            i2 += 1;
        } else if i2 == n2 {
            *m = v1[i1];
            i1 += 1;
        } else {
            if v1[i1] < v2[i2] {
                *m = v1[i1];
                i1 += 1;
            } else {
                *m = v2[i2];
                i2 += 1;
            }
        }
    }
    merged
}
