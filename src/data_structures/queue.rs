pub trait Queue<T> {
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn push_back(&mut self, val: T);
    fn pop_front(&mut self) -> Option<T>;
}

/// A custom implementation of a circular queue which is
/// extremely quick and lightweight.
/// However, the downside is you need to know an upper bound on the number of elements
/// that will be inside the queue at any given time for this queue to work.
pub struct FixedCapacityQueue<T: Clone> {
    ar: Box<[Option<T>]>,
    front: usize,
    back: usize,
    capacity: usize,
}

impl<T: Clone> FixedCapacityQueue<T> {
    /// Initialize a queue where a maximum of `max_sz` elements can be
    /// in the queue at any given time
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: 0,
            back: 0,
            capacity,
            ar: vec![None; capacity].into_boxed_slice(),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.ar.get(self.front).and_then(|x| x.as_ref())
    }
}

impl<T: Clone> Queue<T> for FixedCapacityQueue<T> {
    fn len(&self) -> usize {
        self.back - self.front
    }
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    fn push_back(&mut self, val: T) {
        assert!(self.back < self.capacity, "Queue too small!");
        self.ar[self.back] = Some(val);
        self.back += 1;
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let res = self.ar[self.front].take();
            self.front += 1;
            res
        }
    }
}

impl<T: Clone> Queue<T> for std::collections::VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    fn push_back(&mut self, val: T) {
        self.push_back(val);
    }
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
}
