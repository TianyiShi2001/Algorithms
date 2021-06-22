/// Use the sieve of eratosthenes to find all the prime numbers up to a certain limit.
///
/// - Time Complexity: O(nlog(logn))
#[allow(clippy::needless_range_loop)]
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n <= 2 {
        vec![]
    } else {
        let sqrt_limit = (n as f64).sqrt();
        // Find an upper bound on the number of prime numbers up to our limit.
        // https://en.wikipedia.org/wiki/Prime-counting_function#Inequalities
        let max_num_primes = (1.25506 * n as f64 / sqrt_limit) as usize;
        let mut primes = Vec::with_capacity(max_num_primes);
        let mut is_composite = vec![false; n as usize];
        for i in 2..=sqrt_limit as usize {
            if !is_composite[i] {
                primes.push(i);
                let mut j = i * i;
                while j < n {
                    is_composite[j] = true;
                    j += i;
                }
            }
        }
        for i in sqrt_limit as usize + 1..n {
            if !is_composite[i] {
                primes.push(i);
            }
        }
        primes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes = sieve_of_eratosthenes(0x100);
        assert_eq!(
            primes,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251
            ]
        )
    }
}

pub mod memory_optimised {

    const BITS: usize = 64;
    const SHIFT: usize = 7; // 2 * 7 = 128

    /// Generate a compressed prime sieve using bit manipulation. The idea is that each bit represents a
    /// boolean value indicating whether a number is prime or not. This saves a lot of room when creating
    /// the sieve. In this implementation I store all odd numbers in individual `u64`s meaning that for
    /// each `u64` I use I can represent a range of 128 numbers (even numbers are omitted because they are
    /// not prime, with the exception of 2 which is handled as a special case).
    pub struct SieveOfEratosthenes {
        /// A `Vec` of `u64` with each bit indicating whether a number
        /// is prime or not.
        chunks: Vec<u64>,
        limit: usize,
    }

    impl SieveOfEratosthenes {
        pub fn new(limit: usize) -> Self {
            let num_chunks = (limit as f64 / BITS as f64).ceil() as usize;
            let sqrt_limit = (limit as f64).sqrt() as usize;
            let mut chunks = vec![0u64; num_chunks];
            chunks[0] = 1; // 1 is not prime
            let mut slf = Self { chunks, limit };
            let mut i = 3;

            while i <= sqrt_limit {
                if slf.is_prime(i) {
                    let mut j = i * i;
                    while j <= limit {
                        if slf.is_not_set(j) {
                            slf.set_bit(j);
                        }
                        j += i;
                    }
                }
                i += 2;
            }
            slf
        }
        /// Sets the bit representing n to 1 indicating this number is not prime
        fn set_bit(&mut self, n: usize) {
            // if n is not even
            if n & 1 != 0 {
                self.chunks[n >> SHIFT] |= 1u64.wrapping_shl(n as u32 >> 1);
            }
        }

        // Returns true if the bit for n is off (meaning n is a prime).
        // Note: do not use this method to access numbers outside your prime sieve range!
        fn is_not_set(&self, n: usize) -> bool {
            match n {
                0 | 1 => false,
                2 => true,
                _ => {
                    if n & 1 == 0 {
                        false
                    } else {
                        let chunk = self.chunks[n >> SHIFT];
                        let mask = 1u64.wrapping_shl(n as u32 >> 1);
                        chunk & mask == 0
                    }
                }
            }
        }
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_not_set(n)
        }
        pub fn iter(&self) -> Iter {
            Iter {
                curr: 1,
                inner: self,
            }
        }
    }

    pub struct Iter<'a> {
        curr: usize,
        inner: &'a SieveOfEratosthenes,
    }
    impl<'a> Iterator for Iter<'a> {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            while self.curr < self.inner.limit {
                self.curr += 1;
                if self.inner.is_prime(self.curr) {
                    return Some(self.curr);
                }
            }
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sieve() {
            let sieve = SieveOfEratosthenes::new(0x100);
            let primes: Vec<usize> = sieve.iter().collect();
            assert_eq!(
                primes,
                vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                    163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241,
                    251
                ]
            )
        }
    }
}
