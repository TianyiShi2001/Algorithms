pub struct Graph {
    pub adj: Vec<Vec<usize>>,
    pub one_based_index: bool,
}
// WIP

impl Graph {
    pub fn from_adjacency_list_leetcode(
        leetcode_adjacency_list: Vec<Vec<i32>>,
        one_based_index: bool,
    ) -> Self {
        let adj = leetcode_adjacency_list
            .into_iter()
            .map(|edges| {
                edges
                    .into_iter()
                    .map(|x| if one_based_index { x - 1 } else { x } as usize)
                    .collect()
            })
            .collect();
        Self {
            adj,
            one_based_index,
        }
    }
}
