use super::gcd::*;

pub trait Lcm: Gcd {
    fn lcm(self, other: Self) -> Self {
        (self / self.gcd(other) * other).abs()
    }
}

pub trait LcmUnsigned: GcdUnsigned {
    fn lcm(self, other: Self) -> Self {
        (self / self.gcd(other)) * other
    }
}

impl<I: Gcd> Lcm for I {}

impl<I: GcdUnsigned> LcmUnsigned for I {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcm() {
        assert_eq!(12i32.lcm(18), 36);
        assert_eq!((-12i32).lcm(18), 36);
        assert_eq!((12i32).lcm(-18), 36);
        assert_eq!((-12i32).lcm(-18), 36);
    }
}
