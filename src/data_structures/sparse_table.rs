use std::fmt::Debug;

pub struct SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    // The sparse table values.
    values: Vec<Vec<Option<T>>>,
    // Index Table associated with the values in the sparse table.
    log2: Vec<usize>,
    f: F,
}

impl<T: Clone + Debug, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new(arr: &[T], f: F) -> Self {
        let n = arr.len();
        let log2 = Self::build_log2(n);
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
        Self { values, log2, f }
    }
    fn build_log2(n: usize) -> Vec<usize> {
        let mut log2 = vec![0usize; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }
        log2
    }
    pub fn query(&self, l: usize, r: usize) -> T {
        let len = r - l + 1;
        let i = self.log2[len];
        let left_interval = self.values[i][l].clone();
        let right_interval = self.values[i][1 + r - (1 << i)].clone();
        (self.f)(left_interval.unwrap(), right_interval.unwrap())
    }
    pub fn query_acc(&self, mut l: usize, r: usize) -> T {
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

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use rand::{thread_rng, Rng};

    const SAMPLE_SIZE: usize = 10;
    lazy_static! {
        static ref TEST_DATA: Vec<i32> = {
            let mut rng = thread_rng();
            (0..SAMPLE_SIZE).map(|_| rng.gen_range(-10, 10)).collect()
        };
    }

    fn validate<F>(f: F, overlap_ok: bool)
    where
        F: Fn(i32, i32) -> i32,
    {
        let sparse_table = SparseTable::new(&TEST_DATA, f);
        for i in 0..SAMPLE_SIZE - 1 {
            for j in i..SAMPLE_SIZE {
                let expected = TEST_DATA[i + 1..=j]
                    .iter()
                    .fold(TEST_DATA[i], |acc, curr| (sparse_table.f)(acc, *curr));
                let quried = if overlap_ok {
                    sparse_table.query(i, j)
                } else {
                    sparse_table.query_acc(i, j)
                };
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
    fn test_sparse_table_sum_mul() {
        validate(|a, b| a + b, false);
        validate(|a, b| a * b, false);
    }
}
