pub mod brute_force;
pub mod dp;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algo::graph::WeightedAdjacencyMatrix;
    #[test]
    fn test_tsp() {
        let mut dist = vec![vec![100.; 5]; 5];
        // Assume matrix is symmetric for simplicity.
        dist[1][3] = 1.;
        dist[3][1] = 1.;

        dist[3][0] = 2.;
        dist[0][3] = 2.;

        dist[0][2] = 3.;
        dist[2][0] = 3.;

        dist[2][4] = 4.;
        dist[4][2] = 4.;

        dist[4][1] = 5.;
        dist[1][4] = 5.;
        let dist: WeightedAdjacencyMatrix = dist.into();

        let (best_dist, tour) = dp::TspSolver::solve(&dist, 1);
        assert_eq!(best_dist, 15.);
        assert_eq!(&tour, &[1, 3, 0, 2, 4]);

        let (best_dist, tour) = brute_force::tsp(&dist, 1);
        assert_eq!(best_dist, 15.);
        assert_eq!(&tour, &[1, 3, 0, 2, 4]);
    }
}
