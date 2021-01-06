// //! # Overview
// //!
// //! - Input: number of clusters, $k$, and a set of points $x_1 \ldots x_n$
// //! - Place centroids $c_i \ldots$ at random locations
// //! - Repeat until convergence:
// //!     - for each point $x_i$ in $x_1 \ldots x_n$:
// //!         - find nearest centroid $c_j$
// //!         - assign the point $x_i$ to cluster $c_j$
// //!     - for each cluster $c_j$ in $c_1 \ldots c_k$:
// //!         - new position of $c_j$ = mean of all points assigned to cluster $c_j$
// //!     - minimizes aggregate intra-cluster distance $\sum_{j}\sum{x_i\rightarrowc_j}D(c_j x_i)^2$
// //!         - total squared distance from point to center of its cluster
// //!         - same as variance if Euclidian distance is used.
// //!
// //! # Resources
// //!
// //! [Victor Lavrenko's lecture series](https://www.youtube.com/watch?v=mHl5P-qlnCQ&list=PLBv09BD7ez_6cgkSUAqBXENXEhCkb_2wl)

// // WIP

// // use crate::algo::geometry::Point2D;

// use rand::{thread_rng, Rng};

// #[derive(Copy, Clone, Debug)]
// pub struct Point2D {
//     pub x: f64,
//     pub y: f64,
// }

// pub trait Observation: Clone {
//     fn squared_distance(&self, other: &Self) -> f64;
//     fn random<R: Rng>(rng: &mut R) -> Self;
//     fn mean(observations: &[&Self]) -> Self;
//     fn zero() -> Self;
// }

// impl Observation for Point2D {
//     /// Squared Euledian distance
//     fn squared_distance(&self, other: &Self) -> f64 {
//         let (dx, dy) = (self.x - other.x, self.y - other.y);
//         dx * dx + dy * dy
//     }
//     fn random<R: Rng>(rng: &mut R) -> Self {
//         Self {
//             x: rng.gen(),
//             y: rng.gen(),
//         }
//     }
//     fn mean(observations: &[&Self]) -> Self {
//         let len = observations.len() as f64;
//         let (sum_x, sum_y) = observations.iter().fold((0., 0.), |(sum_x, sum_y), point| {
//             (sum_x + point.x, sum_y + point.y)
//         });
//         Self {
//             x: sum_x / len,
//             y: sum_y / len,
//         }
//     }
//     fn zero() -> Self {
//         Self { x: 0., y: 0. }
//     }
// }

// pub struct Kmeans<'a, T: Observation> {
//     executor: KmeansExecutor<'a, T>,
//     config: KmeansConfig,
// }

// pub struct KmeansConfig {
//     max_iteration: usize,
//     auto_k: bool,
// }

// pub struct KmeansExecutor<'a, T: Observation> {
//     observations: &'a [T],
//     clusters: Vec<Vec<&'a T>>,
//     centroids: Vec<T>,
//     k: usize,
//     iters: usize,
// }
// // WIP

// impl<'a, T: Observation + std::fmt::Debug> KmeansExecutor<'a, T> {
//     pub fn new(observations: &'a [T], k: usize, iters: usize) -> Self {
//         Self {
//             observations,
//             k,
//             iters,
//             clusters: vec![vec![]; k],
//             centroids: vec![T::zero(); k],
//         }
//     }
//     pub fn run(&mut self) {
//         self.init();
//         self.update_centroids();
//         let mut prev_sum_var = -1.0;
//         for _ in 0..self.iters {
//             self.assign_clusters();
//             self.update_centroids();
//             println!("{:?}", &self.centroids);
//             let sum_var = self.variance_sum();
//             if (sum_var - prev_sum_var).abs() < f64::EPSILON {
//                 break;
//             } else {
//                 prev_sum_var = sum_var;
//             }
//         }
//     }
//     fn init(&mut self) {
//         let mut rng = thread_rng();
//         self.observations
//             .iter()
//             .for_each(|point| self.clusters[rng.gen_range(0..self.k)].push(point));
//     }

//     fn assign_clusters(&mut self) {
//         for point in self.observations {
//             let mut min_dist = self.centroids[0].squared_distance(point);
//             let mut min_idx = 0;
//             for (i, centroid) in self.centroids.iter().enumerate() {
//                 let dist = centroid.squared_distance(point);
//                 if dist < min_dist {
//                     min_dist = dist;
//                     min_idx = i;
//                 }
//             }
//             self.clusters[min_idx].push(point);
//         }
//     }

//     fn variance_sum(&self) -> f64 {
//         self.centroids.iter().zip(self.clusters.iter()).fold(
//             0.,
//             |variance_sum, (centroid, cluster)| {
//                 variance_sum
//                     + cluster.iter().fold(0., |sum_squared_distances, point| {
//                         sum_squared_distances + point.squared_distance(centroid).powi(2)
//                     })
//             },
//         )
//     }

//     fn update_centroids(&mut self) {
//         for (centroid, cluster) in self.centroids.iter_mut().zip(self.clusters.iter()) {
//             *centroid = T::mean(cluster);
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use statrs::distribution::normal::sample_unchecked;
//     use statrs::distribution::Normal;

//     use rand::distributions::Distribution;

//     impl Distribution<f64> for Normal {
//         fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> f64 {
//             sample_unchecked(r, self.mean, self.std_dev)
//         }
//     }
//     #[test]
//     fn kmeans() {
//         let mut rng = thread_rng();
//         let mut gen_cluster = |n, x, y, r| -> Vec<Point2D> {
//             let normal_x = Normal::new(x, r).unwrap();
//             let normal_y = Normal::new(y, r).unwrap();
//             (0..n)
//                 .map(|_| Point2D {
//                     x: rng.sample(normal_x),
//                     y: rng.sample(normal_y),
//                 })
//                 .collect()
//         };
//         let cluster1 = gen_cluster(18, 1.2, 3.4, 2.0);
//         let cluster2 = gen_cluster(28, 10.2, 45.4, 3.0);
//         let cluster3 = gen_cluster(12, -15.6, -12.9, 5.);
//         let expected_centroids = [
//             Point2D::mean(&cluster1.iter().collect::<Vec<_>>()),
//             Point2D::mean(&cluster2.iter().collect::<Vec<_>>()),
//             Point2D::mean(&cluster3.iter().collect::<Vec<_>>()),
//         ];
//         let observations = [cluster1, cluster2, cluster3].concat();
//         let mut kmeans = KmeansExecutor::new(&observations, 3, 100);
//         kmeans.run();
//         println!("{:?}\n\n", &kmeans.centroids);
//         println!("{:?}", expected_centroids);
//         //println!("{:?}", &kmeans.clusters);
//     }
// }
