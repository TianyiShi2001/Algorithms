//! The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
//!
//! Given an integer n, return all distinct solutions to the n-queens puzzle.
//!
//! Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
//!
//! # See also
//!
//! - [Leetcode](https://leetcode.com/problems/n-queens/)

use std::collections::HashSet;

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    Board::new(n as usize).solve()
}

struct Board {
    matrix: Vec<Vec<char>>,
    n: usize,
    solutions: HashSet<Vec<String>>,
}

impl Board {
    pub fn new(n: usize) -> Self {
        Self {
            matrix: vec![vec!['.'; n]; n],
            n,
            solutions: HashSet::new(),
        }
    }
    pub fn solve(mut self) -> Vec<Vec<String>> {
        self._solve(0, 0);
        self.solutions.into_iter().collect()
    }
    fn _solve(&mut self, i: usize, count: usize) {
        if count == self.n {
            self.add_solution();
        } else if i == self.n {
        } else {
            for col in 0..self.n {
                if self.safe(i, col) {
                    self.matrix[i][col] = 'Q';
                    self._solve(i + 1, count + 1);
                    self.matrix[i][col] = '.';
                }
            }
        }
    }
    fn add_solution(&mut self) {
        self.solutions.insert(
            self.matrix
                .iter()
                .map(|x| x.iter().copied().collect())
                .collect(),
        );
    }

    fn safe(&self, i: usize, j: usize) -> bool {
        for j_ in 0..self.n {
            if self.matrix[i][j_] == 'Q' {
                return false;
            }
        }
        for i_ in 0..self.n {
            if self.matrix[i_][j] == 'Q' {
                return false;
            }
        }
        let (mut i_, mut j_) = (i + 1, j + 1);
        while i_ > 0 && j_ > 0 {
            i_ -= 1;
            j_ -= 1;
            if self.matrix[i_][j_] == 'Q' {
                return false;
            }
        }
        let (mut i_, mut j_) = (i, j + 1);
        while i_ < self.n && j_ > 0 {
            j_ -= 1;
            if self.matrix[i_][j_] == 'Q' {
                return false;
            }
            i_ += 1;
        }
        let (mut i_, mut j_) = (i, j);
        while i_ < self.n && j_ < self.n {
            if self.matrix[i_][j_] == 'Q' {
                return false;
            }
            i_ += 1;
            j_ += 1;
        }
        let (mut i_, mut j_) = (i + 1, j);
        while i_ > 0 && j_ < self.n {
            i_ -= 1;
            if self.matrix[i_][j_] == 'Q' {
                return false;
            }
            j_ += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_n_queens() {
        let n1 = solve_n_queens(1);
        assert_eq!(n1, vec![vec!["Q".to_string()]]);

        let n2 = solve_n_queens(2);
        assert!(n2.is_empty());

        let n3 = solve_n_queens(3);
        assert!(n3.is_empty());

        let mut n4 = solve_n_queens(4);
        n4.sort();
        assert_eq!(
            n4,
            make_solution(&[
                &["..Q.", "Q...", "...Q", ".Q.."],
                &[".Q..", "...Q", "Q...", "..Q."],
            ])
        );

        let mut n5 = solve_n_queens(5);
        let mut n5_expected = make_solution(&[
            &["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
            &["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
            &["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
            &["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
            &[".Q...", "....Q", "..Q..", "Q....", "...Q."],
            &["....Q", "..Q..", "Q....", "...Q.", ".Q..."],
            &[".Q...", "...Q.", "Q....", "..Q..", "....Q"],
            &["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
            &["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
            &["Q....", "..Q..", "....Q", ".Q...", "...Q."],
        ]);
        n5.sort();
        n5_expected.sort();
        assert_eq!(n5, n5_expected);

        let n8 = solve_n_queens(8);
        assert_eq!(n8.len(), 92);
    }

    fn make_solution(sol: &[&[&'static str]]) -> Vec<Vec<String>> {
        sol.iter()
            .map(|&x| x.iter().map(|&s| s.to_owned()).collect())
            .collect()
    }
}
