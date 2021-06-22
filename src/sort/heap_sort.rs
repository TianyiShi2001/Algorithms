pub fn heap_sort<T: Ord + std::fmt::Debug>(v: &mut [T]) {
    let n = match v.len() {
        0 | 1 => return,
        n => n,
    };

    // Heapify, converts array into binary heap O(n), see:
    // http://www.cs.umd.edu/~meesh/351/mount/lectures/lect14-heapsort-analysis-part.pdf
    for i in (0..n / 2 - 1).rev() {
        sink(&mut *v, i);
    }

    // Sorting bit
    for k in (1..n).rev() {
        v.swap(0, k);
        sink(&mut v[..k], 0);
    }
}

// Top down node sink, O(log(n))
fn sink<T: Ord>(heap: &mut [T], mut i: usize) {
    let heap_size = heap.len();
    loop {
        let left = 2 * i + 1; // Left  node
        let right = 2 * i + 2; // Right node

        let mut largest = i;
        // Find which is larger than the parent, left or right
        if left < heap_size && heap[left] > heap[largest] {
            largest = left;
        };
        if right < heap_size && heap[right] > heap[largest] {
            largest = right
        }
        if largest != i {
            heap.swap(largest, i);
            i = largest;
        } else {
            // Stop if we cannot sink i anymore, i.e. the max heap is satisfied.
            break;
        }
    }
}
