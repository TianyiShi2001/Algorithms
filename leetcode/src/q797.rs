// @TAGS: DFS, graph

//! # Related Algorithms
//!
//! [`crate::algo::graph::dfs`]

#[allow(clippy::ptr_arg)]
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut path = Vec::new();
    fn dfs(g: &Vec<Vec<i32>>, node: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        path.push(node as i32);
        if node == g.len() - 1 {
            res.push(path.clone());
        } else {
            for &next in &g[node] {
                dfs(g, next as usize, path, res);
                // don't forget to backtrack
                path.pop().unwrap();
            }
        }
    }
    dfs(&graph, 0, &mut path, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q797_1() {
        // 0 -> 1
        // ↓    ↓
        // 2 -> 3
        let inp = vec![
            // from 0 to
            vec![1, 2],
            // from 1 to
            vec![3],
            // from 2 to
            vec![3],
            // from 3 to
            vec![],
        ];
        let out = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(all_paths_source_target(inp), out);
    }

    #[test]
    fn test_q797_2() {
        let inp = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let out = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(all_paths_source_target(inp), out);
    }

    #[test]
    fn test_q797_3() {
        let inp = vec![vec![1], vec![]];
        let out = vec![vec![0, 1]];
        assert_eq!(all_paths_source_target(inp), out);
    }

    #[test]
    fn test_q797_4() {
        let inp = vec![vec![1, 2, 3], vec![2], vec![3], vec![]];
        let out = vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]];
        assert_eq!(all_paths_source_target(inp), out);
    }

    #[test]
    fn test_q797_5() {
        let inp = vec![vec![1, 3], vec![2], vec![3], vec![]];
        let out = vec![vec![0, 1, 2, 3], vec![0, 3]];
        assert_eq!(all_paths_source_target(inp), out);
    }
}
