//! This mod contains a recursive implementation of the TSP problem using dynamic programming. The
//! main idea is that since we need to do all n! permutations of nodes to find the optimal solution
//! that caching the results of sub paths can improve performance.
//!
//! For example, if one permutation is: `... D A B C` then later when we need to compute the value
//! of the permutation `... E B A C` we should already have cached the answer for the subgraph
//! containing the nodes `{A, B, C}`.
//!
//! - Time Complexity: O(n^2 * 2^n) Space Complexity: O(n * 2^n)
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=cY4HiiFHO1o&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=25)
//! - [W. Fiset's video](https://www.youtube.com/watch?v=cY4HiiFHO1o&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=25)

use crate::algo::graph::WeightedAdjacencyMatrix;
use crate::data_structures::bit::Bit;

pub struct TspSolver {}

impl TspSolver {
    #[allow(clippy::needless_range_loop)]
    pub fn solve(distance: &WeightedAdjacencyMatrix, start: usize) -> (f64, Vec<usize>) {
        let n = distance.vertices_count();
        let mut memo = vec![vec![f64::INFINITY; 1 << n]; n];
        // store the optimal distance from the start node to each node `i`
        for i in 0..n {
            memo[i][1 << i | 1 << start] = distance[start][i];
        }

        let mut memo = vec![vec![f64::INFINITY; 1 << n]; n];
        // store the optimal distance from the start node to each node `i`
        for i in 0..n {
            memo[i][1 << i | 1 << start] = distance[start][i];
        }
        for r in 3..=n {
            for state in BinaryCombinations::new(n, r as u32).filter(|state| state.get_bit(start)) {
                for next in (0..n).filter(|&node| state.get_bit(node) && node != start) {
                    // the state without the next node
                    let prev_state = state ^ (1 << next);
                    let mut min_dist = f64::INFINITY;
                    for prev_end in
                        (0..n).filter(|&node| state.get_bit(node) && node != start && node != next)
                    {
                        let new_dist = memo[prev_end][prev_state] + distance[prev_end][next];
                        if new_dist < min_dist {
                            min_dist = new_dist;
                        }
                    }
                    memo[next][state] = min_dist;
                }
            }
        }

        // the end state is the bit mask with `n` bits set to 1
        let end_state = (1 << n) - 1;
        let mut min_dist = f64::INFINITY;
        for e in (0..start).chain(start + 1..n) {
            let dist = memo[e][end_state] + distance[e][start];
            if dist < min_dist {
                min_dist = dist;
            }
        }

        let mut state = end_state;
        let mut last_index = start;
        let mut tour = vec![start];
        for _ in 1..n {
            let mut best_j = usize::MAX;
            let mut best_dist = f64::MAX;
            for j in (0..n).filter(|&j| state.get_bit(j) && j != start) {
                let dist = memo[j][state] + distance[j][last_index];
                if dist < best_dist {
                    best_j = j;
                    best_dist = dist;
                }
            }
            tour.push(best_j);
            state ^= 1 << best_j;
            last_index = best_j;
        }

        (min_dist, tour)
    }
}
pub struct BinaryCombinations {
    curr: usize,
    r: u32,
    n: usize,
}

impl Iterator for BinaryCombinations {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.curr..1 << self.n {
            if i.count_ones() == self.r {
                self.curr = i + 1;
                return Some(i);
            }
        }
        None
    }
}

impl BinaryCombinations {
    pub fn new(n: usize, r: u32) -> Self {
        Self { curr: 0, r, n }
    }
}

// // To find all the combinations of size r we need to recurse until we have
// // selected r elements (aka r = 0), otherwise if r != 0 then we still need to select
// // an element which is found after the position of our last selected element
// fn combinations(mut set: u32, at: u32, r: u32, n: u32, subsets: &mut Vec<u32>) {
//     // Return early if there are more elements left to select than what is available.
//     let elements_left_to_pick = n - at;
//     if elements_left_to_pick < r {
//         return;
//     }

//     // We selected 'r' elements so we found a valid subset!
//     if r == 0 {
//         subsets.push(set);
//     } else {
//         for i in at..n {
//             // Try including this element
//             set ^= 1 << i;

//             combinations(set, i + 1, r - 1, n, subsets);

//             // Backtrack and try the instance where we did not include this element
//             set ^= 1 << i;
//         }
//     }
// }
