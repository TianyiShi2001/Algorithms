use crate::algo::graph::WeightedAdjacencyList;

impl WeightedAdjacencyList {
    /// Perform a depth first search on a graph with n nodes
    /// from a starting point to count the number of nodes
    /// in a given component.
    pub fn dfs(&self, start: usize) -> (usize, f64) {
        let mut count = 0;
        let mut cost = 0.;
        let mut visited = vec![false; self.vertices_count()];
        let mut stack = Vec::new();

        // start by visiting the start node
        stack.push(start);
        visited[start] = true;

        while let Some(node) = stack.pop() {
            count += 1;
            let neighbours = &self[node];
            for &edge in neighbours {
                if !visited[edge.to] {
                    cost += edge.cost;
                    stack.push(edge.to);
                    visited[edge.to] = true;
                }
            }
        }

        (count, cost)
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
        const N: usize = 5;
        let mut graph = WeightedAdjacencyList::with_size(N);
        graph.add_directed_edge(0, 1, 4.);
        graph.add_directed_edge(0, 2, 5.);
        graph.add_directed_edge(1, 2, -2.);
        graph.add_directed_edge(1, 3, 6.);
        graph.add_directed_edge(2, 3, 1.);
        graph.add_directed_edge(2, 2, 10.); // Self loop

        let (count, _cost) = graph.dfs(0);
        assert_eq!(count, 4);
        println!("DFS node count starting at node 0: {}", count);

        let (count, _cost) = graph.dfs(4);
        assert_eq!(count, 1);
        println!("DFS node count starting at node 4: {}", count);
    }
}

use crate::algo::graph::UnweightedAdjacencyList;

impl UnweightedAdjacencyList {
    /// Perform a depth first search on a graph with n nodes
    /// from a starting point to count the number of nodes
    /// in a given component.
    pub fn dfs(&self, start: usize) -> usize {
        let mut count = 0;
        let mut visited = vec![false; self.vertices_count()];
        let mut stack = Vec::new();

        // start by visiting the start node
        stack.push(start);
        visited[start] = true;

        while let Some(node) = stack.pop() {
            count += 1;
            let neighbours = &self[node];
            for &edge in neighbours {
                if !visited[edge] {
                    stack.push(edge);
                    visited[edge] = true;
                }
            }
        }

        count
    }
}
