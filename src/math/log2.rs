use num_traits::PrimInt;

pub trait IntLog2: PrimInt {
    fn mylog2(self) -> Self {
        Self::from(8 * std::mem::size_of::<Self>() - (self.leading_zeros() as usize) - 1).unwrap()
    }
}

impl<I: PrimInt> IntLog2 for I {}

pub trait VecLog2 {
    fn mylog2(n: usize) -> Self;
}

impl<I: PrimInt> VecLog2 for Vec<I> {
    fn mylog2(n: usize) -> Vec<I> {
        let mut log2 = vec![I::zero(); n];
        for i in 2..n {
            log2[i] = log2[i / 2] + I::one();
        }
        log2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log2() {
        assert_eq!(0x10u8.mylog2(), 4);
        assert_eq!(0x0111u16.mylog2(), 8);
        assert_eq!(0x0101i32.mylog2(), 8);
    }

    #[test]
    fn test_log2_vec() {
        let log2 = Vec::<i128>::mylog2(17);
        let expected = &[0i128, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4];
        assert_eq!(&log2, expected);
    }
}
