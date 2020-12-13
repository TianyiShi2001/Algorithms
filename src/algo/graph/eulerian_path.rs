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

impl UnweightedAdjacencyList {
    fn count_in_out_degrees(&self) -> [Vec<usize>; 2] {
        let mut in_degrees = vec![0; self.vertices_count()];
        let mut out_degrees = vec![0; self.vertices_count()];
        for [from, to] in self.edges() {
            out_degrees[from] += 1;
            in_degrees[to] += 1;
        }
        [in_degrees, out_degrees]
    }
    /// Returns a list of `edges_count + 1` node ids that give the Eulerian path or
    /// `None` if no path exists or the graph is disconnected.
    pub fn eulerian_path(&self) -> Option<Vec<usize>> {
        let n = self.vertices_count();
        let has_eulerian_path = |[in_degrees, out_degrees]: [&[usize]; 2]| {
            let mut start = None;
            let mut has_end = false;
            let mut start_default = None;
            for (node, (&i, &o)) in in_degrees.iter().zip(out_degrees.iter()).enumerate() {
                if (i as isize - o as isize).abs() > 1 {
                    return None;
                }
                if i.wrapping_sub(o) == 1 {
                    if has_end {
                        return None;
                    } else {
                        has_end = true;
                    }
                }
                if o.wrapping_sub(i) == 1 {
                    if start.is_some() {
                        return None;
                    } else {
                        start = Some(node)
                    }
                }
                if start_default.is_none() && o > 0 {
                    start_default = Some(node);
                }
            }
            if start.is_some() ^ has_end {
                None
            } else {
                Some(start.unwrap_or(start_default.unwrap()))
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
        match has_eulerian_path([&i, &o]) {
            None => None,
            Some(start) => {
                let mut path = Vec::with_capacity(n);
                _dfs(self, &mut o, &mut path, start);
                path.reverse();
                // Make sure all edges of the graph were traversed. It could be the
                // case that the graph is disconnected in which case return `None`.
                if path.len() == self.edges_count() + 1 {
                    Some(path)
                } else {
                    // disconnected graph
                    None
                }
            }
        }
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
    fn test_eulerian_path_invalid() {
        let g = UnweightedAdjacencyList::new_directed(2, &[[0, 1], [0, 1]]);
        assert!(g.eulerian_path().is_none());

        let g = UnweightedAdjacencyList::new_directed(3, &[[0, 1], [1, 0], [1, 2], [2, 0], [2, 0]]);
        assert!(g.eulerian_path().is_none());

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
        assert!(g.eulerian_path().is_none());
    }
}
