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

/// Use the sieve of eratosthenes to find all the prime numbers up to a certain limit.
///
/// - Time Complexity: O(nlog(logn))
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

use crate::math::gcd::GcdUnsigned;
use rand::{thread_rng, Rng};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn prime_factorize(n: usize) -> Vec<usize> {
    println!("Now factorizing {}", n);
    let mut rng = thread_rng();
    let mut pollard_rho = |n: usize| -> usize {
        if n % 2 == 0 {
            2
        } else {
            let mut x = 2 + rng.gen_range(0..1000000usize);
            let c = 2 + rng.gen_range(0..1000000usize);
            let mut y = x;
            let mut d = 1;
            while d == 1 {
                x = (x * x + c) % n;
                y = (y * y + c) % n;
                y = (y * y + c) % n;
                d = (if x > y { x - y } else { y - x }).gcd(n);
                if d == n {
                    break;
                }
            }
            d
        }
    };

    let mut factors = Vec::new();
    match n {
        0 | 1 => (),
        _ => {
            let mut divisor_queue = BinaryHeap::new();
            divisor_queue.push(Reverse(n));
            while let Some(Reverse(divisor)) = divisor_queue.pop() {
                println!("popping {}", divisor);
                if is_prime(divisor) {
                    factors.push(divisor);
                    continue;
                }
                let next_divisor = pollard_rho(divisor);
                println!("Next divisor: {}", next_divisor);
                if next_divisor == divisor {
                    divisor_queue.push(Reverse(divisor));
                } else {
                    divisor_queue.push(Reverse(next_divisor));
                    divisor_queue.push(Reverse(divisor / next_divisor));
                }
            }
        }
    }

    factors.sort();
    factors
}

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

    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes = sieve_of_eratosthenes(64);
        assert_eq!(
            primes,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61]
        )
    }

    #[test]
    fn test_prime_factorize() {
        fn _test_prime_factorise(n: usize, expected: Vec<usize>) {
            let calculated = prime_factorize(n);
            assert_eq!(calculated, expected);
        }
        _test_prime_factorise(7, vec![7]);
        _test_prime_factorise(100, vec![2, 2, 5, 5]);
        _test_prime_factorise(666, vec![2, 3, 3, 37]);
        _test_prime_factorise(872342345, vec![5, 7, 7, 19, 67, 2797]);
    }
}
