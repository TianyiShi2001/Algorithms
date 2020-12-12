fn log2_fast(x: usize) -> usize {
    use std::mem::size_of;
    8 * size_of::<usize>() - (x.leading_zeros() as usize) - 1
}
