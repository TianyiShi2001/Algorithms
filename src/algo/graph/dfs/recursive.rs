use crate::algo::graph::UnweightedAdjacencyList;

pub trait DfsRecursive {
    fn dfs(&self, start: usize) -> usize;
}

impl DfsRecursive for UnweightedAdjacencyList {
    fn dfs(&self, start: usize) -> usize {
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
        _dfs(self, start, &mut vec![false; self.len()], &mut count);
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

        let count = graph.dfs(0);
        assert_eq!(count, 4);
        println!("DFS node count starting at node 0: {}", count);

        let count = graph.dfs(4);
        assert_eq!(count, 1);
        println!("DFS node count starting at node 4: {}", count);
    }
}
