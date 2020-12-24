//! Implementation of a sparse table which is a data structure that can very quickly query a range on
//! a static array in $O(1)$ for overlap-friendly functions (idempotent functions) using $O(n*logn)$ memory.
//! For functions that are only associative, the query is done in $O(log(n))$.
//!
//! A function $f$ is associative if $f(a, f(b, c)) = f(f(a, b), c)$. Examples include scalar and matrix
//! addition and multiplication, and string concatenation.
//! A function is overlap-freindly if $f(f(a, b), f(b, c)) = f(f(a, b), c)$. Examples include min, max, gcd and lcm.

use crate::math::log2::VecLog2;

pub struct SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    // The sparse table values.
    values: Vec<Vec<Option<T>>>,
    // Pre-computed array of log2 values
    log2: Vec<usize>,
    // The function to be applied
    f: F,
    overlap_friendly: bool,
}

impl<T: Clone, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new(arr: &[T], f: F, overlap_friendly: bool) -> Self {
        let n = arr.len();
        let log2 = Vec::log2(n + 1);
        let m = log2[n];
        let mut values = vec![vec![None; n]; m + 1];
        for (i, v) in arr.iter().enumerate() {
            values[0][i] = Some(v.clone());
        }
        // Build sparse table combining the values of the previous intervals.
        for i in 1..=m {
            for j in 0..=(n - (1 << i)) {
                let left_interval = values[i - 1][j].clone();
                let right_interval = values[i - 1][j + (1 << (i - 1))].clone();
                values[i][j] = Some(f(left_interval.unwrap(), right_interval.unwrap()));
            }
        }
        Self {
            values,
            log2,
            f,
            overlap_friendly,
        }
    }
    pub fn query(&self, mut l: usize, r: usize) -> T {
        if self.overlap_friendly {
            let len = r - l + 1;
            let i = self.log2[len];
            let left_interval = self.values[i][l].clone();
            let right_interval = self.values[i][1 + r - (1 << i)].clone();
            (self.f)(left_interval.unwrap(), right_interval.unwrap())
        } else {
            let mut p = self.log2[1 + r - l];
            let mut acc = self.values[p][l].clone().unwrap();
            l += 1 << p;
            while l <= r {
                p = self.log2[1 + r - l];
                acc = (self.f)(acc, self.values[p][l].clone().unwrap());
                l += 1 << p;
            }

            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use rand::{thread_rng, Rng};

    const SAMPLE_SIZE: usize = 10;
    lazy_static! {
        static ref TEST_DATA: Vec<u128> = {
            let mut rng = thread_rng();
            (0..SAMPLE_SIZE).map(|_| rng.gen_range(1..20)).collect()
        };
    }

    fn validate<F>(f: F, overlap_ok: bool)
    where
        F: Fn(u128, u128) -> u128,
    {
        let sparse_table = SparseTable::new(&TEST_DATA, f, overlap_ok);
        for i in 0..SAMPLE_SIZE - 1 {
            for j in i..SAMPLE_SIZE {
                let expected = TEST_DATA[i + 1..=j]
                    .iter()
                    .fold(TEST_DATA[i], |acc, curr| (sparse_table.f)(acc, *curr));
                let quried = sparse_table.query(i, j);
                assert_eq!(expected, quried);
            }
        }
    }

    #[test]
    fn test_sparse_table_min_max() {
        validate(std::cmp::min, true);
        validate(std::cmp::max, true);
    }

    #[test]
    fn test_sparse_table_add_mul() {
        validate(|a, b| a + b, false);
        validate(|a, b| a * b, false);
    }

    #[test]
    fn test_gcd_lcm() {
        use crate::math::{gcd::GcdUnsigned, lcm::LcmUnsigned};
        validate(|a, b| a.gcd(b), true);
        validate(|a, b| a.lcm(b), true);
    }

    #[test]
    fn test_string_concat() {
        let mut rng = thread_rng();
        let data: Vec<String> = (0..SAMPLE_SIZE * 4)
            .map(|_| rng.gen_range(b'a'..b'z'))
            .collect::<Vec<_>>()
            .chunks_exact(4)
            .map(|x| unsafe { String::from_utf8_unchecked(vec![x[0], x[1], x[2], x[3]]) })
            .collect();
        let sparse_table = SparseTable::new(&data, |a, b| a + &b, false);
        for i in 0..SAMPLE_SIZE - 1 {
            for j in i..SAMPLE_SIZE {
                let expected = data[i + 1..=j].iter().fold(data[i].clone(), |acc, curr| {
                    (sparse_table.f)(acc, curr.clone())
                });
                let quried = sparse_table.query(i, j);
                assert_eq!(expected, quried);
            }
        }
    }

    #[test]
    fn test_sparse_table_matrix_add_mul() {
        let mut rng = thread_rng();
        let data: Vec<Matrix2x2> = (0..SAMPLE_SIZE * 4)
            .map(|_| rng.gen_range(-10i128..10))
            .collect::<Vec<_>>()
            .chunks_exact(4)
            .map(|x| [[x[0], x[1]], [x[2], x[3]]])
            .collect();

        let sparse_table_add = SparseTable::new(&data, matrix_addition_2x2, false);
        let sparse_table_mul = SparseTable::new(&data, matrix_multiplication_2x2, false);
        for i in 0..SAMPLE_SIZE - 1 {
            for j in i..SAMPLE_SIZE {
                let (expected_sum, expected_product) =
                    data[i + 1..=j]
                        .iter()
                        .fold((data[i], data[i]), |acc, curr| {
                            (
                                matrix_addition_2x2(acc.0, *curr),
                                matrix_multiplication_2x2(acc.1, *curr),
                            )
                        });
                let quried_sum = sparse_table_add.query(i, j);
                let quried_product = sparse_table_mul.query(i, j);
                assert_eq!(expected_sum, quried_sum);
                assert_eq!(expected_product, quried_product);
            }
        }
    }

    type Matrix2x2 = [[i128; 2]; 2];
    fn matrix_addition_2x2(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
        [
            [a[0][0] + b[0][0], a[0][1] + b[0][1]],
            [a[1][0] + b[1][0], a[1][1] + b[1][1]],
        ]
    }
    fn matrix_multiplication_2x2(a: Matrix2x2, b: Matrix2x2) -> Matrix2x2 {
        [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]
    }
}
