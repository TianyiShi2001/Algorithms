use num_traits::{PrimInt, Signed, Unsigned};

pub trait Gcd: PrimInt + Signed {
    fn gcd(self, other: Self) -> Self {
        if other == Self::zero() {
            self.abs()
        } else {
            other.gcd(self % other)
        }
    }
    fn coprime(self, other: Self) -> bool {
        self.gcd(other) == Self::one()
    }
}

pub trait GcdUnsigned: PrimInt + Unsigned {
    fn gcd(self, other: Self) -> Self {
        if other == Self::zero() {
            self
        } else {
            other.gcd(self % other)
        }
    }
}

impl<I: Signed + PrimInt> Gcd for I {}

impl<I: Unsigned + PrimInt> GcdUnsigned for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(12i32.gcd(18), 6);
        assert_eq!((-12i32).gcd(18), 6);
        assert_eq!(12i32.gcd(-18), 6);
        assert_eq!((-12i32).gcd(-18), 6);
        assert_eq!((5i32).gcd(0), 5);
        assert_eq!((0i32).gcd(5), 5);
        assert_eq!((-5i32).gcd(0), 5);
        assert_eq!((0i32).gcd(-5), 5);
        assert_eq!((0i32).gcd(0), 0);

        assert_eq!(12u32.gcd(18), 6);
        assert_eq!(12u128.gcd(18), 6);
        assert_eq!(12i128.gcd(18), 6);
    }
    #[test]
    fn test_coprime() {
        assert!(7.coprime(12));
        assert!(!12.coprime(18));
    }
}
