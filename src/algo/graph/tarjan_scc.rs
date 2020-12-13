use crate::algo::graph::UnweightedAdjacencyList;
use std::cmp::min;

const UNVISITED: i32 = -1;
struct SccSolver<'a> {
    g: &'a UnweightedAdjacencyList,
    ids: Vec<i32>,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    id: i32,
    low_link: Vec<i32>,
    sccs: Vec<Vec<usize>>,
}

impl<'a> SccSolver<'a> {
    fn new(g: &'a UnweightedAdjacencyList) -> Self {
        let n = g.vertices_count();
        Self {
            g,
            ids: vec![UNVISITED; n],
            sccs: Vec::new(),
            low_link: vec![0; n],
            id: 0,
            stack: Vec::new(),
            on_stack: vec![false; n],
        }
    }
}

impl UnweightedAdjacencyList {
    pub fn scc(&self) -> SccResult {
        let n = self.vertices_count();
        let mut s = SccSolver::new(self);

        fn _dfs(s: &mut SccSolver, at: usize) {
            s.low_link[at] = s.id;
            s.ids[at] = s.id;
            s.id += 1;
            s.stack.push(at);
            s.on_stack[at] = true;
            // visit all neighbours and min low-link on callback
            for &neighbour in &s.g[at] {
                if s.ids[neighbour] == UNVISITED {
                    _dfs(s, neighbour);
                }
                if s.on_stack[neighbour] {
                    s.low_link[at] = min(s.low_link[at], s.low_link[neighbour])
                }
            }
            // after having visited all the neighbours of `at` if we're at the start of
            // a SCC empty the seen stack until we're back to the start of the SCC
            if s.ids[at] == s.low_link[at] {
                let mut this_scc = Vec::new();
                while let Some(node) = s.stack.pop() {
                    s.on_stack[node] = false;
                    s.low_link[node] = s.ids[at];
                    this_scc.push(node);
                    if node == at {
                        s.sccs.push(this_scc);
                        break;
                    }
                }
            }
        }
        for i in 0..n {
            if s.ids[i] == UNVISITED {
                _dfs(&mut s, i);
            }
        }
        let SccSolver { sccs, low_link, .. } = s;
        SccResult { sccs, low_link }
    }
}

#[derive(Debug)]
pub struct SccResult {
    sccs: Vec<Vec<usize>>,
    low_link: Vec<i32>,
}

impl SccResult {
    pub fn scc_count(&self) -> usize {
        self.sccs.len()
    }
    pub fn in_same_scc(&self, nodes: &[usize]) -> bool {
        let id = self.low_link[nodes[0]];
        nodes.iter().all(|&node| self.low_link[node] == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tarjan_scc() {
        let graph = UnweightedAdjacencyList::new_directed(
            10,
            &[
                // SCC 1 with nodes 0,1,2
                [0, 1],
                [1, 2],
                [2, 0],
                // SCC 2 with nodes 3,4,5,6
                [5, 4],
                [5, 6],
                [3, 5],
                [4, 3],
                [4, 5],
                [6, 4],
                // SCC 3 with nodes 7,8
                [7, 8],
                [8, 7],
                // SCC 4 is node 9 all alone by itself
                // Add a few more edges to make things interesting
                [1, 5],
                [1, 7],
                [2, 7],
                [6, 8],
                [9, 8],
                [9, 4],
            ],
        );
        let res = graph.scc();
        assert_eq!(res.scc_count(), 4);
        assert!(res.in_same_scc(&[0, 1, 2]));
        assert!(res.in_same_scc(&[3, 4, 5, 6]));
        assert!(res.in_same_scc(&[7, 8]));
        assert!(res.in_same_scc(&[9]));
        // not in the same scc
        assert!(!res.in_same_scc(&[8, 9]));
    }
}
