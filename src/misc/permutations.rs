pub struct Permutations<T: Ord> {
    data: Vec<T>,
    first: bool,
}

impl<T: Ord> Iterator for Permutations<T> {
    type Item = *const [T];
    /// Generates the next ordered permutation in-place (skips repeated permutations).
    /// Calling this when the vec is already at the highest permutation returns `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some(&self.data[..])
        } else {
            self.get_first().map(|mut first| {
                let mut to_swap = self.data.len() - 1;
                while self.data[first] >= self.data[to_swap] {
                    to_swap -= 1;
                }
                self.data.swap(first, to_swap);
                first += 1;
                to_swap = self.data.len() - 1;
                while first < to_swap {
                    self.data.swap(first, to_swap);
                    first += 1;
                    to_swap -= 1;
                }
                &self.data[..] as *const [T]
            })
        }
    }
}

impl<T: Ord> Permutations<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, first: true }
    }
    fn get_first(&self) -> Option<usize> {
        for i in (0..self.data.len() - 1).rev() {
            if self.data[i] < self.data[i + 1] {
                return Some(i);
            }
        }
        None
    }
}

pub trait IntoPermutations<T: Ord> {
    fn permutations(self) -> Permutations<T>;
}

impl<T: Ord> IntoPermutations<T> for Vec<T> {
    fn permutations(self) -> Permutations<T> {
        Permutations::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::factorial::Factorial;
    #[test]
    fn test_permutations() {
        for perm in vec![1, 2, 3, 4, 5].permutations() {
            println!("{:?}", unsafe { &*perm });
        }
        assert_eq!(vec![1, 2, 3, 4, 5].permutations().count(), 5.factorial());
    }
}
