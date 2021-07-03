//! # Resources
//!
//! - [Wikipedia](https://en.wikipedia.org/wiki/Succinct_data_structure)

use num_traits::{PrimInt, Unsigned};
use std::fmt::{Debug, Display};

#[derive(Default)]
pub struct BitMap<T: PrimInt + Unsigned + Debug + Display + Default> {
    inner: Vec<T>,
}

impl<T: PrimInt + Unsigned + Debug + Display + Default> BitMap<T> {
    pub fn new() -> Self {
        BitMap::<T>::default()
    }
}
