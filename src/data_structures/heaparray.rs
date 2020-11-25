//! Dynamically allocated array that mimics:
//!
//! - `int[] arr = new int[size];` in Java
//! - `int *arr = (int *)malloc(size * sizeof(int));` in C

use std::alloc::{alloc, dealloc, Layout};
pub struct HeapArray<T: Clone> {
    ptr: *mut T,
    len: usize,
}

impl<T: Clone> HeapArray<T> {
    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, std::mem::size_of::<T>());
            alloc(layout) as *mut T
        };
        Self { ptr, len }
    }
    pub fn init(len: usize, val: T) -> Self {
        let mut arr = Self::new(len);
        for i in 0..len {
            unsafe {
                *arr.get_unchecked_mut(i) = val.clone();
            }
        }
        arr
    }
    pub fn init_with(len: usize, f: impl Fn(usize) -> T) -> Self {
        let mut arr = Self::new(len);
        for i in 0..len {
            unsafe {
                *arr.get_unchecked_mut(i) = f(i);
            }
        }
        arr
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T: Clone> Drop for HeapArray<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.len, std::mem::size_of::<T>()),
            )
        };
    }
}

impl<T: Clone> std::ops::Deref for HeapArray<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T: Clone> std::ops::DerefMut for HeapArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: Clone> std::ops::Index<usize> for HeapArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T: Clone> std::ops::Index<std::ops::Range<usize>> for HeapArray<T> {
    type Output = [T];
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        unsafe { std::slice::from_raw_parts(self.ptr.add(index.start), index.end - index.start) }
    }
}
impl<T: Clone> std::ops::IndexMut<usize> for HeapArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<T: Clone> std::ops::IndexMut<std::ops::Range<usize>> for HeapArray<T> {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.add(index.start), index.end - index.start)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heaparray() {
        let mut arr = HeapArray::init_with(10, |x| x);
        assert_eq!(arr[1], 1);
        let slc = &mut arr[5..10];
        slc[2] = 100;
        assert_eq!(slc, &[5, 6, 100, 8, 9]);
        println!("{:?}", arr[9]);
    }
}
