use super::is_prime::is_prime;
use crate::math::gcd::GcdUnsigned;
use rand::{thread_rng, Rng};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(clippy::many_single_char_names)]
pub fn prime_factorize(n: usize) -> Vec<usize> {
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
                if is_prime(divisor) {
                    factors.push(divisor);
                    continue;
                }
                let next_divisor = pollard_rho(divisor);
                if next_divisor == divisor {
                    divisor_queue.push(Reverse(divisor));
                } else {
                    divisor_queue.push(Reverse(next_divisor));
                    divisor_queue.push(Reverse(divisor / next_divisor));
                }
            }
        }
    }

    factors.sort_unstable();
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

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
