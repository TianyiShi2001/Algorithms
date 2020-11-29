// https://www.youtube.com/watch?v=QOJ-CmQiXko&list=PLDV1Zeh2NRsB6SWUrDFW2RmDotAfPbeHu&index=16

use super::PriorityQueue;

struct BinaryHeap<T: PartialOrd> {
    heap: Vec<T>,
}
impl<T: PartialOrd> PriorityQueue<T> for BinaryHeap<T> {
    /// Adds an element to the priority queue, O(log(n))
    fn insert(&mut self, el: T) {
        self.heap.push(el);
        self.swim(self.heap.len() - 1);
    }
    /// Test if an element is in heap, O(n)
    fn contains(&self, el: &T) -> bool {
        self.heap.contains(el)
    }
    /// Removes a particular element in the heap, O(n)
    fn remove(&mut self, el: &T) {
        if let Some(idx) = self.heap.iter().position(|x| x == el) {
            self.remove_at(idx);
        }
    }
    /// Removes the root of the heap, O(log(n))
    fn poll(&mut self) -> Option<T> {
        self.remove_at(0)
    }
}

impl<T: PartialOrd> BinaryHeap<T> {
    pub fn with_capacity(sz: usize) -> Self {
        Self {
            heap: Vec::with_capacity(sz),
        }
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    /// Removes a node at particular index, O(log(n))
    fn remove_at(&mut self, i: usize) -> Option<T> {
        let end = self.heap.len() - 1;
        self.heap.swap(i, end);
        let item = self.heap.pop();
        self.sink(i);
        item
    }

    /// Perform bottom up node swim, O(log(n))
    fn swim(&mut self, mut k: usize) {
        // Grab the index of the next parent node WRT to k
        let mut parent = (k - 1) / 2;

        // Keep swimming while we have not reached the
        // root and while we're less than our parent.
        while k > 0 && self.heap[k] < self.heap[parent] {
            // Exchange k with the parent
            self.heap.swap(parent, k);
            k = parent;

            // Grab the index of the next parent node WRT to k
            parent = (k - 1) / 2;
        }
    }

    // Top down node sink, O(log(n))
    fn sink(&mut self, mut k: usize) {
        let heap_size = self.heap.len();
        loop {
            let left = 2 * k + 1; // Left  node
            let right = 2 * k + 2; // Right node

            // Find which is smaller left or right
            let smallest = if right < heap_size && self.heap[right] < self.heap[left] {
                right
            } else {
                left
            };

            // Stop if we're outside the bounds of the tree
            // or stop early if we cannot sink k anymore
            if left >= heap_size || self.heap[k] < self.heap[smallest] {
                break;
            }

            // Move down the tree following the smallest node
            self.heap.swap(smallest, k);
            k = smallest;
        }
    }
}
