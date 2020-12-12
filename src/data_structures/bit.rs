use num_traits::Unsigned;

// pub struct BitArray<T: PrimInt> {
//     inner: T,
// }

pub trait Bit: Unsigned {
    fn set_bit(&mut self, pos: usize);
    fn clear_bit(&mut self, pos: usize);
    fn get_bit(&self, pos: usize) -> bool;
}

macro_rules! impl_bit {
    ($type:ty) => {
        impl Bit for $type {
            fn set_bit(&mut self, pos: usize) {
                *self = *self | (1 << pos);
            }
            fn clear_bit(&mut self, pos: usize) {
                *self = *self & !(1 << pos);
            }
            fn get_bit(&self, pos: usize) -> bool {
                (self >> pos) % 2 != 0
            }
        }
    };
}

impl_bit!(u8);
impl_bit!(u16);
impl_bit!(u32);
impl_bit!(u64);
impl_bit!(u128);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bit() {
        let mut x = 0b10101010u8;
        assert!(x.get_bit(1));
        assert!(!x.get_bit(2));
        x.set_bit(2);
        assert!(x.get_bit(2));
        x.clear_bit(2);
        assert!(!x.get_bit(2));
    }
}
