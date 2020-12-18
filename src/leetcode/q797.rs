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
