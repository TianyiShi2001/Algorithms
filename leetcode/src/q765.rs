/// # Strategy
///
/// - Visit each pair of people in order. If the pair is not a couple, find the partner of the first person
///   of the pair and swap it with the second person of the pair.
pub struct S1;

impl S1 {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let is_couple = |a, b| a == b ^ 1;
        let n = row.len();
        let mut res = 0;
        for i in (0..n).filter(|&i| i % 2 == 0) {
            if !is_couple(row[i], row[i + 1]) {
                res += 1;
                for j in i + 2..n {
                    if is_couple(row[i], row[j]) {
                        row.swap(i + 1, j);
                        break;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q765_1() {
        let inp = vec![0, 2, 1, 3];
        let out = 1;
        assert_eq!(S1::min_swaps_couples(inp), out);
    }

    #[test]
    fn test_q765_2() {
        let inp = vec![3, 2, 0, 1];
        let out = 0;
        assert_eq!(S1::min_swaps_couples(inp), out);
    }

    #[test]
    fn test_q765_3() {
        let inp = vec![0, 2, 1, 3, 5, 6, 11, 12, 8, 9, 4, 10, 7, 13];
        let out = 4;
        assert_eq!(S1::min_swaps_couples(inp), out);
    }
}
