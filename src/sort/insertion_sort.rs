/// Sort the given slice using insertion sort. The idea behind
/// insertion sort is that at the slice is already sorted from
/// [0, i] and you want to add the element at position i+1, so
/// you 'insert' it at the appropriate location.
pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}
