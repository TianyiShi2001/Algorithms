//! - [Wikipedia](https://www.wikiwand.com/en/Tower_of_Hanoi)

#[derive(Debug)]
pub struct TowerOfHanoi {
    a: Vec<usize>,
    b: Vec<usize>,
    c: Vec<usize>,
    history: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)>,
}

impl TowerOfHanoi {
    fn init(size: usize) -> Self {
        Self {
            a: (1..=size).rev().collect(),
            b: vec![],
            c: vec![],
            history: Vec::new(),
        }
    }

    pub fn solve(size: usize) -> Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> {
        let mut tower = Self::init(size);
        fn _move(
            n: usize,
            tower: *mut TowerOfHanoi,
            source: &mut Vec<usize>,
            target: &mut Vec<usize>,
            auxiliary: &mut Vec<usize>,
        ) {
            if n > 0 {
                // Move n - 1 disks from source to auxiliary, so they are out of the way
                _move(n - 1, tower, source, auxiliary, target);
                // Move the nth disk from source to target
                target.push(source.pop().unwrap());
                // Update progress
                unsafe {
                    (*tower).history.push((
                        (*tower).a.clone(),
                        (*tower).b.clone(),
                        (*tower).c.clone(),
                    ))
                };

                // Move the n - 1 disks that we left on auxiliary onto target
                _move(n - 1, tower, auxiliary, target, source)
            }
        }
        _move(
            size,
            &mut tower as *mut TowerOfHanoi,
            &mut tower.a,
            &mut tower.b,
            &mut tower.c,
        );
        tower.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_tower_of_hanoi() {
        let history = TowerOfHanoi::solve(4);
        assert_eq!(
            &history,
            &[
                (vec![4, 3, 2], vec![], vec![1]),
                (vec![4, 3], vec![2], vec![1]),
                (vec![4, 3], vec![2, 1], vec![]),
                (vec![4], vec![2, 1], vec![3]),
                (vec![4, 1], vec![2], vec![3]),
                (vec![4, 1], vec![], vec![3, 2]),
                (vec![4], vec![], vec![3, 2, 1]),
                (vec![], vec![4], vec![3, 2, 1]),
                (vec![], vec![4, 1], vec![3, 2]),
                (vec![2], vec![4, 1], vec![3]),
                (vec![2, 1], vec![4], vec![3]),
                (vec![2, 1], vec![4, 3], vec![]),
                (vec![2], vec![4, 3], vec![1]),
                (vec![], vec![4, 3, 2], vec![1]),
                (vec![], vec![4, 3, 2, 1], vec![])
            ]
        );
    }
}
