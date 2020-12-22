use crate::graph::WeightedUndirectedAdjacencyMatrixCondensed;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::cmp::{max, min};

use super::{Dendrogram, Method};

pub struct HierarchicalClusterer<'a> {
    dis: &'a mut WeightedUndirectedAdjacencyMatrixCondensed,
}

impl<'a> HierarchicalClusterer<'a> {
    pub fn new(dis: &'a mut WeightedUndirectedAdjacencyMatrixCondensed) -> Self {
        Self { dis }
    }
    pub fn linkage(&mut self, method: Method) -> Dendrogram {
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
                let dij = self.dis[(i, j)];
                let dik = self.dis[(idx, i)];
                let djk = &mut self.dis[(idx, j)];
                let nx = number_of_nodes[idx];

                *djk = match method {
                    Method::Single => single(dik, *djk),
                    Method::Complete => complete(dik, *djk),
                    Method::Median => median(dik, *djk, dij),
                    Method::Average => average(dik, *djk, ni, nj, nk),
                    Method::Centroid => centroid(dik, *djk, dij, ni, nj, nk),
                    Method::Ward => ward(dik, *djk, dij, ni, nj, nk, nx),
                    Method::Weighted => weighted(dik, *djk),
                };
                number_of_nodes[j] = nk;
                // update the priority queue
                pq.push((min(idx, j), max(idx, j)), -OrderedFloat(*djk));
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

fn single(a: f64, b: f64) -> f64 {
    ::partial_min_max::min(a, b)
}

fn complete(a: f64, b: f64) -> f64 {
    ::partial_min_max::max(a, b)
}

fn average(a: f64, b: f64, size_a: usize, size_b: usize, size_ab: usize) -> f64 {
    (a * size_a as f64 + b * size_b as f64) / size_ab as f64
}

fn median(a: f64, b: f64, c: f64) -> f64 {
    (0.5 * (a * a + b * b) - 0.25 * c * c).sqrt()
}

pub fn centroid(a: f64, b: f64, c: f64, size_a: usize, size_b: usize, size_ab: usize) -> f64 {
    let (size_a, size_b, size_ab) = (size_a as f64, size_b as f64, size_ab as f64);
    let (a_2, b_2, c_2) = (a * a, b * b, c * c);

    ((((size_a * a_2) + (size_b * b_2)) / size_ab)
        - ((size_a * size_b * c_2) / (size_ab * size_ab)))
        .sqrt()
}
pub fn ward(
    a: f64,
    b: f64,
    c: f64,
    size_a: usize,
    size_b: usize,
    size_ab: usize,
    size_x: usize,
) -> f64 {
    let (size_a, size_b, size_ab, size_x) =
        (size_a as f64, size_b as f64, size_ab as f64, size_x as f64);
    let (a_2, b_2, c_2) = (a * a, b * b, c * c);
    let numerator = (size_x + size_a) * a_2 + (size_x + size_b) * b_2 - size_x * c_2;
    let denominator = size_ab + size_x;
    (numerator / denominator).sqrt()
}

pub fn weighted(a: f64, b: f64) -> f64 {
    0.5 * (a + b)
}
