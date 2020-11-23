use super::gcd::{gcd, gcd_unsigned};
use num_traits::{Signed, Unsigned};

pub fn lcm<T: Signed + Copy>(a: T, b: T) -> T {
    ((a / gcd(a, b)) * b).abs()
}
pub fn lcm_unsigned<T: Unsigned + Copy>(a: T, b: T) -> T {
    (a / gcd_unsigned(a, b)) * b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(-12, 18), 36);
        assert_eq!(lcm(12, -18), 36);
        assert_eq!(lcm(-12, -18), 36);
    }
}
