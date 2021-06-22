// https://www.youtube.com/watch?v=QOJ-CmQiXko&list=PLDV1Zeh2NRsB6SWUrDFW2RmDotAfPbeHu&index=16

use super::PriorityQueue;

pub struct BinaryHeap<T: PartialOrd> {
    heap: Vec<T>,
}
impl<T: PartialOrd> PriorityQueue<T> for BinaryHeap<T> {
    fn with_capacity(sz: usize) -> Self {
        Self {
            heap: Vec::with_capacity(sz),
        }
    }
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
    pub fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    /// Removes a node at particular index, O(log(n))
    fn remove_at(&mut self, i: usize) -> Option<T> {
        let end = self.heap.len() - 1;
        self.heap.swap(i, end);
        let removed = self.heap.pop();
        // Try sinking element
        let i_ = self.sink(i);
        // If sinking did not work try swimming
        if i_ == i {
            self.swim(i);
        }
        removed
    }

    /// Perform bottom up node swim, O(log(n))
    fn swim(&mut self, mut k: usize) -> usize {
        // Grab the index of the next parent node WRT to k
        let mut parent = (k.saturating_sub(1)) / 2;

        // Keep swimming while we have not reached the
        // root and while we're less than our parent.
        while k > 0 && self.heap[k] < self.heap[parent] {
            // Exchange k with the parent
            self.heap.swap(parent, k);
            k = parent;

            // Grab the index of the next parent node WRT to k
            parent = (k.saturating_sub(1)) / 2;
        }
        k
    }

    // Top down node sink, O(log(n))
    fn sink(&mut self, mut k: usize) -> usize {
        let heap_size = self.heap.len();
        loop {
            let left = 2 * k + 1; // Left  node
            let right = 2 * k + 2; // Right node

            let mut smallest = k;
            // Find which is smaller than the parent, left or right
            if left < heap_size && self.heap[left] < self.heap[smallest] {
                smallest = left;
            };
            if right < heap_size && self.heap[right] < self.heap[smallest] {
                smallest = right
            }
            if smallest != k {
                self.heap.swap(smallest, k);
                k = smallest;
            } else {
                // Stop if we cannot sink k anymore
                break;
            }
        }
        k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_priority_queue_binary_heap() {
        let mut pq = BinaryHeap::with_capacity(8);
        pq.insert(5);
        pq.insert(7);
        pq.insert(3);
        pq.insert(8);
        pq.insert(2);
        pq.insert(1);
        assert_eq!(pq.poll().unwrap(), 1);
        pq.remove(&2);
        assert_eq!(pq.poll().unwrap(), 3);
    }
}
