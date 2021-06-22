use num_traits::PrimInt;

pub(crate) const EPS: f64 = 1e-6;

pub trait MinMax<'a, T>: Sized + Iterator<Item = &'a T>
where
    T: 'a + PrimInt,
{
    fn min_max(self) -> (T, T) {
        let mut min = T::max_value();
        let mut max = T::min_value();
        for &n in self {
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
        }
        (min, max)
    }
}

impl<'a, I, T> MinMax<'a, T> for I
where
    I: Sized + Iterator<Item = &'a T>,
    T: 'a + PrimInt,
{
}
