use crate::algo::graph::WeightedUndirectedAdjacencyMatrixCondensed;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::cmp::{max, min};

use super::Dendrogram;

pub struct HierarchicalClusterer<'a> {
    dis: &'a mut WeightedUndirectedAdjacencyMatrixCondensed,
}

impl<'a> HierarchicalClusterer<'a> {
    pub fn new(dis: &'a mut WeightedUndirectedAdjacencyMatrixCondensed) -> Self {
        Self { dis }
    }
    /// Note that in the naïve implementation, when merging two clusters, we remove distances associated with
    /// the two old clusters which become invalid, and distances from the newly formed node are added to a new
    /// row in the adjacency matrix. What we could do instead is to *update* distances associated with one of the
    /// two old clusters with the distances associated with the newly formed cluster.
    pub fn complete(&mut self) -> Dendrogram {
        let n = self.dis.node_count();
        // Dynamically tracks the pair of clusters with the shortest distance
        let mut pq = PriorityQueue::with_capacity(n * (n - 1) / 2);
        // another `n-1` clusters will be generated
        let total_clusters_count = n + (n - 1);
        // Tracks whether cluster `i` has been merged.
        let mut merged = vec![false; total_clusters_count];
        // when merging clusters `i` and `j` where `i < j`, cluster `i` becomes invalid and cluster `j` now
        // represent the newly formed cluster `k`. `actual_idx` is a dynamic mapping from the "apparent" clusters
        // `0..n` to the actual clusters `0..2n-1`
        let mut actual_idx: Vec<_> = (0..n).collect();
        for (i, j, dist) in self.dis.edges() {
            pq.push((i, j), -OrderedFloat(dist));
        }
        let mut steps = Vec::new();
        let mut k = n;
        while let Some(((i, j), dist)) = pq.pop() {
            let dist = -dist.into_inner();
            // cluster `i` is invalidated.
            // Note that `j` is still valid, but now it's representing the newly formed cluster
            merged[i] = true;
            let (_i, _j) = (actual_idx[i], actual_idx[j]);
            steps.push((min(_i, _j), max(_i, _j), dist));
            actual_idx[j] = k;
            for idx in (0..n).filter(|idx| !merged[*idx] && *idx != j) {
                let di = self.dis[(idx, i)];
                let dj = &mut self.dis[(idx, j)];
                let dk = ::partial_min_max::max(di, *dj);
                *dj = dk;
                // update the priority queue
                // the new distance is no shorter than the previous one
                pq.push_decrease((min(idx, j), max(idx, j)), -OrderedFloat(dk));
                // distances to `i` becomes invalid and are removed from the PQ.
                pq.remove(&(min(idx, i), max(idx, i)));
            }

            k += 1;
            if k == total_clusters_count {
                break;
            }
        }
        Dendrogram { steps }
    }

    pub fn average(&mut self) -> Dendrogram {
        let n = self.dis.node_count();
        // Dynamically tracks the pair of clusters with the shortest distance
        let mut pq = PriorityQueue::with_capacity(n * (n - 1) / 2);
        // another `n-1` clusters will be generated
        let total_clusters_count = n + (n - 1);
        // Tracks whether cluster `i` has been merged.
        let mut merged = vec![false; total_clusters_count];
        // A dynamic mapping from the "apparent" clusters `0..n` to the actual clusters `0..2n-1`
        let mut actual_idx: Vec<_> = (0..n).collect();
        // number of nodes contained in each cluster
        let mut number_of_nodes = vec![1; n];
        for (i, j, dist) in self.dis.edges() {
            pq.push((i, j), -OrderedFloat(dist));
        }
        let mut steps = Vec::new();
        let mut k = n;
        // println!("{}", self.dis);
        while let Some(((i, j), dist)) = pq.pop() {
            let dist = -dist.into_inner();
            merged[i] = true;
            let (_i, _j) = (actual_idx[i], actual_idx[j]);
            steps.push((min(_i, _j), max(_i, _j), dist));
            actual_idx[j] = k;
            let (ni, nj) = (number_of_nodes[i], number_of_nodes[j]);
            let nk = ni + nj;
            for idx in (0..n).filter(|idx| !merged[*idx] && *idx != j) {
                let di = self.dis[(idx, i)];
                let dj = &mut self.dis[(idx, j)];

                // calculate the average
                let dk = (di * ni as f64 + *dj * nj as f64) / nk as f64;
                number_of_nodes[j] = nk;
                *dj = dk;
                // update the priority queue
                // the new distance can either be longer or shorter
                pq.push((min(idx, j), max(idx, j)), -OrderedFloat(dk));
                // distances to `i` becomes invalid and are removed from the PQ.
                pq.remove(&(min(idx, i), max(idx, i)));
                self.dis[(i, idx)] = f64::INFINITY;
            }
            self.dis[(i, j)] = f64::INFINITY;
            // println!("{}", self.dis);

            k += 1;
            if k == total_clusters_count {
                break;
            }
        }
        Dendrogram { steps }
    }
}
