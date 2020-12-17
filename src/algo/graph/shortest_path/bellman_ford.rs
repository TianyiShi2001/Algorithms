//! An implementation of the Bellman-Ford algorithm. The algorithm finds the shortest path between
//! a starting node and all other nodes in the graph. The algorithm also detects negative cycles.
//! If a node is part of a negative cycle then the minimum cost for that node is set to
//! `f64::NEG_INFINITY`

use crate::algo::graph::{Edge, WeightedAdjacencyList};

impl WeightedAdjacencyList {
    pub fn bellman_ford(&self, start: usize) -> Vec<f64> {
        // Initialize the distance to all nodes to be infinity
        // except for the start node which is zero.
        let n = self.node_count();
        let mut dists = vec![f64::INFINITY; n];
        dists[start] = 0.;

        // For each vertex, apply relaxation for all the edges
        for _ in 1..n {
            for (from, edges) in self.vertices() {
                for &Edge { to, cost } in edges {
                    let new_cost = dists[from] + cost;
                    if new_cost < dists[to] {
                        dists[to] = new_cost;
                    }
                }
            }
        }

        // Run algorithm a second time to detect which nodes are part
        // of a negative cycle. A negative cycle has occurred if we
        // can find a better path beyond the optimal solution.
        for _ in 1..n {
            for (from, edges) in self.vertices() {
                for &Edge { to, cost } in edges {
                    if dists[from] + cost < dists[to] {
                        dists[to] = f64::NEG_INFINITY;
                    }
                }
            }
        }

        dists
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bellman_ford() {
        let graph = WeightedAdjacencyList::new_directed(
            9,
            &[
                (0, 1, 1.),
                (1, 2, 1.),
                (2, 4, 1.),
                (4, 3, -3.),
                (3, 2, 1.),
                (1, 5, 4.),
                (1, 6, 4.),
                (5, 6, 5.),
                (6, 7, 4.),
                (5, 7, 3.),
            ],
        );

        let dists = graph.bellman_ford(0);
        assert_eq!(
            &dists,
            &[
                0.00,           // 0 -> 0
                1.00,           // 0 -> 1
                -f64::INFINITY, // 0 -> 2
                -f64::INFINITY, // 0 -> 3
                -f64::INFINITY, // 0 -> 4
                5.00,           // 0 -> 5
                5.00,           // 0 -> 6
                8.00,           // 0 -> 7
                f64::INFINITY,  // 0 -> 8
            ]
        );
    }
}
