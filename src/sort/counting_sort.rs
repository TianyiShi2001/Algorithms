use crate::utils::MinMax;

// Works best when the range of the values (i.e. max_value - min_value) is small.
//
// - Time complexity: O(n+maxVal-maxVal)
pub fn counting_sort<T: num_traits::PrimInt>(v: &mut [T]) {
    let (min, max) = v.iter().min_max();

    let sz = (max - min + T::one()).to_usize().unwrap();
    // `frequency[i]` stores how many times `i` occured in the vector to be sorted;
    // later, we can then build the sorted vector by reading the `frequency` from
    // left to right, pushing the value `i` to the sorted vector `frequency[i]` times.
    let mut frequency = vec![0; sz];
    for m in v.iter().map(|&n| (n - min).to_usize().unwrap()) {
        frequency[m] += 1;
    }
    let mut k = 0;
    for (idx, &freq) in frequency.iter().enumerate().take(sz) {
        let val = T::from(idx).unwrap() + min;
        for _ in 0..freq {
            v[k] = val;
            k += 1;
        }
    }
}
