use crate::utils::MinMax;

/// Performs a bucket sort of an array in which all the elements are
/// bounded in the range [minValue, maxValue]. For bucket sort to give linear
/// performance the elements need to be uniformly distributed
pub fn bucket_sort<T: num_traits::PrimInt + std::fmt::Display>(v: &[T]) -> Vec<T> {
    let n = v.len();
    if n <= 1 {
        return v.to_vec();
    }
    let (min, max) = v.iter().min_max();
    let range = max - min + T::one();
    let nbuckets = range.to_usize().unwrap() / n + 1;
    let mut buckets: Vec<Vec<T>> = vec![vec![]; nbuckets];

    // place each element in a bucket
    for &num in v {
        let bi = T::from(nbuckets).unwrap() * (num - min) / range;
        buckets[bi.to_usize().unwrap()].push(num);
        println!("{:>3} placed into bucket {}", num, bi);
    }

    // sort buckets and stitch together answer
    buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.sort_unstable();
            bucket
        })
        .collect()
}
