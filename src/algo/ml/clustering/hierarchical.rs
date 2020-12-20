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
