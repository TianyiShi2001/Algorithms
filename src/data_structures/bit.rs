use num_traits::Unsigned;

// pub struct BitArray<T: PrimInt> {
//     inner: T,
// }

pub trait Bit: Unsigned {
    fn set_bit(&mut self, pos: usize);
    fn clear_bit(&mut self, pos: usize);
    fn get_bit(&self, pos: usize) -> bool;
    fn toggle_bit(&mut self, pos: usize);
    /// Returns a number with the first n bits set to 1
    fn set_all(&mut self, pos: usize);
    fn is_power_of_two(&self) -> bool;
}

macro_rules! impl_bit {
    ($type:ty) => {
        impl Bit for $type {
            fn set_bit(&mut self, pos: usize) {
                *self |= (1 << pos);
            }
            fn clear_bit(&mut self, pos: usize) {
                *self &= !(1 << pos);
            }
            fn get_bit(&self, pos: usize) -> bool {
                (self >> pos) % 2 != 0
            }
            // Toggles the i'th bit from 0 -> 1 or 1 -> 0
            fn toggle_bit(&mut self, pos: usize) {
                *self ^= (1 << pos);
            }
            fn set_all(&mut self, pos: usize) {
                *self = (1 << pos) - 1;
            }
            fn is_power_of_two(&self) -> bool {
                *self > 0 && *self & (*self - 1) == 0
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
        x.toggle_bit(2);
        assert!(x.get_bit(2));
        x.set_all(5);
        assert_eq!(x, 0b00011111);
        assert!((x + 1).is_power_of_two());
    }
}
