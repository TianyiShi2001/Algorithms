pub mod binary_heap;

pub trait PriorityQueue<T: PartialOrd> {
    fn with_capacity(sz: usize) -> Self;
    fn insert(&mut self, el: T);
    fn contains(&self, el: &T) -> bool;
    fn remove(&mut self, el: &T);
    fn poll(&mut self) -> Option<T>;
}
