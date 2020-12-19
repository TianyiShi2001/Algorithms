//! - Strategy: Find degrees

pub struct S1 {}

impl S1 {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trusts = vec![0; n as usize];
        let mut trusted_by = vec![0; n as usize];
        for t in trust {
            // this question's index is 1-based
            let from = t[0] as usize - 1;
            let to = t[1] as usize - 1;
            trusts[from] += 1;
            trusted_by[to] += 1;
        }
        let mut target = -1;
        for (i, (&t, &tb)) in trusts.iter().zip(trusted_by.iter()).enumerate() {
            if t == 0 && tb == n - 1 {
                target = i as i32 + 1;
            }
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref TEST_CASES: Vec<(i32, Vec<Vec<i32>>, i32)> = vec![
            (
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]],
                3
            ),
            (3, vec![vec![1, 2], vec![2, 3]], -1),
            (3, vec![vec![1, 3], vec![2, 3]], 3)
        ];
    }
    #[test]
    fn test_q997() {
        for (arg1, arg2, res) in TEST_CASES.iter() {
            assert_eq!(S1::find_judge(*arg1, arg2.clone()), *res);
        }
    }
}
