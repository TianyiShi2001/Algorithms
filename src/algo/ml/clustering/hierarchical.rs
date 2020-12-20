//! # Resources
//!
//! - [Victor Lavrenko's lecture series](https://www.youtube.com/watch?v=GVz6Y8r5AkY&list=PLBv09BD7ez_7qIbBhyQDr-LAKWUeycZtx&index=1)
//! - [Understanding the Concept of Hierarchical Clustering](https://towardsdatascience.com/understanding-the-concept-of-hierarchical-clustering-technique-c6e8243758ec)

pub mod improved;
pub mod naive;

// Copied from the `kodama` crate under MIT license
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
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// min(d[ab, x] for ab in AB for x in X)
    /// ```
    ///
    /// where `ab` and `x` correspond to all observations in `AB` and `X`,
    /// respectively.
    Single,
    /// Assigns the maximum dissimilarity between all pairs of observations.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// max(d[ab, x] for ab in AB for x in X)
    /// ```
    ///
    /// where `ab` and `x` correspond to all observations in `AB` and `X`,
    /// respectively.
    Complete,
    /// Assigns the average dissimilarity between all pairs of observations.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// sum(d[ab, x] for ab in AB for x in X) / (|AB| * |X|)
    /// ```
    ///
    /// where `ab` and `x` correspond to all observations in `AB` and `X`,
    /// respectively, and `|AB|` and `|X|` correspond to the total number of
    /// observations in `AB` and `X`, respectively.
    Average,
    /// Assigns the weighted dissimilarity between clusters.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// 0.5 * (d(A, X) + d(B, X))
    /// ```
    ///
    /// where `A` and `B` correspond to the clusters that merged to create
    /// `AB`.
    Weighted,
    /// Assigns the Ward dissimilarity between clusters.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// let t1 = d(A, X)^2 * (|A| + |X|);
    /// let t2 = d(B, X)^2 * (|B| + |X|);
    /// let t3 = d(A, B)^2 * |X|;
    /// let T = |A| + |B| + |X|;
    /// sqrt(t1/T + t2/T + t3/T)
    /// ```
    ///
    /// where `A` and `B` correspond to the clusters that merged to create
    /// `AB`.
    Ward,
    /// Assigns the centroid dissimilarity between clusters.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// let t1 = |A| * d(A, X) + |B| * d(B, X));
    /// let t2 = |A| * |B| * d(A, B);
    /// let size = |A| + |B|;
    /// sqrt(t1/size + t2/size^2)
    /// ```
    ///
    /// where `A` and `B` correspond to the clusters that merged to create
    /// `AB`.
    Centroid,
    /// Assigns the median dissimilarity between clusters.
    ///
    /// Specifically, if `AB` is a newly merged cluster and `X` is every other
    /// cluster, then the pairwise dissimilarity between `AB` and `X` is
    /// computed by
    ///
    /// ```text
    /// sqrt(d(A, X)/2 + d(B, X)/2 - d(A, B)/4)
    /// ```
    ///
    /// where `A` and `B` correspond to the clusters that merged to create
    /// `AB`.
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
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let cl = naive::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.single(), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Single);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let cl = naive::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.single(), expected);
    }

    #[test]
    fn test_complete() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Complete);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let cl = naive::HierarchicalClusterer::new(&m);
        assert_eq!(cl.complete(), expected);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.complete(), expected);

        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Complete);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let cl = naive::HierarchicalClusterer::new(&m);
        assert_eq!(cl.complete(), expected);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.complete(), expected);
    }

    #[test]
    fn test_average() {
        let expected = _generate_expected(6, &KODAMA_EXAMPLE, ::kodama::Method::Average);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&KODAMA_EXAMPLE);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.average(), expected);
        let expected = _generate_expected(10, &RANDOM, ::kodama::Method::Average);
        let mut m = WeightedUndirectedAdjacencyMatrixCondensed::from_slice(&RANDOM);
        let mut cl = improved::HierarchicalClusterer::new(&mut m);
        assert_eq!(cl.average(), expected);
    }
}
