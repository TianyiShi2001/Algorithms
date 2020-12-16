//! This file contains a dynamic programming solutions to the classic 0/1 knapsack problem where are
//! you are trying to maximize the total profit of items selected without exceeding the capacity of
//! your knapsack.
//!
//! Time Complexity: O(nW) Space Complexity: O(nW)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=cJ21moQpofY)

use std::cmp::max;

pub struct Item {
    value: usize,
    weight: usize,
}

impl Item {
    pub fn new(value: usize, weight: usize) -> Self {
        Self { value, weight }
    }
}

pub fn knapsack(capacity: usize, items: &[Item]) -> (usize, Vec<usize>) {
    let m = items.len();
    let n = capacity;
    // Initialize a table where individual rows represent items
    // and columns represent the weight of the knapsack.
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        let v = items[i - 1].value;
        let w = items[i - 1].weight;
        for j in 0..w {
            dp[i][j] = dp[i - 1][j];
        }
        for j in w..=n {
            dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - w] + v);
        }
    }

    let mut selection = Vec::new();

    // Using the information inside the table we can backtrack and determine
    // which items were selected during the dynamic programming phase. The idea
    // is that if DP[i][sz] != DP[i-1][sz] then the item was selected
    let mut sz = n;
    for i in (1..=m).rev() {
        if dp[i][sz] != dp[i - 1][sz] {
            let item_index = i - 1;
            selection.push(item_index);
            sz -= items[item_index].weight
        }
    }

    (dp[m][n], selection)
}

pub fn knapsack_value_only(capacity: usize, items: &[Item]) -> usize {
    let n = capacity;
    // Initialize a table where individual rows represent items
    // and columns represent the weight of the knapsack.
    // To save space, only two rows are stored.
    let mut prev = vec![0; n + 1];
    let mut curr;
    for &Item { value, weight } in items {
        curr = vec![0; n + 1];
        curr[..weight].clone_from_slice(&prev[..weight]);
        for j in weight..=n {
            curr[j] = max(prev[j], prev[j - weight] + value);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_knapsack() {
        let items = vec![
            Item::new(2, 3),
            Item::new(2, 1),
            Item::new(4, 3),
            Item::new(5, 4),
            Item::new(3, 2),
        ];
        let (max_val, selection) = knapsack(7, &items);
        assert_eq!(max_val, 10);
        assert_eq!(&selection, &[4, 3, 1]);

        assert_eq!(knapsack_value_only(7, &items), 10);
    }
}
