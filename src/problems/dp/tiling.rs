pub fn tile(n: i32) -> usize {
    match n {
        i32::MIN..=-1 => 0,
        0 => 1,
        _ => tile(n - 1) + tile(n - 2),
    }
}

pub fn tile_memo(n: usize) -> usize {
    let mut memo = vec![1; n + 1];
    for i in 2..=n {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    memo[n]
}

pub fn tile_custom_sizes(n: usize, sizes: &Vec<usize>) -> usize {
    let mut memo = vec![0; n + 1];
    memo[0] = 1;
    for i in 1..=n {
        memo[i] = sizes
            .iter()
            .fold(0, |sum, &sz| sum + if sz > i { 0 } else { memo[i - sz] });
    }
    return memo[n];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_sizes() {
        let n = 6;
        let sizes = vec![1, 2, 3];
        let res = tile_custom_sizes(n, &sizes);
        println!("{:?}", res);
    }
}
