//! This module contains a naive implementation of hierarchical clustering.

use crate::algo::graph::WeightedUndirectedAdjacencyMatrixCondensed;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::cmp::{max, min};

use super::Dendrogram;

pub struct HierarchicalClusterer<'a> {
    dis: &'a WeightedUndirectedAdjacencyMatrixCondensed,
}

impl<'a> HierarchicalClusterer<'a> {
    pub fn new(dis: &'a WeightedUndirectedAdjacencyMatrixCondensed) -> Self {
        Self { dis }
    }
    pub fn complete(&self) -> Dendrogram {
        let n = self.dis.node_count();
        // Dynamically tracks the pair of clusters with the shortest distance
        let mut pq = PriorityQueue::with_capacity(n * (n - 1) / 2);
        // another `n-1` clusters will be generated
        let total_clusters_count = n + (n - 1);
        // Tracks whether cluster `i` has been merged. Each cluster is merged exactly once during
        // clustering. (This is very obvious if you consider the shape of a dendrogram—the output
        // of hierarchical clustering)
        let mut merged = vec![false; total_clusters_count];
        // We need to extend the adjacency matrix from size `n` to `2n - 1` to track distances
        // from each newly formed cluster to the other clusters.
        let mut distances = self.dis.resized(total_clusters_count);
        for (i, j, dist) in self.dis.edges() {
            pq.push((i, j), -OrderedFloat(dist));
        }
        let mut steps = Vec::new();
        let mut k = n;
        while let Some(((i, j), dist)) = pq.pop() {
            let dist = -dist.into_inner();
            merged[i] = true;
            merged[j] = true;
            steps.push((i, j, dist));
            // only need to calculate distances to clusters which has not yet been merged
            // because those that have been merged will not be merged again
            for idx in (0..k).filter(|idx| !merged[*idx]) {
                // The distance from cluster `idx` to the new cluster `k` formed from clusters `i` and `j`
                // is the larger of the distances from `idx` to `i` and from `idx` to `j`
                let dist_to_k = ::partial_min_max::max(distances[(idx, i)], distances[(idx, j)]);
                distances[(idx, k)] = dist_to_k;

                pq.push((idx, k), -OrderedFloat(dist_to_k));
                // distances to `i` and `j` becomes invalid and are removed from the PQ.
                pq.remove(&(min(idx, i), max(idx, i)));
                pq.remove(&(min(idx, j), max(idx, j)));
            }

            k += 1;
            if k == total_clusters_count {
                break;
            }
        }
        Dendrogram { steps }
    }
}
