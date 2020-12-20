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
        let mut steps = Vec::new();
        let mut k = n;
        // A union-find used to track to which most recently formed cluster each node belongs to.
        let mut uf = UnionFind::with_size(total_clusters_count);
        // process edges in ascending order of distance (short edges first)
        for (i, j, dist) in edges {
            // find the representative of nodes `i` and `j`, i.e. the most recently formed
            // clusters that contains nodes `i` and `j`, respectively
            let (_i, _j) = (uf.find(i), uf.find(j));
            // skip if `_i == _j`, which means `i` and `j` are already in one cluster
            if _i != _j {
                // `k` becomes the parent of the most recently formed clusters that contain
                // `i` and `j`
                // Note that here we do not use the `union` method, which makes the larger cluster be the
                // parent of the smaller cluster. Here we want exactly that the new cluster, `k`, to become
                // the parent
                uf.set_parent(_i, k);
                uf.set_parent(_j, k);
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
