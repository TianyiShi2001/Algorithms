use std::cmp::min;

pub fn edit_distance(s1: &[u8], s2: &[u8]) -> u32 {
    let (m, n) = (s1.len(), s2.len());
    let mut dp_matrix = vec![vec![0u32; n + 1]; m + 1];
    for j in 1..=n {
        dp_matrix[0][j] = j as u32;
    }
    for i in 1..=m {
        dp_matrix[i][0] = i as u32;
    }
    for i in 1..=m {
        for j in 1..=n {
            let diag = dp_matrix[i - 1][j - 1] + if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            let up = dp_matrix[i - 1][j] + 1;
            let left = dp_matrix[i][j - 1] + 1;
            dp_matrix[i][j] = min(diag, min(up, left));
        }
    }
    dp_matrix[m][n]
}

pub fn edit_distance_space_efficient(s1: &[u8], s2: &[u8]) -> u32 {
    let (m, n) = (s1.len(), s2.len());
    let mut dp_matrix: Vec<u32> = Vec::with_capacity(n + 1); // the dynamic programming matrix (only 1 column stored)
    let mut s_diag: u32; // dp_matrix[i - 1][j - 1]
    let mut s_left: u32; // dp_matrix[i][j - 1]
    let mut a: u8; // s1[i - 1]
    let mut b: u8; // s2[j - 1]

    // 0th row
    for j in 0..=(n as u32) {
        dp_matrix.push(j);
    }
    // rows 1 to m
    for i in 1..=m {
        s_diag = (i - 1) as u32;
        s_left = i as u32;
        a = s1[i - 1];
        for j in 1..=n {
            b = s2[j - 1];
            s_left = min(
                s_diag + if a == b { 0 } else { 1 },
                min(s_left + 1, dp_matrix[j] + 1),
            );
            s_diag = dp_matrix[j];
            dp_matrix[j] = s_left;
        }
    }

    dp_matrix[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_edit_distance() {
        let a = b"banana";
        let b = b"Canada";
        assert_eq!(edit_distance(a, b), 2);
        assert_eq!(edit_distance_space_efficient(a, b), 2);
        let a = b"Mississippi";
        let b = b"ssi";
        assert_eq!(edit_distance(a, b), 8);
        assert_eq!(edit_distance_space_efficient(a, b), 8);
    }
}
