use crate::algo::graph::network_flow::{
    EdmondsKarpSolver, MaxFlowSolver, NetworkFlowAdjacencyList,
};
#[allow(clippy::many_single_char_names)]
#[allow(clippy::needless_range_loop)]
pub fn mice_and_owls(mice: &[Mouse], holes: &[Hole], radius: f64) -> i32 {
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
            if mouse.position.distance(&hole.position) <= radius {
                g.add_edge(mouse_id, hole_id, 1);
            }
        }
    }

    for i in 0..h {
        g.add_edge(m + i, t, holes[i].capacity);
    }

    EdmondsKarpSolver::max_flow(&mut g)
}

#[derive(Copy, Clone)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn distance(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct Mouse {
    position: Point2D,
}

pub struct Hole {
    position: Point2D,
    capacity: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mice_and_owls() {
        let mice = &[
            Mouse {
                position: Point2D { x: 1., y: 0. },
            },
            Mouse {
                position: Point2D { x: 0., y: 1. },
            },
            Mouse {
                position: Point2D { x: 8., y: 1. },
            },
            Mouse {
                position: Point2D { x: 12., y: 0. },
            },
            Mouse {
                position: Point2D { x: 12., y: 4. },
            },
            Mouse {
                position: Point2D { x: 15., y: 5. },
            },
        ];
        let holes = &[
            Hole {
                position: Point2D { x: 1., y: 1. },
                capacity: 1,
            },
            Hole {
                position: Point2D { x: 10., y: 2. },
                capacity: 2,
            },
            Hole {
                position: Point2D { x: 14., y: 5. },
                capacity: 1,
            },
        ];

        let res = mice_and_owls(mice, holes, 3.);
        assert_eq!(res, 4)
    }
}
