use crate::algo::graph::WeightedAdjacencyMatrix;
use crate::algo::math::permutations::*;

pub fn tsp(g: &WeightedAdjacencyMatrix, start: usize) -> (f64, Vec<usize>) {
    let n = g.vertices_count();
    let permutations = (0..n)
        .filter(|&i| i != start)
        .collect::<Vec<_>>()
        .permutations();
    let mut tour = vec![];
    let mut best_tour_cost = f64::INFINITY;
    for perm in permutations {
        let perm = unsafe { &*perm };
        let cost = compute_tour_cost(g, perm, start);
        if cost < best_tour_cost {
            best_tour_cost = cost;
            tour = perm.to_owned();
        }
    }
    tour.insert(0, start);
    (best_tour_cost, tour)
}

fn compute_tour_cost(g: &WeightedAdjacencyMatrix, tour: &[usize], start: usize) -> f64 {
    tour.windows(2)
        .fold(0., |cost, step| cost + g[step[0]][step[1]])
        + g[start][*tour.first().unwrap()]
        + g[*tour.last().unwrap()][start]
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::algo::graph::WeightedAdjacencyList;
//     #[test]
//     fn test_tsp_brute_force() {
//         let g: WeightedAdjacencyMatrix = WeightedAdjacencyList::new_undirected(
//             6,
//             &[
//                 (5, 0, 10.),
//                 (1, 5, 12.),
//                 (4, 1, 2.),
//                 (2, 4, 4.),
//                 (3, 2, 6.),
//                 (0, 3, 8.),
//             ],
//         )
//         .into();
//         assert_eq!(&tsp(&g), &[0, 3, 2, 4, 1, 5, 0]);
//     }
// }
