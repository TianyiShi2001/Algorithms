//! Implementation of finding an Eulerian Path on a graph. This implementation verifies that the
//! input graph is fully connected and supports self loops and repeated edges between nodes.
//!
//! Time Complexity: $O(E)$
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=xR4sGgwtR2I&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=27)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=8MpoO2zA2l4&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=28)

use crate::algo::graph::UnweightedAdjacencyList;

#[derive(Debug)]
pub enum EulerianPathError {
    DisconnectedGraph,
    InvalidDegrees,
}

impl UnweightedAdjacencyList {
    fn count_in_out_degrees(&self) -> [Vec<usize>; 2] {
        let mut in_degrees = vec![0; self.node_count()];
        let mut out_degrees = vec![0; self.node_count()];
        for [from, to] in self.edges() {
            out_degrees[from] += 1;
            in_degrees[to] += 1;
        }
        [in_degrees, out_degrees]
    }
    /// Returns a list of `edge_count + 1` node ids that give the Eulerian path or
    /// an `EulerianPathError` if no path exists or the graph is disconnected.
    pub fn eulerian_path(&self) -> Result<Vec<usize>, EulerianPathError> {
        let n = self.node_count();
        // Checks whether a graph has an Eulerian path by inspecting the in and out degrees of all nodes.
        // If a graph contains an Eulerian path, either
        //    - every node has the same number of incoming edges and outgoing edges (in which case the path
        //      is also an Eulerian circuit/cycle), OR
        //    - exactly one node has one more outgoing edges than incoming edges, and exactly
        //      one node has more more incoming edges than outgoing edges. They become the start
        //      and end nodes of the Eulerian path, respectively.
        let has_eulerian_path = |[in_degrees, out_degrees]: [&[usize]; 2]| {
            // tracks the start node (with an extra outgoing edge), if any
            let mut start = None;
            // tracks if the graph has a end node (with an extra incoming edge)
            // its value is not important, when constructing the path only a start node is required
            let mut has_end = false;
            // if no start or end nodes are found, the graph has an Eulerian cycle, in which case
            // the first non-singleton node's id is returned (any non-singleton node can be the starting point)
            let mut start_default = None;
            for (node, (&i, &o)) in in_degrees.iter().zip(out_degrees.iter()).enumerate() {
                // The difference is more than 1
                if (i as isize - o as isize).abs() > 1 {
                    return None;
                }
                // start node (one extra outgoing edge)
                if o.wrapping_sub(i) == 1 {
                    if start.is_some() {
                        // can only have exactly one such node
                        return None;
                    } else {
                        start = Some(node)
                    }
                }
                // same logic for the end node.
                if i.wrapping_sub(o) == 1 {
                    if has_end {
                        return None;
                    } else {
                        has_end = true;
                    }
                }
                // set the default start node to be the first non-singleton node
                if start_default.is_none() && o > 0 {
                    start_default = Some(node);
                }
            }
            // Fails If the graph has only a start but not an end node or vice versa
            // (it must contain either none of both or exactly one of each)
            if start.is_some() ^ has_end {
                None
            } else {
                Some(start.unwrap_or_else(|| start_default.unwrap()))
            }
        };
        let [i, mut o] = self.count_in_out_degrees();
        fn _dfs(
            g: &UnweightedAdjacencyList,
            out: &mut Vec<usize>,
            path: &mut Vec<usize>,
            at: usize,
        ) {
            // while the current node still has outgoing edge(s)
            while out[at] > 0 {
                out[at] -= 1;
                // select the next unvisited outgoing edge
                let next = g[at][out[at]];
                _dfs(g, out, path, next);
            }
            // add current node to solution.
            path.push(at);
        };
        has_eulerian_path([&i, &o]).map_or(Err(EulerianPathError::InvalidDegrees), |start| {
            let mut path = Vec::with_capacity(n);
            _dfs(self, &mut o, &mut path, start);
            path.reverse();
            // Make sure all edges of the graph were traversed. It could be the
            // case that the graph is disconnected.
            if path.len() == self.edge_count() + 1 {
                Ok(path)
            } else {
                // disconnected graph
                Err(EulerianPathError::DisconnectedGraph)
            }
        })
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_eulerian_path() {
        let g = UnweightedAdjacencyList::new_directed(
            7,
            &[
                [1, 2],
                [1, 3],
                [2, 2],
                [2, 4],
                [2, 4],
                [3, 1],
                [3, 2],
                [3, 5],
                [4, 3],
                [4, 6],
                [5, 6],
                [6, 3],
            ],
        );
        let path = g.eulerian_path().unwrap();
        assert_eq!(&path, &[1, 3, 5, 6, 3, 2, 4, 3, 1, 2, 2, 4, 6]);

        let g = UnweightedAdjacencyList::new_directed(
            5,
            &[[0, 1], [1, 2], [1, 4], [1, 3], [2, 1], [4, 1]],
        );
        let path = g.eulerian_path().unwrap();
        assert_eq!(&path, &[0, 1, 4, 1, 2, 1, 3]);
    }
    #[test]
    fn test_eulerian_path_invalid1() {
        let g = UnweightedAdjacencyList::new_directed(2, &[[0, 1], [0, 1]]);
        assert!(matches!(
            g.eulerian_path(),
            Err(EulerianPathError::InvalidDegrees)
        ));
    }
    #[test]
    fn test_eulerian_path_invalid2() {
        let g = UnweightedAdjacencyList::new_directed(3, &[[0, 1], [1, 0], [1, 2], [2, 0], [2, 0]]);
        assert!(matches!(
            g.eulerian_path(),
            Err(EulerianPathError::InvalidDegrees)
        ));
    }
    #[test]
    fn test_eulerian_path_invalid3() {
        let g = UnweightedAdjacencyList::new_directed(
            4,
            &[
                [0, 2],
                [2, 1],
                [2, 3],
                [3, 0],
                [3, 1],
                [1, 3],
                [1, 0],
                [1, 2],
                [0, 3],
                [2, 0],
            ],
        );
        assert!(matches!(
            g.eulerian_path(),
            Err(EulerianPathError::InvalidDegrees)
        ));
    }

    #[test]
    fn test_eulerian_path_invalid4() {
        let g = UnweightedAdjacencyList::new_directed(
            8,
            &[
                [0, 1],
                [1, 2],
                [2, 3],
                [3, 1],
                [4, 5],
                [5, 6],
                [6, 7],
                [7, 4],
            ],
        );
        assert!(matches!(
            g.eulerian_path(),
            Err(EulerianPathError::DisconnectedGraph)
        ));
    }
}
