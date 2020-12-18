//! This mod shows you how to determine if a graph is bipartite or not. This can be achieved in
//! linear time by coloring the visited nodes.
//!
//! - Time Complexity: O(V + E)

use crate::algo::graph::WeightedAdjacencyList;

pub const BLACK: i8 = 0b10;
pub const RED: i8 = BLACK ^ 1;
const EMPTY: i8 = 0;

#[derive(Debug)]
pub enum BipartiteCheckError {
    NotTwoColorable,
    UnreachableNodes,
}

impl WeightedAdjacencyList {
    // If the input graph is bipartite it has a two coloring which can be obtained
    // through this method. Each index in the returned vec is either RED or BLACK
    // indicating which color node i was colored.
    // If the graph is not bipartite, a `BipartiteCheckError` is returned.
    pub fn two_color(&self) -> Result<Vec<i8>, BipartiteCheckError> {
        let n = self.node_count();
        let mut colors = vec![EMPTY; n];
        // Do a depth first search coloring the nodes of the graph as we go.
        // This method returns the count of the number of nodes visited while
        // coloring the graph or -1 if this graph is not bipartite.
        fn color_graph(
            g: &WeightedAdjacencyList,
            node: usize,
            color: i8,
            colors: &mut [i8],
        ) -> usize {
            colors[node] = color;
            // Toggles the color between RED and BLACK by exploiting the binary representation
            // of the constants and flipping the least significant bit on and off.
            let next_color = color ^ 1;
            let mut visit_count = 1;
            for edge in &g[node] {
                if colors[edge.to] == color {
                    // Contradiction found. In a bipartite graph no two
                    // nodes of the same color can be next to each other!
                    return 0;
                } else if colors[edge.to] == next_color {
                    continue;
                } else {
                    let count = color_graph(g, edge.to, next_color, colors);
                    // If a contradiction is found propagate return -1
                    // otherwise keep track of the number of visited nodes.
                    if count == 0 {
                        return 0;
                    } else {
                        visit_count += count;
                    }
                }
            }
            visit_count
        }
        let visit_count = color_graph(self, 0, BLACK, &mut colors);
        if visit_count == 0 {
            Err(BipartiteCheckError::NotTwoColorable)
        } else if visit_count == n {
            Ok(colors)
        } else {
            Err(BipartiteCheckError::UnreachableNodes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bipartite_check() {
        // Singleton (not bipartite)
        let g = WeightedAdjacencyList::new_undirected_unweighted(1, &[[0, 0]]);
        assert!(g.two_color().is_err());

        // Two nodes one edge between them (bipartite)
        let g = WeightedAdjacencyList::new_undirected_unweighted(2, &[[0, 1]]);
        assert!(g.two_color().is_ok());

        // Triangle graph (not bipartite)
        let g = WeightedAdjacencyList::new_undirected_unweighted(3, &[[0, 1], [1, 2], [2, 0]]);
        assert!(g.two_color().is_err());

        // Disjoint graph is bipartite connected components (altogether not bipartite)
        let g = WeightedAdjacencyList::new_undirected_unweighted(4, &[[0, 1], [2, 3]]);
        assert!(g.two_color().is_err());

        // Prints:
        // Graph has 4 node(s) and the following edges:
        // 0 -> 1
        // 1 -> 0
        // 2 -> 3
        // 3 -> 2
        // This graph is bipartite: false

        // Square graph (bipartite)
        let g =
            WeightedAdjacencyList::new_undirected_unweighted(4, &[[0, 1], [1, 2], [2, 3], [3, 0]]);
        assert!(g.two_color().is_ok());

        // Square graph with additional edge (not bipartite)

        let g = WeightedAdjacencyList::new_undirected_unweighted(
            4,
            &[[0, 1], [1, 2], [2, 3], [3, 0], [0, 2]],
        );
        assert!(g.two_color().is_err());
    }
}
