use super::Matrix;

#[derive(Debug, PartialEq, Clone)]
pub enum Solution {
    Unique(Vec<f64>),
    Infinite(Vec<f64>, Vec<Vec<f64>>),
    None,
}

impl Solution {
    pub fn unwrap(self) -> Vec<f64> {
        match self {
            Self::Unique(res) => res,
            Self::Infinite(_, _) => panic!("Infinite solutions!"),
            Self::None => panic!("No solutions!"),
        }
    }
}

pub struct Solutions {
    pub augmented: Matrix,
    pub nrows: usize,
    pub ncols: usize,
    pub nullspace_cols: Vec<usize>,
}

impl Solutions {
    pub fn nullspace(&self) -> Vec<Vec<f64>> {
        self.nullspace_cols
            .iter()
            .rev()
            .map(|&j| {
                let mut ns_el = self.augmented.column(j).collect::<Vec<_>>();
                ns_el[j] = -1.;
                ns_el
            })
            .collect()
    }
    pub fn solutions_iter(&self) -> impl Iterator<Item = Solution> + '_ {
        println!("{}", self.augmented);
        println!("{:?}", self.nullspace_cols);
        println!("{:?}", (self.nrows, self.ncols));
        let nullspace = self.nullspace();
        (self.nrows..self.ncols).map(move |j| {
            let sol = self.augmented.column(j).collect();
            if self.nullspace_cols.is_empty() {
                Solution::Unique(sol)
            } else {
                for &j_ in &self.nullspace_cols {
                    if sol[j_] != 0. {
                        return Solution::None;
                    }
                }
                Solution::Infinite(sol, nullspace.clone())
            }
        })
    }
    pub fn solutions(&self) -> Vec<Solution> {
        self.solutions_iter().collect()
    }
    pub fn first(&self) -> Solution {
        self.solutions_iter().next().unwrap()
    }
    pub fn unwrap_first(&self) -> Vec<f64> {
        self.first().unwrap()
    }
    pub fn solutions_matrix(self) -> Option<Matrix> {
        if !self.nullspace_cols.is_empty() {
            None
        } else {
            Some(self.augmented.hsplit(self.nrows).1)
        }
    }
}

pub trait LinearSystemSolver {
    fn solve(augmented: Matrix) -> Solutions;
}
