use num_traits::PrimInt;

pub fn log2<I: PrimInt>(x: I) -> usize {
    8 * std::mem::size_of::<I>() - (x.leading_zeros() as usize) - 1
}

pub fn log2_vec<I: PrimInt>(n: usize) -> Vec<I> {
    let mut log2 = vec![I::zero(); n];
    for i in 2..n {
        log2[i] = log2[i / 2] + I::one();
    }
    log2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log2() {
        assert_eq!(log2(0x10u8), 4);
        assert_eq!(log2(0x0111u16), 8);
        assert_eq!(log2(0x0101i32), 8);
    }

    #[test]
    fn test_log2_vec() {
        let log2 = log2_vec::<i128>(17);
        let expected = &[0i128, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4];
        assert_eq!(&log2, expected);
    }
}
