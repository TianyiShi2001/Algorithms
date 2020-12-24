use crate::graph::{Edge, WeightedAdjacencyList};
use partial_min_max::min;

impl WeightedAdjacencyList {
    pub fn dag_shortest_path(&self, start: usize) -> Vec<f64> {
        // a node with ID on the left can only access nodes with ID on
        // the right
        let toposort = self.toposort_khan();
        let mut dists = vec![f64::INFINITY; self.node_count()];
        dists[start] = 0.;
        let i = toposort
            .iter()
            .position(|&node_id| node_id == start)
            .unwrap();
        toposort.into_iter().skip(i).for_each(|node_id| {
            let cur_dist = dists[node_id];
            if cur_dist.is_finite() {
                for &Edge { to, weight } in &self[node_id] {
                    let new_dist = cur_dist + weight;
                    let dist = &mut dists[to];
                    *dist = min(*dist, new_dist);
                }
            }
        });
        dists
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn dag_shortest_path() {
        let edges = &[
            (0, 1, 3.),
            (0, 2, 2.),
            (0, 5, 3.),
            (1, 3, 1.),
            (1, 2, 6.),
            (2, 3, 1.),
            (2, 4, 10.),
            (3, 4, 5.),
            (5, 4, 7.),
        ];
        let graph = WeightedAdjacencyList::new_directed(7, edges);
        let dists = graph.dag_shortest_path(0);
        assert_eq!(&dists, &[0., 3., 2., 3., 8., 3., f64::INFINITY])
    }
}
