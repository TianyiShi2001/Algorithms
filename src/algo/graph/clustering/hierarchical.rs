//! # Resources
//!
//! - [**Video**: Agglomerative Clustering: how it works](https://www.youtube.com/watch?v=XJ3194AmH40)
//! - [Understanding the Concept of Hierarchical Clustering](https://towardsdatascience.com/understanding-the-concept-of-hierarchical-clustering-technique-c6e8243758ec)

use crate::algo::graph::WeightedUndirectedAdjacencyMatrixCondensed;
use crate::data_structures::union_find::UnionFind;
use ordered_float::OrderedFloat;

impl WeightedUndirectedAdjacencyMatrixCondensed {
    pub fn hierarchical_cluster(&self) -> Vec<(usize, usize, f64)> {
        //-> Option<(f64, WeightedAdjacencyList)> {
        let n = self.node_count();
        let total_clusters_count = n + (n - 1); // another `n-1` clusters will be generated
        let mut edges = self.edges().collect::<Vec<_>>();
        edges.sort_by_key(|(_f, _t, dist)| OrderedFloat(*dist));
        let mut steps = Vec::new();
        let mut k = n;
        let mut ds = UnionFind::with_ranks([vec![0; n], (1..n).collect()].concat());
        for (i, j, dist) in edges {
            // if not connected i.e. adding this edge will not produce a cycle
            let (_i, _j) = (ds.find(i), ds.find(j));
            if _i != _j {
                ds.union(_i, k);
                ds.union(_j, k);
                steps.push((_i, _j, dist));

                k += 1;
                if k == total_clusters_count {
                    break;
                }
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::geometry::geographical_coordinate::GeographicalCoordinate;
    #[test]
    fn test_hierarchical_clustering() {
        let coordinates = vec![
            GeographicalCoordinate::new(42.5833333, -71.8027778),
            GeographicalCoordinate::new(42.2791667, -71.4166667),
            GeographicalCoordinate::new(42.3458333, -71.5527778),
            GeographicalCoordinate::new(42.1513889, -71.6500000),
            GeographicalCoordinate::new(42.3055556, -71.5250000),
            GeographicalCoordinate::new(42.2694444, -71.6166667),
        ];
        let mut condensed = vec![];
        for i in 0..coordinates.len() - 1 {
            for j in i + 1..coordinates.len() {
                condensed.push(coordinates[i].distance(coordinates[j]));
            }
        }
        let mut condensed_ = condensed.clone();
        let expected = kodama::linkage(&mut condensed_, coordinates.len(), kodama::Method::Single)
            .steps()
            .into_iter()
            .map(|x| (x.cluster1, x.cluster2, x.dissimilarity))
            .collect::<Vec<_>>();

        let m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&condensed);
        assert_eq!(m.hierarchical_cluster(), expected);
    }
}
