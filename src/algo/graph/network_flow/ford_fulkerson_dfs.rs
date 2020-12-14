//! An implementation of the Ford-Fulkerson (FF) method with a DFS as a method of finding augmenting
//! paths. FF allows you to find the max flow through a directed graph as well as the min cut as a
//! byproduct.
//!
//! - Time Complexity: O(fV^2), where f is the max flow
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=LdOnanfc5TM&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=33)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=Xu8jjJnwvxE&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=34)
//! - [Wikipedia](https://www.wikiwand.com/en/Ford%E2%80%93Fulkerson_algorithm)

use super::{Edge, NetworkFlowAdjacencyList};

struct FordFulkersonSolver<'a> {
    g: &'a mut NetworkFlowAdjacencyList,
    s: usize,
    t: usize,
    visited: Vec<u32>,
    visited_token: u32,
}

impl<'a> FordFulkersonSolver<'a> {
    pub fn init(g: &'a mut NetworkFlowAdjacencyList, source: usize, sink: usize) -> Self {
        let n = g.vertices_count();
        Self {
            g,
            s: source,
            t: sink,
            visited: vec![0; n],
            visited_token: 1,
        }
    }
    pub fn solve(&mut self) -> i32 {
        let mut flow = 0;
        let mut f = -1;
        while f != 0 {
            f = self.dfs(self.s, i32::MAX);
            flow += f;
            self.visited_token += 1;
        }
        flow
    }
    pub fn dfs(&mut self, node: usize, flow: i32) -> i32 {
        // at sink node, return augmented path flow
        if node == self.t {
            return flow;
        }
        self.visited[node] = self.visited_token;
        for edge in
            unsafe { &mut *(&mut self.g[node] as *mut Vec<std::rc::Rc<std::cell::RefCell<Edge>>>) }
        {
            let rcap = edge.borrow().reamaining_capacity();
            if self.visited[edge.borrow().to] != self.visited_token && rcap > 0 {
                let bottleneck = self.dfs(edge.borrow().to, std::cmp::min(flow, rcap));
                // if we made it from s --> t (a.k.a bottleneck > 0) then augment flow with the bottleneck value
                if bottleneck > 0 {
                    edge.borrow_mut().augment(bottleneck);
                    return bottleneck;
                }
            }
        }
        0
    }
}

impl NetworkFlowAdjacencyList {
    pub fn ford_fulkerson(&mut self, source: usize, sink: usize) -> i32 {
        let mut s = FordFulkersonSolver::init(self, source, sink);
        s.solve()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_max_flow(n: usize, edges: &[(usize, usize, i32)], expected_max_flow: i32) {
        let mut graph = NetworkFlowAdjacencyList::from_edges(n, edges);
        let max_flow = graph.ford_fulkerson(n - 1, n - 2);
        assert_eq!(max_flow, expected_max_flow);
    }

    #[test]
    fn test_small_graph() {
        test_max_flow(
            6,
            &[
                // Source edges
                (5, 0, 10),
                (5, 1, 10),
                // Sink edges
                (2, 4, 10),
                (3, 4, 10),
                // Middle edges
                (0, 1, 2),
                (0, 2, 4),
                (0, 3, 8),
                (1, 3, 9),
                (3, 2, 6),
            ],
            19,
        );
    }

    #[test]
    fn test_disconnected() {
        test_max_flow(4, &[(3, 0, 9), (1, 2, 9)], 0);
    }

    #[test]
    fn test_medium_graph() {
        test_max_flow(
            12,
            &[
                // from source
                (11, 0, 5),
                (11, 1, 20),
                (11, 2, 10),
                // to sink
                (7, 10, 7),
                (8, 10, 15),
                (9, 10, 60),
                // middle
                (0, 1, 3),
                (0, 5, 4),
                (1, 4, 14),
                (1, 5, 14),
                (2, 1, 5),
                (2, 3, 4),
                (3, 4, 3),
                (3, 9, 11),
                (4, 6, 4),
                (4, 8, 22),
                (5, 6, 8),
                (5, 7, 3),
                (6, 7, 12),
                (7, 8, 9),
                (8, 9, 11),
            ],
            29,
        );
    }
}
