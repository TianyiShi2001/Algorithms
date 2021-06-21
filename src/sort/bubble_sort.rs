// Sort the vector using bubble sort. The idea behind
// bubble sort is to look for adjacent indexes which
// are out of place and interchange their elements
// until the entire vector is sorted.
pub fn bubble_sort<T: Ord>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }
    loop {
        let mut sorted = true;
        for i in 1..v.len() {
            if v[i] < v[i - 1] {
                v.swap(i, i - 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}
