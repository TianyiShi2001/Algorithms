//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy all of the following rules:
//!
//! - Each of the digits 1-9 must occur exactly once in each row.
//! - Each of the digits 1-9 must occur exactly once in each column.
//! - Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
//!
//! The '.' character indicates empty cells.
//!
//! # Strategy
//!
//! DFS.
//!
//! # See also
//!
//! - [LeetCode](https://leetcode.com/problems/sudoku-solver/)

pub struct Sudoku {
    inner: [[char; 9]; 9],
}

impl Sudoku {
    pub fn solve_iterative(&mut self) {
        let mut stack = Vec::new();
        let [i, j] = self.next_blank().unwrap();
        for x in '1'..='9' {
            stack.push((i, j, x));
        }
        loop {
            let (i, j, v) = stack.pop().unwrap();

            if v == '.' {
                self.set(i, j, v);
            } else if self.can_set(i, j, v) {
                self.set(i, j, v);
                if let Some([i, j]) = self.next_blank() {
                    // if 1..=9 all fail, remember to empty this cell.
                    stack.push((i, j, '.'));
                    for x in '1'..='9' {
                        stack.push((i, j, x));
                    }
                } else {
                    return;
                }
            }
        }
    }
}

impl Sudoku {
    pub fn solve_recursive(&mut self) -> bool {
        if let Some([i, j]) = self.next_blank() {
            for v in '1'..='9' {
                if self.can_set(i, j, v) {
                    self.set(i, j, v);
                    if self.solve_recursive() {
                        return true;
                    } else {
                        self.erase(i, j);
                    }
                }
            }
        } else {
            return true;
        }
        false
    }
}

impl Sudoku {
    pub fn new(matrix: [[char; 9]; 9]) -> Self {
        Self { inner: matrix }
    }

    fn next_blank(&self) -> Option<[usize; 2]> {
        for i in 0..9 {
            for j in 0..9 {
                if self.inner[i][j] == '.' {
                    return Some([i, j]);
                }
            }
        }
        None
    }
    fn can_set(&self, i: usize, j: usize, n: char) -> bool {
        // check row
        for j_ in 0..9 {
            if self.inner[i][j_] == n {
                return false;
            }
        }
        // check column
        for i_ in 0..9 {
            if self.inner[i_][j] == n {
                return false;
            }
        }
        // check 3x3 grid
        let i1 = i / 3;
        let j1 = j / 3;
        for i2 in 0..3 {
            for j2 in 0..3 {
                if self.inner[i1 * 3 + i2][j1 * 3 + j2] == n {
                    return false;
                }
            }
        }
        true
    }

    fn set(&mut self, i: usize, j: usize, v: char) {
        self.inner[i][j] = v
    }

    fn erase(&mut self, i: usize, j: usize) {
        self.inner[i][j] = '.';
    }
}
use std::fmt;

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(180);
        for i in 0..9 {
            for j in 0..9 {
                s.push_str(&format!("{} ", self.inner[i][j]))
            }
            s.push('\n')
        }
        s.push('\n');
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sudoku() {
        let ans = [
            ['5', '1', '9', '7', '4', '8', '6', '3', '2'],
            ['7', '8', '3', '6', '5', '2', '4', '1', '9'],
            ['4', '2', '6', '1', '3', '9', '8', '7', '5'],
            ['3', '5', '7', '9', '8', '6', '2', '4', '1'],
            ['2', '6', '4', '3', '1', '7', '5', '9', '8'],
            ['1', '9', '8', '5', '2', '4', '3', '6', '7'],
            ['9', '7', '5', '8', '6', '3', '1', '2', '4'],
            ['8', '3', '2', '4', '9', '1', '7', '5', '6'],
            ['6', '4', '1', '2', '7', '5', '9', '8', '3'],
        ];

        let s = [
            ['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            ['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            ['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            ['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            ['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            ['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            ['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ];

        let mut sudoku = Sudoku::new(s);
        sudoku.solve_iterative();
        println!("{}", &sudoku);
        assert_eq!(sudoku.inner, ans);

        let mut sudoku = Sudoku::new(s);
        sudoku.solve_recursive();
        println!("{}", &sudoku);
        assert_eq!(sudoku.inner, ans);

        let ans = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let s = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let mut sudoku = Sudoku::new(s);
        sudoku.solve_iterative();
        println!("{}", &sudoku);
        assert_eq!(sudoku.inner, ans);

        let mut sudoku = Sudoku::new(s);
        sudoku.solve_recursive();
        println!("{}", &sudoku);
        assert_eq!(sudoku.inner, ans);
    }
}
