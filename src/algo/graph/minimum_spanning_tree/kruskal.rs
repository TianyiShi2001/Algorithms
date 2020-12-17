//! Kruskal's algorithm for finding an miminum spanning forest.
//!
//! # Algorithm Overview
//!
//! - create a forest F (a set of trees), where each vertex in the graph is a separate tree
//! - create a set S containing all the edges in the graph
//! - while S is nonempty and F is not yet spanning:
//!   - remove an edge with minimum weight from S
//!   - if the removed edge connects two different trees then add it to the forest F, combining
//!     two trees into a single tree
//! - At the termination of the algorithm, the forest forms a minimum spanning forest of the graph.
//!   If the graph is connected, the forest has a single component and forms a minimum spanning tree.

use crate::algo::graph::WeightedAdjacencyList;
use crate::data_structures::disjoint_set::UnionFind;
use ordered_float::OrderedFloat;

impl WeightedAdjacencyList {
    pub fn kruskal(&self) -> Option<(f64, WeightedAdjacencyList)> {
        let n = self.vertices_count();
        let mut edges = self.edges().collect::<Vec<_>>();
        edges.sort_by_key(|(_f, _t, cost)| OrderedFloat(*cost));
        let mut msf_edges = Vec::new();
        let mut mst_cost = 0.;
        let mut ds = UnionFind::with_size(n);
        for (from, to, cost) in edges {
            // if not connected i.e. adding this edge will not produce a cycle
            if !ds.in_same_set(from, to) {
                msf_edges.push((from, to, cost));
                mst_cost += cost;
                ds.union(from, to);
            }
        }
        if msf_edges.len() == n - 1 {
            return Some((mst_cost, WeightedAdjacencyList::new_directed(n, &msf_edges)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kruskal1() {
        // from https://www.youtube.com/watch?v=jsmMtJpPnhU&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=30
        // at 10:05
        let g = WeightedAdjacencyList::new_directed(
            8,
            &[
                (0, 1, 10.),
                (0, 2, 1.),
                (0, 3, 4.),
                (2, 1, 3.),
                (2, 5, 8.),
                (2, 3, 2.),
                (2, 0, 1.),
                (3, 2, 2.),
                (3, 5, 2.),
                (3, 6, 7.),
                (3, 0, 4.),
                (5, 2, 8.),
                (5, 4, 1.),
                (5, 7, 9.),
                (5, 6, 6.),
                (5, 3, 2.),
                (4, 1, 0.),
                (4, 5, 1.),
                (4, 7, 8.),
                (1, 0, 10.),
                (1, 2, 3.),
                (1, 4, 0.),
                (6, 3, 7.),
                (6, 5, 6.),
                (6, 7, 12.),
                (7, 4, 8.),
                (7, 5, 9.),
                (7, 6, 12.),
            ],
        );
        let (cost, mst) = g.kruskal().unwrap();
        println!("{}", mst);
        assert_eq!(cost, 20.);
    }
    #[test]
    fn test_kruskal2() {
        // from https://www.youtube.com/watch?v=xq3ABa-px_g&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=31
        // at 08:31
        let g = WeightedAdjacencyList::new_directed(
            7,
            &[
                (0, 2, 0.),
                (0, 5, 7.),
                (0, 3, 5.),
                (0, 1, 9.),
                (2, 0, 0.),
                (2, 5, 6.),
                (3, 0, 5.),
                (3, 1, -2.),
                (3, 6, 3.),
                (3, 5, 2.),
                (1, 0, 9.),
                (1, 3, -2.),
                (1, 6, 4.),
                (1, 4, 3.),
                (5, 2, 6.),
                (5, 0, 7.),
                (5, 3, 2.),
                (5, 6, 1.),
                (6, 5, 1.),
                (6, 3, 3.),
                (6, 1, 4.),
                (6, 4, 6.),
                (4, 1, 3.),
                (4, 6, 6.),
            ],
        );
        let (cost, mst) = g.kruskal().unwrap();
        println!("{}", mst);
        assert_eq!(cost, 9.);
    }
}
