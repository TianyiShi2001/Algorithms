//! This file contains an implementation of the Floyd-Warshall algorithm to find all pairs of
//! shortest paths between nodes in a graph. We also demonstrate how to detect negative cycles and
//! reconstruct the shortest path.
//! Time Complexity: $O(V^3)$

use crate::algo::graph::WeightedAdjacencyMatrix;

#[derive(Debug, Eq, PartialEq)]
pub enum ShortestPathError {
    NegativeCycle,
    Unreachable,
}

pub struct FloydWarshall {
    dp: Vec<Vec<f32>>,
    next: Vec<Vec<Option<usize>>>,
}

impl FloydWarshall {
    pub fn new(graph: &WeightedAdjacencyMatrix) -> Self {
        let n = graph.vertices_count();
        // Copy input matrix and setup 'next' matrix for path reconstruction.
        let mut dp = graph.inner.clone();
        let mut next = vec![vec![None; n]; n];
        for i in 0..n {
            for j in 0..n {
                if graph[i][j] != f32::INFINITY {
                    next[i][j] = Some(j);
                }
            }
        }

        // Compute all pairs shortest paths.
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dp[i][k] + dp[k][j] < dp[i][j] {
                        dp[i][j] = dp[i][k] + dp[k][j];
                        next[i][j] = next[i][k];
                    }
                }
            }
        }

        // Identify negative cycles by propagating the value 'f32::NEG_INFINITY'
        // to every edge that is part of or reaches into a negative cycle.
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dp[i][k] + dp[k][j] < dp[i][j] {
                        dp[i][j] = f32::NEG_INFINITY;
                        next[i][j] = None;
                    }
                }
            }
        }

        Self { dp, next }
    }

    pub fn distance(&self, start: usize, end: usize) -> f32 {
        self.dp[start][end]
    }

    /// Reconstructs the shortest path (of nodes) from `start` to `end` inclusive.
    pub fn path(&self, start: usize, end: usize) -> Result<Vec<usize>, ShortestPathError> {
        let mut path = Vec::new();
        if self.dp[start][end] == f32::INFINITY {
            return Err(ShortestPathError::Unreachable);
        };
        let mut prev = start;
        while let Some(at) = self.next[prev][end] {
            path.push(prev);
            if at == end {
                if at != prev {
                    path.push(at);
                }
                return Ok(path);
            }
            prev = at;
        }
        Err(ShortestPathError::NegativeCycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::graph::WeightedAdjacencyList;
    #[test]
    fn test_floyd_warshall() {
        let graph: WeightedAdjacencyMatrix = WeightedAdjacencyList::new_directed(
            7,
            &[
                (0, 1, 2.),
                (0, 2, 5.),
                (0, 6, 10.),
                (1, 2, 2.),
                (1, 4, 11.),
                (2, 6, 2.),
                (6, 5, 11.),
                (4, 5, 1.),
                (5, 4, -2.),
            ],
        )
        .into();
        let result = FloydWarshall::new(&graph);

        assert_eq!(result.distance(0, 0), 0.0);
        assert_eq!(result.distance(0, 1), 2.000);
        assert_eq!(result.distance(0, 2), 4.000);
        assert_eq!(result.distance(0, 3), f32::INFINITY);
        assert_eq!(result.distance(0, 4), f32::NEG_INFINITY);
        assert_eq!(result.distance(0, 5), f32::NEG_INFINITY);
        assert_eq!(result.distance(0, 6), 6.000);
        assert_eq!(result.distance(1, 0), f32::INFINITY);
        assert_eq!(result.distance(1, 1), 0.000);
        assert_eq!(result.distance(1, 2), 2.000);
        assert_eq!(result.distance(1, 3), f32::INFINITY);

        assert_eq!(result.path(0, 0), Ok(vec![0]));
        assert_eq!(result.path(0, 1), Ok(vec![0, 1]));
        assert_eq!(result.path(0, 2), Ok(vec![0, 1, 2]));
        assert_eq!(result.path(0, 3), Err(ShortestPathError::Unreachable));
        assert_eq!(result.path(0, 4), Err(ShortestPathError::NegativeCycle));
        assert_eq!(result.path(0, 5), Err(ShortestPathError::NegativeCycle));
        assert_eq!(result.path(0, 6), Ok(vec![0, 1, 2, 6]));
        assert_eq!(result.path(1, 0), Err(ShortestPathError::Unreachable));
        assert_eq!(result.path(1, 1), Ok(vec![1]));
        assert_eq!(result.path(1, 2), Ok(vec![1, 2]));
        assert_eq!(result.path(1, 3), Err(ShortestPathError::Unreachable));
        assert_eq!(result.path(1, 4), Err(ShortestPathError::NegativeCycle));
        assert_eq!(result.path(1, 5), Err(ShortestPathError::NegativeCycle));
        assert_eq!(result.path(1, 6), Ok(vec![1, 2, 6]));
        assert_eq!(result.path(2, 0), Err(ShortestPathError::Unreachable));
    }
}
