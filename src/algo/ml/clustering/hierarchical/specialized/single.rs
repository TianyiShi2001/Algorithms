use super::super::Dendrogram;
use crate::algo::graph::WeightedUndirectedAdjacencyMatrixCondensed;
use crate::data_structures::union_find::UnionFind;
use ordered_float::OrderedFloat;
use std::cmp::{max, min};

pub struct HierarchicalClusterer<'a> {
    dis: &'a WeightedUndirectedAdjacencyMatrixCondensed,
}

impl<'a> HierarchicalClusterer<'a> {
    pub fn new(dis: &'a WeightedUndirectedAdjacencyMatrixCondensed) -> Self {
        Self { dis }
    }
    pub fn single(&self) -> Dendrogram {
        let n = self.dis.node_count();
        // another `n-1` clusters will be generated
        let total_clusters_count = n + (n - 1);
        let mut edges = self.dis.edges().collect::<Vec<_>>();
        // sort edges in ascending order of the distance
        edges.sort_by_key(|(_f, _t, dist)| OrderedFloat(*dist));
        // the sequence of instructions to produce the dendrogram
        // each item is a tuple of `(i, j, dist)`
        let mut steps = Vec::new();
        // the index of the next cluster. Starts from `n` because clusters `0` to `n-1`
        // are 'singleton' clusters formed by individual nodes.
        let mut k = n;
        // A union-find used to track which most recently formed cluster a node belongs to
        // clusters `0` to `n-1` have a rank of `0`, and clusters `n` to `2n - 1`, have ranks
        // `1` to `n-1`, which is in accordance with their order of formation in the next iteration.
        // This ensures that the most recently formed clusters becomes the parents. (when mergeing,
        // UF chooses the element with a higher rank to be the parent.)
        let mut uf = UnionFind::with_ranks([vec![0; n], (1..n).collect()].concat());
        // process edges in ascending order of distance (short edges first)
        for (i, j, dist) in edges {
            // find the representative of nodes `i` and `j`, i.e. the most recently formed
            // clusters that contains nodes `i` and `j`, respectively
            let (_i, _j) = (uf.find(i), uf.find(j));
            // skip if `_i == _j`, which means `i` and `j` are already in one cluster
            if _i != _j {
                // `k` becomes the parent of the most recently formed clusters that contain
                // `i` and `j`
                uf.union(_i, k);
                uf.union(_j, k);
                steps.push((min(_i, _j), max(_i, _j), dist));

                k += 1;
                if k == total_clusters_count {
                    break;
                }
            }
        }
        Dendrogram { steps }
    }
}
