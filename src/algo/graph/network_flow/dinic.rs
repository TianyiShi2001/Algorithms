//! Implementation of Dinic's network flow algorithm. The algorithm works by first constructing a
//! level graph using a BFS and then finding augmenting paths on the level graph using multiple DFSs.
//!
//! - Time Complexity: O(EV²)
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=M6cm8UeeziI&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=42)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=_SdF4KK_dyM&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=43)
//! - [Wikipedia](https://www.wikiwand.com/en/Dinic%27s_algorithm)

use super::{Edge, MaxFlowSolver, NetworkFlowAdjacencyList};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
pub struct DinicSolver<'a> {
    g: &'a mut NetworkFlowAdjacencyList,
    n: usize,
    levels: Vec<isize>,
}

const INF: i32 = i32::MAX / 2;

impl<'a> DinicSolver<'a> {
    fn init(g: &'a mut NetworkFlowAdjacencyList) -> Self {
        let n = g.vertices_count();
        Self {
            g,
            n,
            levels: vec![0; n],
        }
    }
    fn solve(&mut self) -> i32 {
        let mut max_flow = 0;

        while self.bfs() {
            // `next[i]` indicates the next unused edge index in the adjacency list for node `i`. This is part
            // of the Shimon Even and Alon Itai optimization of pruning deads ends as part of the DFS phase.
            let mut next = vec![0usize; self.n];
            // Find max flow by adding all augmenting path flows.
            let mut f = -1;
            while f != 0 {
                f = self.dfs(self.g.source, &mut next, INF);
                max_flow += f;
            }
        }
        max_flow
    }

    // for i in 0..self.n if (self.levels[i] != -1) minCut[i] = true;
    // }

    // Do a BFS from source to sink and compute the depth/level of each node
    // which is the minimum number of edges from that node to the source.
    fn bfs(&mut self) -> bool {
        self.levels = vec![-1; self.n];
        self.levels[self.g.source] = 0;
        let mut q = VecDeque::with_capacity(self.n);
        q.push_back(self.g.source);
        while let Some(node) = q.pop_front() {
            for edge in &self.g[node] {
                let edge = edge.borrow();
                let rcap = edge.reamaining_capacity();
                if rcap > 0 && self.levels[edge.to] == -1 {
                    self.levels[edge.to] = self.levels[node] + 1;
                    q.push_back(edge.to)
                }
            }
        }
        self.levels[self.g.sink] != -1
    }

    fn dfs(&mut self, at: usize, next: &mut [usize], flow: i32) -> i32 {
        if at == self.g.sink {
            return flow;
        }
        let num_edges = self.g[at].len();
        while next[at] < num_edges {
            let edge = unsafe { &*(&self.g[at][next[at]] as *const Rc<RefCell<Edge>>) };
            let _edge = edge.borrow();
            let rcap = _edge.reamaining_capacity();
            if rcap > 0 && self.levels[_edge.to] == self.levels[at] + 1 {
                let bottleneck = self.dfs(_edge.to, next, std::cmp::min(flow, rcap));
                if bottleneck > 0 {
                    edge.borrow_mut().augment(bottleneck);
                }
            }
            next[at] += 1;
        }

        0
    }
}

impl<'a> MaxFlowSolver for DinicSolver<'a> {
    fn max_flow(graph: &mut NetworkFlowAdjacencyList) -> i32 {
        let mut s = DinicSolver::init(graph);
        s.solve()
    }
}
