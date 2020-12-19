use std::collections::HashSet;

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut servers = HashSet::new();
    let n = grid[0].len();
    let mut first_server_in_column = vec![None; n];
    for (i, row) in grid.iter().enumerate() {
        let mut first_server_in_row = None;
        for (j, &cell) in row.iter().enumerate() {
            if cell == 1 {
                match first_server_in_column[j] {
                    None => first_server_in_column[j] = Some(i),
                    Some(_i) => {
                        servers.insert([_i, j]);
                        servers.insert([i, j]);
                    }
                }
                match first_server_in_row {
                    None => first_server_in_row = Some(j),
                    Some(_j) => {
                        servers.insert([i, _j]);
                        servers.insert([i, j]);
                    }
                }
            }
        }
    }
    servers.len() as i32
}
