use crate::geometry::Point2D;
use crate::graph::network_flow::{MaxFlowSolver, NetworkFlowAdjacencyList};

#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
pub fn mice_and_owls<S: MaxFlowSolver>(mice: &[Mouse], holes: &[Hole], radius: f64) -> i32 {
    let m = mice.len();
    let h = holes.len();
    let n = m + h + 2;

    let mut g = NetworkFlowAdjacencyList::with_size(n);
    let s = g.source;
    let t = g.sink;

    for mouse in 0..m {
        g.add_edge(s, mouse, 1);
    }

    // Hook up each mouse with the holes they are able to reach
    for (mouse_id, mouse) in mice.iter().enumerate() {
        for (j, hole) in holes.iter().enumerate() {
            let hole_id = m + j;
            if mouse.position.distance_to_point(&hole.position) <= radius {
                g.add_edge(mouse_id, hole_id, 1);
            }
        }
    }

    for i in 0..h {
        g.add_edge(m + i, t, holes[i].capacity);
    }

    S::max_flow(&mut g)
}

pub struct Mouse {
    position: Point2D,
}

impl Mouse {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Point2D { x, y },
        }
    }
}

pub struct Hole {
    position: Point2D,
    capacity: i32,
}

impl Hole {
    pub fn new(x: f64, y: f64, capacity: i32) -> Self {
        Self {
            position: Point2D { x, y },
            capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::network_flow::EdmondsKarpSolver;
    #[test]
    fn test_mice_and_owls() {
        let mice = &[
            Mouse::new(1., 0.),
            Mouse::new(8., 1.),
            Mouse::new(12., 0.),
            Mouse::new(12., 4.),
            Mouse::new(15., 5.),
        ];
        let holes = &[
            Hole::new(1., 1., 1),
            Hole::new(10., 2., 2),
            Hole::new(14., 5., 1),
        ];

        let res = mice_and_owls::<EdmondsKarpSolver>(mice, holes, 3.);
        assert_eq!(res, 4)
    }
}
