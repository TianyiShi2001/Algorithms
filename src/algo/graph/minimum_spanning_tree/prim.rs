//! An implementation of the eager version of Prim's algorithm which relies on using an indexed
//! priority queue data structure to query the next best edge.
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=jsmMtJpPnhU&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=30)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=xq3ABa-px_g&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=31)
//! - [W. Fiset's video 3](https://www.youtube.com/watch?v=CI5Fvk-dGVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=32)
//! - [Wikipedia](https://www.wikiwand.com/en/Prim%27s_algorithm)

use crate::algo::graph::{Edge, WeightedAdjacencyList};
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

impl WeightedAdjacencyList {
    pub fn prim(&self) -> Option<(f64, WeightedAdjacencyList)> {
        let n = self.node_count();
        // the number of edges in the MST (a tree with `n` vertices has `n - 1` edges)
        let m = n - 1;

        let mut visited = vec![false; n];
        let mut pq = PriorityQueue::new();

        let add_edges = |from, visited: &mut [bool], pq: &mut PriorityQueue<_, _>| {
            visited[from] = true;
            // iterate over all edges going outwards from the current node.
            // Add edges to the PQ which point to unvisited nodes.
            for &Edge { to, cost } in &self[from] {
                if !visited[to] {
                    // `push_increase` queues an element if it's not already present.
                    // Otherwise, it updates the element's priority if the new priority is higher.
                    pq.push_increase((from, to), OrderedFloat(-cost));
                }
            }
        };

        let mut min_mst_cost = f64::INFINITY;
        let mut best_mst_edges = Vec::new();
        for i in 0..n {
            let mut mst_cost = 0.;
            let mut mst_edges = Vec::with_capacity(m);
            add_edges(i, &mut visited, &mut pq);

            while let Some(((from, to), cost)) = pq.pop() {
                if mst_edges.len() == m {
                    break;
                };
                if visited[to] {
                    continue;
                }
                mst_edges.push((from, to, -cost.into_inner()));
                mst_cost += -cost.into_inner();

                add_edges(to, &mut visited, &mut pq);
            }
            if mst_edges.len() != m {
                continue;
            }
            if mst_cost < min_mst_cost {
                min_mst_cost = mst_cost;
                best_mst_edges = mst_edges
            }
        }
        if min_mst_cost == f64::INFINITY {
            None
        } else {
            Some((
                min_mst_cost,
                WeightedAdjacencyList::new_directed(n, &best_mst_edges),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prim1() {
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
        let (cost, mst) = g.prim().unwrap();
        println!("{}", mst);
        assert_eq!(cost, 20.);
    }
    #[test]
    fn test_prim2() {
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
        let (cost, mst) = g.prim().unwrap();
        println!("{}", mst);
        assert_eq!(cost, 9.);
    }
}
