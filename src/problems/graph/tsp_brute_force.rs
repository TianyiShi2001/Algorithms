use crate::algo::graph::WeightedAdjacencyMatrix;
use crate::algo::math::permutations::*;

pub fn tsp(g: &WeightedAdjacencyMatrix) -> Vec<usize> {
    let n = g.vertices_count();
    let permutations = (0..n).collect::<Vec<_>>().permutations();
    let mut tour = Vec::new();
    let mut best_tour_cost = f32::INFINITY;
    for perm in permutations {
        let perm = unsafe { &*perm };
        let cost = compute_tour_cost(g, perm);
        if cost < best_tour_cost {
            best_tour_cost = cost;
            tour = perm.to_owned();
        }
    }
    tour.push(tour[0]);
    tour
}

fn compute_tour_cost(g: &WeightedAdjacencyMatrix, tour: &[usize]) -> f32 {
    tour.windows(2)
        .fold(0., |cost, step| cost + g[step[0]][step[1]])
        + g[*tour.last().unwrap()][*tour.first().unwrap()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::graph::WeightedAdjacencyList;
    #[test]
    fn test_tsp_brute_force() {
        let g: WeightedAdjacencyMatrix = WeightedAdjacencyList::new_undirected(
            6,
            &[
                (5, 0, 10.),
                (1, 5, 12.),
                (4, 1, 2.),
                (2, 4, 4.),
                (3, 2, 6.),
                (0, 3, 8.),
            ],
        )
        .into();
        assert_eq!(&tsp(&g), &[0, 3, 2, 4, 1, 5, 0]);
    }
}
