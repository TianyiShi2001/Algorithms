use crate::data_structures::quadtree::{Node, Point2D, Rectangle};

use ordered_float::OrderedFloat;
use std::cmp::min;
use std::collections::BinaryHeap;

impl Rectangle {
    /// Calculate the minimum distance from a point to this rectangle.
    pub fn min_distance_to_point(&self, point: &Point2D) -> f64 {
        let (x, y) = (point.x as i64, point.y as i64);
        let dx0 = x - self.x0 as i64;
        let dx1 = x - self.x1 as i64;
        let dy0 = y - self.y0 as i64;
        let dy1 = y - self.y1 as i64;

        if dx0 * dx1 <= 0 {
            // x is between x1 and x2
            if dy0 * dy1 <= 0 {
                // (x, y) is inside the rectangle
                0. // return 0 if the point is in the rectangle
            } else {
                min(dy0.abs(), dy1.abs()) as f64
            }
        } else if dy0 * dy1 <= 0 {
            // y is between y1 and y2
            min(dx0.abs(), dx1.abs()) as f64
        } else {
            self.vertices()
                .iter()
                .map(|v| v.distance(point))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }
    }
}

impl Node {
    /// Find the k nearest neighbors of a certain point
    pub fn knn(&self, point: &Point2D, k: usize) -> Vec<(&Point2D, f64)> {
        // tracks the k nearest neighbors along with their distance to the query point
        // a max-heap is used because later we need to determine whether each new point has a shorter distance
        // than the worst point (with longest distance) in the heap
        let mut result_pq: BinaryHeap<(OrderedFloat<f64>, &Point2D)> = BinaryHeap::with_capacity(k);
        // tracks the next 'most promising node' whose region is closest (i.e. with shortest distance) to the
        // query point. Thus, this needs to be a min-heap.
        let mut node_pq = BinaryHeap::new();
        // push the root onto the node priority queue
        node_pq.push((
            -OrderedFloat(self.region.min_distance_to_point(point)),
            self as *const Node, // `Ord` is not implemented for `&Node`; using a raw pointer is a quick and dirty solution
                                 // (we won't be modifying the tree while running this function so using a raw pointer is ok)
        ));
        while let Some((_dist, node)) = node_pq.pop() {
            let node: &Node = unsafe { &*node };
            for point1 in &node.points {
                // Get distance from the query point to this point
                let distance = point.distance(point1);
                if result_pq.len() < k {
                    result_pq.push((OrderedFloat(distance), point1));
                } else {
                    // Get the longest distance.
                    let mx = result_pq
                        .peek()
                        .map_or(f64::INFINITY, |(dist, _p)| dist.into_inner());

                    if distance <= mx {
                        result_pq.pop().unwrap();
                        result_pq.push((OrderedFloat(distance), point1));
                    }
                }
            }
            for child in [&node.nw, &node.ne, &node.sw, &node.se]
                .iter()
                .copied()
                .flatten()
            // flatten to ignore `None`s
            {
                let dist = child.region.min_distance_to_point(point);
                // here is the heart of this algorithm.
                // only add a child onto the queue if it is possible to contain a point
                // that's closer to the query point than the worst point in the current
                // results.
                if dist <= result_pq.peek().unwrap().0.into_inner() {
                    node_pq.push((-OrderedFloat(dist), child.as_ref() as *const Node));
                }
            }
        }
        result_pq
            .into_iter()
            // .into_iter_sorted() // TODO: use into_iter_sorted() when it becomes stable
            .map(|(dist, point)| (point, dist.into_inner()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::quadtree::tests::*;

    #[test]
    fn knn() {
        let target = Point2D { x: 32, y: 25 };
        let k = 10;
        let mut expected = POINTS
            .iter()
            .map(|p| p.distance(&target))
            .collect::<Vec<_>>();
        expected.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        let mut actual: Vec<_> = QT.knn(&target, k).into_iter().map(|x| x.1).collect();
        actual.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        for (a, b) in actual.iter().zip(expected.iter().take(k)) {
            println!("{:?}", (a, b));
            assert!((*a - *b).abs() < std::f64::EPSILON);
        }
    }
}
