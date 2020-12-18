// @TAGS: graph

pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut has_incoming_edges = vec![false; n as usize];
    for edge in edges {
        has_incoming_edges[edge[1] as usize] = true;
    }
    let mut res = Vec::new();
    for (idx, x) in has_incoming_edges.into_iter().enumerate() {
        if !x {
            res.push(idx as i32);
        }
    }
    res
}
