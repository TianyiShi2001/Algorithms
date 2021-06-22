pub fn selection_sort<T: Ord>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }
    for i in 0..n - 1 {
        // Find the index beyond i with a lower value than i
        let mut swap_idx = i;
        for j in i + 1..n {
            if v[j] < v[swap_idx] {
                swap_idx = j;
            }
        }
        v.swap(i, swap_idx);
    }
}
