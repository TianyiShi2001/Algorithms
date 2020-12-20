//! # Resources
//!
//! - [Victor Lavrenko's lecture series](https://www.youtube.com/watch?v=GVz6Y8r5AkY&list=PLBv09BD7ez_7qIbBhyQDr-LAKWUeycZtx&index=1)
//! - [Understanding the Concept of Hierarchical Clustering](https://towardsdatascience.com/understanding-the-concept-of-hierarchical-clustering-technique-c6e8243758ec)

pub mod generic;
pub mod improved;
pub mod naive;
pub mod specialized;

// Adapted from the `::kodama::Method`
/// A method for computing the dissimilarities between clusters.
///
/// The method selected dictates how the dissimilarities are computed whenever
/// a new cluster is formed. In particular, when clusters `a` and `b` are
/// merged into a new cluster `ab`, then the pairwise dissimilarity between
/// `ab` and every other cluster is computed using one of the methods variants
/// in this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    /// Assigns the minimum dissimilarity between all pairs of observations.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \min (D_{i,k}, D_{j, k} )
    /// $$
    ///
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Single,
    /// Assigns the maximum dissimilarity between all pairs of observations.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \max (D_{i,k}, D_{j, k} )
    /// $$
    ///
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Complete,
    /// Assigns the average dissimilarity between all pairs of observations.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \dfrac{D_{i,k} \cdot |i| + D_{j, k} \cdot |j|}{|k|}
    /// $$
    ///
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Average,
    /// Assigns the weighted dissimilarity between clusters.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = 0.5 (D_{i, x} + D_{j, x})
    /// $$
    ///
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Weighted,
    /// Assigns the Ward dissimilarity between clusters.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \sqrt{\dfrac{D_{i, x}^2 \cdot (|i| + |x|) + D_{j, x}^2 \cdot (|j| + |x|) + D_{i, j} \cdot |x|}{|i| + |j| + |x|}}
    /// $$
    ///
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Ward,
    /// Assigns the centroid dissimilarity between clusters.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \sqrt{\dfrac{|i|\cdot D_{i, x}^2 + |j| \cdot D_{j, x}^2}{|k|} - \dfrac{|i|\cdot |j| \cdot D_{i,j}^2}{|k|^2}}
    /// $$
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Centroid,
    /// Assigns the median dissimilarity between clusters.
    ///
    /// Specifically, if $k$ is a newly merged cluster and $x$ is every other
    /// cluster, then the pairwise dissimilarity between `k` and `x` is
    /// computed by
    ///
    /// $$
    /// D_{k, x} = \sqrt{\dfrac{D_{i, x}^2 + D_{j, x}}{2}^2 - \dfrac{D_{i,j}}{4} }
    /// $$
    ///
    /// where $i$ and $j$ correspond to the clusters that merged to create $k$
    Median,
}

#[derive(Debug)]
pub struct Dendrogram {
    steps: Vec<(usize, usize, f64)>,
}

impl PartialEq for Dendrogram {
    fn eq(&self, other: &Dendrogram) -> bool {
        if self.steps.len() != other.steps.len() {
            return false;
        }
        for (s0, s1) in self.steps.iter().zip(other.steps.iter()) {
            if s0.0 != s1.0 || s0.1 != s1.1 || (s0.2 - s1.2).abs() > 0.0001 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algo::geometry::geographical_coordinate::GeographicalCoordinate;
    use crate::algo::graph::WeightedUndirectedAdjacencyMatrixCondensed;
    use lazy_static::lazy_static;
    use rand::{thread_rng, Rng};
    lazy_static! {
        // From kodama's example
        static ref KODAMA_EXAMPLE: Vec<f64> = {
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
            condensed
        };
        // random
        static ref RANDOM: Vec<f64> =  {
            const N: usize = 10;
            let mut rng = thread_rng();
            let mut condensed = Vec::new();
            condensed.extend((0..(N - 1) * N / 2).map(|_|rng.gen_range(1.0, 10.0)));
            condensed
        };
    }
    fn _generate_expected(n: usize, v: &Vec<f64>, method: ::kodama::Method) -> Dendrogram {
        let mut condensed_ = v.clone();
        let steps = kodama::linkage(&mut condensed_, n, method)
            .steps()
            .into_iter()
            .map(|x| (x.cluster1, x.cluster2, x.dissimilarity))
            .collect::<Vec<_>>();
        Dendrogram { steps }
    }
    #[test]
    fn test_single() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Single);
        let m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let cl = specialized::single::HierarchicalClusterer::new(&m);
        assert_eq!(cl.single(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Single), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Single);
        let m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let cl = specialized::single::HierarchicalClusterer::new(&m);
        assert_eq!(cl.single(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Single), expected);
    }

    #[test]
    fn test_complete() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Complete);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let cl = naive::HierarchicalClusterer::new(&m);
        assert_eq!(cl.complete(), expected);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.complete(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Complete), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Complete);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let cl = naive::HierarchicalClusterer::new(&m);
        assert_eq!(cl.complete(), expected);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.complete(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Complete), expected);
    }

    #[test]
    fn test_average() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Average);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.average(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Average), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Average);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.average(), expected);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Average), expected);
    }

    #[test]
    fn test_median() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Median);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Median), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Median);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Median), expected);
    }

    #[test]
    fn test_centroid() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Centroid);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Centroid), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Centroid);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Centroid), expected);
    }

    #[test]
    fn test_ward() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Ward);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Ward), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Ward);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Ward), expected);
    }

    #[test]
    fn test_weighted() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Weighted);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Weighted), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Weighted);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = generic::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.linkage(Method::Weighted), expected);
    }
}
