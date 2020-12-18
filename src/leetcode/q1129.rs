pub struct S1;

impl S1 {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut steps = vec![-1; n];
        let mut vis = vec![vec![false; n]; 2];
        // tracks whether an edge (not node!) has been visited
        // vis[0] tracks red edges; vis[1] tracks blue edges

        let mut g = vec![vec![]; n];
        for (color, edge_set) in [red_edges, blue_edges].iter().enumerate() {
            for edge in edge_set {
                g[edge[0] as usize].push((edge[1] as usize, color));
            }
        }

        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 0));
        q.push_back((0, 1, 0)); // can start with either a red node or a blue node
        while let Some((node, next_color, step)) = q.pop_front() {
            if steps[node] == -1 || step < steps[node] {
                steps[node] = step
            };
            for &(nei_node, edge_color) in &g[node] {
                if edge_color == next_color {
                    if !vis[edge_color][nei_node] {
                        vis[edge_color][nei_node] = true;

                        q.push_back((nei_node, next_color ^ 1, step + 1));
                    }
                }
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_q1129() {
        for (arg1, arg2, arg3, res) in vec![
            (3, vec![vec![0, 1], vec![1, 2]],     vec![],                       vec![0, 1, -1]),
            (3, vec![],                           vec![vec![0, 1], vec![1, 2]], vec![0, 1, -1]),
            (3, vec![vec![0, 1]],                 vec![vec![2, 1]],             vec![0, 1, -1]),
            (3, vec![vec![1, 0]],                 vec![vec![2, 1]],             vec![0, -1, -1]),
            (3, vec![vec![0, 1], vec![0,2]],      vec![vec![1, 0]],             vec![0,1,1]),
            (5, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4]],
                vec![vec![1,2],vec![2,3],vec![3,1]],                            vec![0,1,2,3,7])
        ]
        .into_iter()
        {
            assert_eq!(
                S1::shortest_alternating_paths(arg1, arg2.clone(), arg3),
                res
            );
        }
    }
}
