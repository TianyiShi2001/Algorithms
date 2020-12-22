use num_traits::{PrimInt, Unsigned};

pub trait Factorial: PrimInt + Unsigned {
    fn factorial(self) -> Self;
}

macro_rules! impl_factorial {
    ($T:ty) => {
        impl Factorial for $T {
            fn factorial(self) -> Self {
                match self {
                    0 => 1,
                    1 => 1,
                    _ => self * (self - 1).factorial(),
                }
            }
        }
    };
}

impl_factorial!(u8);
impl_factorial!(u16);
impl_factorial!(u32);
impl_factorial!(u64);
impl_factorial!(u128);
impl_factorial!(usize);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial() {
        assert_eq!(0u32.factorial(), 1);
        assert_eq!(1u32.factorial(), 1);
        assert_eq!(2u32.factorial(), 2);
        assert_eq!(3u32.factorial(), 6);
        assert_eq!(4u32.factorial(), 24);
    }
}
