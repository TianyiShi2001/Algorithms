use num_traits::{Signed, Unsigned};

pub fn gcd<T: Signed + Copy>(a: T, b: T) -> T {
    if b == T::zero() {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

pub fn gcd_unsigned<T: Unsigned + Copy>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd_unsigned(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(-12, 18), 6);
        assert_eq!(gcd(12, -18), 6);
        assert_eq!(gcd(-12, -18), 6);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(-5, 0), 5);
        assert_eq!(gcd(0, -5), 5);
        assert_eq!(gcd(0, 0), 0);
    }
}
