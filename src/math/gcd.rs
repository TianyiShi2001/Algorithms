use num_traits::{PrimInt, Signed, Unsigned};

pub trait Gcd: PrimInt + Signed {
    /// Find the greatest common divisor (GCD) of two integers using Euclidian division.
    fn gcd(self, other: Self) -> Self {
        if other == Self::zero() {
            self.abs()
        } else {
            // dividing a by b yields a quotient q and a remainder r (which is equal to a - bq); i.e.
            // a = bq + r
            // a = bq + (a - bq)
            // let k be the common divisor of a and b, then a/k and b/k are integers,
            // thus (a - bq)/k, which is r/k, must also be an integer, i.e.
            // the common divisor of a and b must also be a common divisor of the remainder of a/b.
            // Accordingly, a.gcd(b) = b.gcd(r) = b.gcd(a % b)
            other.gcd(self % other)
        }
    }
    /// Two integers a and b are coprime, relatively prime or mutually prime if the only positive
    /// integer that is a divisor of both of them is 1
    fn coprime(self, other: Self) -> bool {
        self.gcd(other) == Self::one()
    }
    /// This function performs the extended euclidean algorithm on two numbers a and b.
    /// The function returns the gcd(a,b) as well as the numbers x and y such
    /// that ax + by = gcd(a,b). This calculation is important in number theory
    /// and can be used for several things such as finding modular inverses and
    /// solutions to linear Diophantine equations.
    ///
    /// - Returns: `(r, s, t)`, where `s * self + t * other = r`, where `r = self.gcd(other)`
    ///
    /// # Resources
    ///
    /// - [Wikipedia](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
    /// - [CP Algorithms](https://cp-algorithms.com/algebra/extended-euclid-algorithm.html)
    fn egcd(self, other: Self) -> (Self, Self, Self) {
        if other == Self::zero() {
            (self, Self::one(), Self::zero())
        } else {
            let (r, s, t) = other.egcd(self % other);
            (r, t, s - t * (self / other))
        }
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
    fn coprime(self, other: Self) -> bool {
        self.gcd(other) == Self::one()
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
        assert!(7i32.coprime(12));
        assert!(!12i32.coprime(18));

        assert!(7u32.coprime(12));
        assert!(!12u32.coprime(18));
    }

    #[test]
    fn test_egcd() {
        let (a, b) = (240i32, 46i32);
        let (r, s, t) = a.egcd(b);
        assert_eq!(r, a.gcd(b));
        assert_eq!(r, s * a + t * b);
    }
}
