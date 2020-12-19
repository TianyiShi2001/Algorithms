use std::collections::VecDeque;

/// # Strategy
///
/// BFS
pub struct S1;
impl S1 {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut q = VecDeque::new();
        q.extend((lo..=hi).map(|i| (i, 0, i)));
        let mut res = Vec::new();
        while let Some((curr, power, val)) = q.pop_front() {
            if curr == 1 {
                res.push(val);
            } else if curr % 2 == 0 {
                q.push_back((curr / 2, power + 1, val));
            } else {
                q.push_back((curr * 3 + 1, power + 1, val));
            }
        }
        res[k as usize - 1]
    }
}

use std::collections::HashMap;

/// # Strategy
///
/// Use a `HashMap` to memoise powers
pub struct S2;

impl S2 {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        fn power(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match memo.get(&i) {
                Some(p) => *p,
                None => {
                    let p = if i % 2 == 0 {
                        power(i / 2, memo) + 1
                    } else {
                        power(i * 3 + 1, memo) + 1
                    };
                    memo.insert(i, p);
                    p
                }
            }
        }
        let mut memo = HashMap::new();
        memo.insert(1, 0);
        let mut nums: Vec<_> = (lo..=hi).collect();
        nums.sort_by_key(|&i| power(i, &mut memo));
        nums[k as usize - 1]
    }
}
