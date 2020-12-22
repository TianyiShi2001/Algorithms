//! This mod contains an implementation of Dijkstra's shortest path algorithm from a start node to a
//! specific ending node. Dijkstra can also be modified to find the shortest path between a starting
//! node and all other nodes in the graph. However, in this implementation since we're only going
//! from a starting node to an ending node we can employ an optimization to stop early once we've
//! visited all the neighbors of the ending node.
//!
//! # Resources
//!
//! - [W. Fiset's video](https://www.youtube.com/watch?v=pSqmAO-m7Lk&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=18)

use crate::graph::{Edge, WeightedAdjacencyList};
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

impl WeightedAdjacencyList {
    pub fn dijkstra(&self, start: usize, end: usize) -> Option<(f64, Vec<usize>)> {
        let n = self.node_count();
        let mut dists = vec![f64::INFINITY; n];
        let mut prev = vec![None; n];
        let mut vis = vec![false; n];
        let mut pq = PriorityQueue::with_capacity(self.node_count());
        // `priority_queue::PriorityQueue` requires that the priority implements `Ord`,
        // but the std floats implement only `PartialOrd`
        pq.push(start, OrderedFloat::from(-0f64));
        dists[start] = 0f64;
        while let Some((node, cur_dist)) = pq.pop() {
            // Once we've visited all the nodes spanning from the end
            // node we know we can return the minimum distance value to
            // the end node because it cannot get any better after this point.
            if node == end {
                break;
            };
            // `priority_queue::PriorityQueue` is a max priority queue, but we want the min.
            // Negating the priority (dist) immediately before pushing and after popping will do.
            let cur_dist = -cur_dist.into_inner();

            vis[node] = true;

            let dist = &mut dists[node];

            // We already found a better path before we got to
            // processing this node so we can ignore it.
            if *dist < cur_dist {
                continue;
            }
            *dist = cur_dist;
            for &Edge { to, weight } in &self[node] {
                // You cannot get a shorter path by revisiting
                // a node you have already visited before.
                if !vis[to] {
                    // Relax edge by updating minimum cost if applicable.
                    let new_dist = cur_dist + weight;
                    if new_dist < dists[to] {
                        prev[to] = Some(node);
                        dists[to] = new_dist;
                        pq.push(to, (-new_dist).into());
                    }
                }
            }
        }

        if prev[end].is_none() {
            if start == end {
                Some((dists[start], vec![start]))
            } else {
                None
            }
        } else {
            // reconstruct path
            let mut path = vec![end];
            let mut i = end;
            while let Some(node) = prev[i] {
                path.push(node);
                i = node;
            }
            path.reverse();
            Some((dists[end], path))
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_dijkstra() {
        // example from https://www.youtube.com/watch?v=pSqmAO-m7Lk&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=18
        // at 16:47
        let graph = WeightedAdjacencyList::new_directed(
            6,
            &[
                (0, 1, 5.),
                (0, 2, 1.),
                (1, 2, 2.),
                (2, 1, 3.),
                (1, 3, 3.),
                (1, 4, 20.),
                (2, 4, 12.),
                (3, 2, 3.),
                (3, 4, 2.),
                (3, 5, 6.),
                (3, 4, 2.),
                (4, 5, 1.),
            ],
        );
        let (dist, path) = graph.dijkstra(0, 5).unwrap();
        assert_eq!(&path, &[0, 2, 1, 3, 4, 5]);
        assert_eq!(dist, 10.);
        assert_eq!(graph.dijkstra(1, 1).unwrap(), (0.0, vec![1]));
    }
}
