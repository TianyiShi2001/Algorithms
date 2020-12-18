pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for pair in dislikes {
        let (a, b) = (pair[0], pair[1]);
        g[a as usize].push(b as usize);
        g[b as usize].push(a as usize);
    }
    let mut color = vec![-1; n + 1];
    for node in 1..n + 1 {
        if color[node] == -1 {
            color[node] = 0;
            let mut q = std::collections::VecDeque::new();
            q.push_back(node);
            while let Some(node) = q.pop_front() {
                for &neighbour in &g[node] {
                    if color[neighbour] == -1 {
                        color[neighbour] = color[node] ^ 1;
                        q.push_back(neighbour);
                    } else if color[neighbour] == color[node] {
                        return false;
                    }
                }
            }
        }
    }
    true
}
