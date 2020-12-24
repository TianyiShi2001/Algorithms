//! # Motivation
//!
//! - Given some points $X={x_1, x_2, \ldots x_n}$ contained within a 2D rectangle, we want to query:
//!    - all points that are contained in a query rectangle
//!    - the $k$ nearest neighbors of a point (the query point does not need to be one in the collection),
//!      also known as the knn problem
//! - The naive approach needs to compute information (checking whether it's contained in the query rectangle or measuring
//!   its distance to the query point) for every point in the collection, which takes $O(n)$ time.
//! - A quad tree is able two solve these two problems in $O(log(n))$ time
//!
//! # Resources
//!
//! - [Watch a Quadtree in action (interactive animation)](https://ericandrewlewis.github.io/how-a-quadtree-works/)
//! - [k-nearest-neighbor search using D3 quadtrees (Interactive visualization and Javascript implementation)](http://bl.ocks.org/llb4ll/8709363)
//! - [Wikipedia](https://www.wikiwand.com/en/Quadtree)
use ordered_float::OrderedFloat;
use std::cmp::min;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

impl Point2D {
    pub fn distance(&self, other: &Self) -> f64 {
        let (x0, y0) = (self.x as f64, self.y as f64);
        let (x1, y1) = (other.x as f64, other.y as f64);
        ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt()
    }
}

#[derive(Debug)]
/// A quad tree node that represents a region with its contained points
pub struct Node {
    /// The region this node encompasses
    region: Rectangle,
    /// Tracks the coordinates of points within this quad tree node.
    points: Vec<Point2D>,
    /// Maximum capacity of `points` that each node can hole
    capacity: usize,
    // When the capacity is full, add new points to subdivisions:
    // north west (nw), north east (ne), south west(sw) and south east(se).
    nw: Option<Box<Node>>,
    ne: Option<Box<Node>>,
    sw: Option<Box<Node>>,
    se: Option<Box<Node>>,
}

impl Node {
    /// Initialise a new node
    pub fn new(capacity: usize, region: Rectangle) -> Self {
        Self {
            points: Vec::new(),
            nw: None,
            ne: None,
            sw: None,
            se: None,
            region,
            capacity,
        }
    }
    /// Insert a point into the node.
    pub fn push(&mut self, point: Point2D) -> bool {
        if !self.region.contains_point(&point) {
            false
        } else if self.points.len() < self.capacity {
            self.points.push(point);
            true
        } else {
            // capacity is full; push point onto an appropriate child

            // Find the center of this region at (cx, cy)
            let cx = (self.region.x0 + self.region.x1) / 2;
            let cy = (self.region.y0 + self.region.y1) / 2;
            // Lazily subdivide each of the regions into four parts to save memory.
            if self.nw.is_none() {
                self.nw = Some(Box::new(Node::new(
                    self.capacity,
                    Rectangle::new(self.region.x0, self.region.y0, cx, cy),
                )));
            }
            if self.nw.as_mut().unwrap().push(point) {
                return true;
            }
            if self.ne.is_none() {
                self.ne = Some(Box::new(Node::new(
                    self.capacity,
                    Rectangle::new(cx, self.region.y0, self.region.x1, cy),
                )));
            }
            if self.ne.as_mut().unwrap().push(point) {
                return true;
            }
            if self.sw.is_none() {
                self.sw = Some(Box::new(Node::new(
                    self.capacity,
                    Rectangle::new(self.region.x0, cy, cx, self.region.y1),
                )));
            }
            if self.sw.as_mut().unwrap().push(point) {
                return true;
            }
            if self.se.is_none() {
                self.se = Some(Box::new(Node::new(
                    self.capacity,
                    Rectangle::new(cx, cy, self.region.x1, self.region.y1),
                )));
            }
            if self.se.as_mut().unwrap().push(point) {
                return true;
            }
            false
        }
    }

    /// Count how many points are found within a certain rectangular region
    pub fn count(&self, area: &Rectangle) -> usize {
        if !self.region.intersects(&area) {
            0
        } else {
            let count = if area.contains_rectangle(&self.region) {
                // The area we're considering fully contains
                // the region of this node, so simply add the
                // number of points within this region to the count
                self.points.len()
            } else {
                // Our regions overlap, so some points in this
                // region may intersect with the area we're considering
                self.points
                    .iter()
                    .filter(|p| area.contains_point(*p))
                    .count()
            };
            // Dig into each of the quadrants and count all points
            // which overlap with the area and sum their count
            count
                + self.nw.as_ref().map_or(0, |node| node.count(&area))
                + self.ne.as_ref().map_or(0, |node| node.count(&area))
                + self.sw.as_ref().map_or(0, |node| node.count(&area))
                + self.se.as_ref().map_or(0, |node| node.count(&area))
        }
    }
    // Find all points that lie within a certain rectangular region
    pub fn query(&self, area: &Rectangle) -> Vec<&Point2D> {
        let mut res = Vec::new();
        fn _query<'a>(node: &'a Node, area: &Rectangle, res: &mut Vec<&'a Point2D>) {
            if node.region.intersects(&area) {
                if area.contains_rectangle(&node.region) {
                    res.extend(node.points.iter());
                } else {
                    res.extend(node.points.iter().filter(|p| area.contains_point(*p)))
                };
                if let Some(child) = node.nw.as_ref() {
                    _query(child, area, res);
                }
                if let Some(child) = node.ne.as_ref() {
                    _query(child, area, res);
                }
                if let Some(child) = node.sw.as_ref() {
                    _query(child, area, res);
                }
                if let Some(child) = node.se.as_ref() {
                    _query(child, area, res);
                }
            }
        }
        _query(&self, area, &mut res);
        res
    }
    /// Find the k nearest neighbors of a certain point
    pub fn knn(&self, point: &Point2D, k: usize) -> Vec<(Point2D, f64)> {
        // tracks the k nearest neighbors along with their distance to the query point
        // a max-heap is used because later we need to determine whether each new point has a shorter distance
        // than the worst point (with longest distance) in the heap
        let mut result_pq: BinaryHeap<(OrderedFloat<f64>, Point2D)> = BinaryHeap::with_capacity(k);
        // tracks the next 'most promising node' whose region is closest (i.e. with shortest distance) to the
        // query point. Thus, this needs to be a min-heap.
        let mut node_pq = BinaryHeap::new();
        // push the root onto the node priority queue
        node_pq.push((
            -OrderedFloat(self.region.min_distance_to_point(&point)),
            self as *const Node, // `Ord` is not implemented for `&Node`; using a raw pointer is a quick and dirty solution
                                 // (we won't be modifying the tree while running this function so using a raw pointer is ok)
        ));
        while let Some((_dist, node)) = node_pq.pop() {
            let node: &Node = unsafe { &*node };
            for point1 in &node.points {
                // Get distance from the query point to this point
                let distance = point.distance(point1);
                if result_pq.len() < k {
                    result_pq.push((OrderedFloat(distance), *point1));
                } else {
                    // Get the longest distance.
                    let mx = result_pq
                        .peek()
                        .map_or(f64::INFINITY, |(dist, _p)| dist.into_inner());

                    if distance <= mx {
                        result_pq.pop().unwrap();
                        result_pq.push((OrderedFloat(distance), *point1));
                    }
                }
            }
            for child in [&node.nw, &node.ne, &node.sw, &node.se].iter() {
                if let Some(child) = child {
                    let dist = child.region.min_distance_to_point(&point);
                    // here is the heart of this algorithm.
                    // only add a child onto the queue if it is possible to contain a point
                    // that's closer to the query point than the worst point in the current
                    // results.
                    if dist <= result_pq.peek().unwrap().0.into_inner() {
                        node_pq.push((-OrderedFloat(dist), child.as_ref() as *const Node));
                    }
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

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rectangle {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        Self { x0, y0, x1, y1 }
    }

    /// Check for an intersection between two rectangles. The easiest way to do this is to
    /// check if the two rectangles do not intersect and negate the logic afterwards.
    fn intersects(&self, other: &Rectangle) -> bool {
        !(other.x1 < self.x0 || other.x0 > self.x1 || other.y0 > self.y1 || other.y1 < self.y0)
    }

    /// Check if a point (x, y) is within this rectangle, this
    /// includes the boundary of the rectangle.
    fn contains_point(&self, point: &Point2D) -> bool {
        (self.x0 <= point.x && point.x <= self.x1) && (self.y0 <= point.y && point.y <= self.y1)
    }

    // Check if another rectangle is strictly contained within this rectangle.
    fn contains_rectangle(&self, other: &Rectangle) -> bool {
        self.contains_point(&Point2D {
            x: other.x0,
            y: other.y0,
        }) && self.contains_point(&Point2D {
            x: other.x1,
            y: other.y1,
        })
    }

    /// Calculate the minimum distance from a point to this rectangle.
    fn min_distance_to_point(&self, point: &Point2D) -> f64 {
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
    // nw, ne, sw, se vertices
    fn vertices(&self) -> [Point2D; 4] {
        [
            Point2D {
                x: self.x0,
                y: self.y0,
            },
            Point2D {
                x: self.x1,
                y: self.y0,
            },
            Point2D {
                x: self.x0,
                y: self.y1,
            },
            Point2D {
                x: self.x1,
                y: self.y1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use rand::{thread_rng, Rng};
    const N: usize = 200;
    const WIDTH: usize = 100;
    const HEIGHT: usize = 50;
    const CAPACITY: usize = 10;

    lazy_static! {
        static ref POINTS: Vec<Point2D> = {
            let mut rng = thread_rng();
            (0..N)
                .map(|_| Point2D {
                    x: rng.gen_range(0..WIDTH),
                    y: rng.gen_range(0..HEIGHT),
                })
                .collect()
        };
        static ref QT: Node = {
            let mut qt = Node::new(CAPACITY, Rectangle::new(0, 0, WIDTH, HEIGHT));
            for &point in POINTS.iter() {
                if !qt.push(point) {
                    panic!("boom!");
                }
            }
            qt
        };
    }
    #[test]
    fn query() {
        const WINDOW: Rectangle = Rectangle {
            x0: 32,
            y0: 12,
            x1: 67,
            y1: 25,
        };
        let mut expected = POINTS
            .iter()
            .filter(|p| WINDOW.contains_point(*p))
            .collect::<Vec<_>>();
        expected.sort();
        let mut actual = QT.query(&WINDOW);
        actual.sort();
        assert_eq!(actual, expected);
    }

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
