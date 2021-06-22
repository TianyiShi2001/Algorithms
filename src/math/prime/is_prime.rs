/// Tests whether a number is a prime number or not Time Complexity: O(sqrt(n))
pub fn is_prime(n: usize) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ => {
            if n % 2 == 0 || n % 3 == 0 {
                return false;
            }
            let limit = (n as f64).sqrt() as usize;
            let mut i = 5;
            while i <= limit {
                if n % i == 0 || n % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}

// TODO: Rabin-Miller primality check, with a failure rate of (1/4)^k

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(31), true);
        assert_eq!(is_prime(1433), true);
        // assert_eq!(is_prime(8763857775536878331), true); // true; but too slow
    }
}
