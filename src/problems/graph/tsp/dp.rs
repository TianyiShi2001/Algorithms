use crate::algo::graph::WeightedAdjacencyMatrix;
use crate::data_structures::bit::Bit;

pub struct TspSolver<'a> {
    memo: Vec<Vec<f32>>,
    distance: &'a WeightedAdjacencyMatrix,
}

impl<'a> TspSolver<'a> {
    #[allow(clippy::needless_range_loop)]
    pub fn solve(distance: &'a WeightedAdjacencyMatrix, start: usize) -> (f32, Vec<usize>) {
        let n = distance.vertices_count();
        let mut memo = vec![vec![f32::INFINITY; 1 << n]; n];
        // store the optimal distance from the start node to each node `i`
        for i in 0..n {
            memo[i][1 << i | 1 << start] = distance[start][i];
        }
        let mut solver = Self { memo, distance };
        solver._solve(n, start)
    }
    fn _solve(&mut self, n: usize, start: usize) -> (f32, Vec<usize>) {
        for r in 3..=n {
            for state in BinaryCombinations::new(n, r as u32).filter(|state| state.get_bit(start)) {
                for next in (0..n).filter(|&node| state.get_bit(node) && node != start) {
                    // the state without the next node
                    let prev_state = state ^ (1 << next);
                    let mut min_dist = f32::INFINITY;
                    for prev_end in
                        (0..n).filter(|&node| state.get_bit(node) && node != start && node != next)
                    {
                        let new_dist =
                            self.memo[prev_end][prev_state] + self.distance[prev_end][next];
                        if new_dist < min_dist {
                            min_dist = new_dist;
                        }
                    }
                    self.memo[next][state] = min_dist;
                }
            }
        }

        // the end state is the bit mask with `n` bits set to 1
        let end_state = (1 << n) - 1;
        let mut min_dist = f32::INFINITY;
        for e in (0..start).chain(start + 1..n) {
            let dist = self.memo[e][end_state] + self.distance[e][start];
            if dist < min_dist {
                min_dist = dist;
            }
        }

        let mut state = end_state;
        let mut last_index = start;
        let mut tour = vec![start];
        for _ in 1..n {
            let mut best_j = usize::MAX;
            let mut best_dist = f32::MAX;
            for j in (0..n).filter(|&j| state.get_bit(j) && j != start) {
                let dist = self.memo[j][state] + self.distance[j][last_index];
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
