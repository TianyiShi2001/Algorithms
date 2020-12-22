use super::{MaxFlowSolver, NetworkFlowAdjacencyList};
use std::collections::VecDeque;

pub struct EdmondsKarpSolver {}

impl MaxFlowSolver for EdmondsKarpSolver {
    fn max_flow(graph: &mut NetworkFlowAdjacencyList) -> i32 {
        let n = graph.node_count();
        let mut visited = vec![0; n];
        let mut visited_token = 1;

        let mut bfs = |visited_token| {
            let mut q = VecDeque::with_capacity(n);
            let mut prev = vec![None; n];
            visited[graph.source] = visited_token;
            q.push_back(graph.source);
            while let Some(node) = q.pop_front() {
                if node == graph.sink {
                    break;
                }
                for edge in &graph[node] {
                    let _edge = edge.borrow();
                    if _edge.reamaining_capacity() > 0 && visited[_edge.to] != visited_token {
                        visited[_edge.to] = visited_token;
                        prev[_edge.to] = Some(edge.clone());
                        q.push_back(_edge.to);
                    }
                }
            }
            if prev[graph.sink].is_none() {
                return 0;
            }

            let mut bottleneck = i32::MAX;
            let mut node = graph.sink;

            while let Some(prev_edge) = &prev[node] {
                bottleneck = std::cmp::min(bottleneck, prev_edge.borrow().reamaining_capacity());
                node = prev_edge.borrow().from;
            }

            node = graph.sink;

            while let Some(prev_edge) = &prev[node] {
                prev_edge.borrow_mut().augment(bottleneck);
                node = prev_edge.borrow().from;
            }

            bottleneck
        };
        let mut flow = 0;
        let mut f = -1;
        while f != 0 {
            f = bfs(visited_token);

            flow += f;
            visited_token += 1;
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_max_flow(n: usize, edges: &[(usize, usize, i32)], expected_max_flow: i32) {
        let mut graph = NetworkFlowAdjacencyList::from_edges(n, edges);
        let max_flow = EdmondsKarpSolver::max_flow(&mut graph);
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
