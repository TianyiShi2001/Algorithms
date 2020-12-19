pub mod iterative {
    //! An implementation of iterative DFS with an adjacency list
    //!
    //! - Time Complexity: O(V + E)
    //!
    //! # Resources
    //!
    //! - [W. Fiset's video](https://www.youtube.com/watch?v=7fujbpJ0LB4&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=4)

    use crate::algo::graph::{Edge, WeightedAdjacencyList};

    impl WeightedAdjacencyList {
        /// Perform a depth first search on a graph with n nodes
        /// from a starting point to count the number of nodes
        /// in a given component.
        ///
        /// In this particular implementation we just increment a counter each time we
        /// visit a new node. This, by itself, is not of much use, but you'll soon see that
        /// many other advanced algorithms are based on this DFS prototype.
        pub fn dfs(&self, start: usize) -> usize {
            let mut count = 0;
            let mut visited = vec![false; self.node_count()];
            let mut stack = Vec::new();

            // start by visiting the start node
            stack.push(start);
            visited[start] = true;

            while let Some(node) = stack.pop() {
                count += 1;
                let neighbours = &self[node];
                for &Edge { to, weight: _ } in neighbours {
                    if !visited[to] {
                        stack.push(to);
                        visited[to] = true;
                    }
                }
            }

            count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_dfs_iterative() {
            // Create a fully connected graph
            //           (0)
            //           / \
            //        5 /   \ 4
            //         /     \
            // 10     <   -2  >
            //   +->(2)<------(1)      (4)
            //   +--- \       /
            //         \     /
            //        1 \   / 6
            //           > <
            //           (3)
            let graph = WeightedAdjacencyList::new_directed(
                5,
                &[
                    (0, 1, 4.),
                    (0, 2, 5.),
                    (1, 2, -2.),
                    (1, 3, 6.),
                    (2, 3, 1.),
                    (2, 2, 10.), // Self loop
                ],
            );

            let count = graph.dfs(0);
            assert_eq!(count, 4);
            println!("DFS node count starting at node 0: {}", count);

            let count = graph.dfs(4);
            assert_eq!(count, 1);
            println!("DFS node count starting at node 4: {}", count);
        }
    }
}
pub mod recursive {
    //! An implementation of recursive DFS with an adjacency list
    //!
    //! - Time Complexity: O(V + E)
    //!
    //! # Resources
    //!
    //! - [W. Fiset's video](https://www.youtube.com/watch?v=7fujbpJ0LB4&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=4)

    use crate::algo::graph::UnweightedAdjacencyList;

    impl UnweightedAdjacencyList {
        pub fn dfs_recursive(&self, start: usize) -> usize {
            fn _dfs(
                graph: &UnweightedAdjacencyList,
                node: usize,
                visited: &mut [bool],
                count: &mut usize,
            ) {
                *count += 1;
                visited[node] = true;
                for &neighbour in &graph[node] {
                    if !visited[neighbour] {
                        _dfs(graph, neighbour, visited, count);
                    }
                }
            }
            let mut count = 0;
            // let visited = vec![false; self.len()];
            _dfs(self, start, &mut vec![false; self.node_count()], &mut count);
            count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_dfs_recursive() {
            // Create a fully connected graph
            //           (0)
            //           / \
            //          /   \
            //         /     \
            //        <       >
            //   +->(2)<------(1)      (4)
            //   +--- \       /
            //         \     /
            //          \   /
            //           > <
            //           (3)
            const N: usize = 5;
            let mut graph = UnweightedAdjacencyList::with_size(N);
            graph.add_directed_edge(0, 1);
            graph.add_directed_edge(0, 2);
            graph.add_directed_edge(1, 2);
            graph.add_directed_edge(1, 3);
            graph.add_directed_edge(2, 3);
            graph.add_directed_edge(2, 2); // Self loop

            let count = graph.dfs_recursive(0);
            assert_eq!(count, 4);
            println!("DFS node count starting at node 0: {}", count);

            let count = graph.dfs_recursive(4);
            assert_eq!(count, 1);
            println!("DFS node count starting at node 4: {}", count);
        }
    }
}
