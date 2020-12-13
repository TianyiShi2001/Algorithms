// //! A simple but working vector.

// use std::alloc::{alloc, dealloc, realloc, Layout};

// const DEFAULT_CAPACITY: usize = 4;

// pub struct Vector<T> {
//     ptr: *mut T,
//     len: usize,
//     capacity: usize,
// }

// impl<T> Vector<T> {
//     pub fn new() -> Self {
//         let ptr = unsafe {
//             let layout = Self::layout(DEFAULT_CAPACITY);
//             alloc(layout) as *mut T
//         };
//         Self {
//             ptr,
//             len: 0,
//             capacity: DEFAULT_CAPACITY,
//         }
//     }
//     pub fn push(&mut self, v: T) {
//         unsafe {
//             *self.ptr.add(self.len) = v;
//             self.len += 1;
//             if self.len == self.capacity {
//                 self.ptr = realloc(
//                     self.ptr as *mut u8,
//                     Self::layout(self.capacity),
//                     self.capacity * 2,
//                 ) as *mut T;
//                 self.capacity *= 2;
//             }
//         }
//     }
//     pub fn get(&self, idx: usize) -> Option<&T> {
//         if idx < self.len {
//             unsafe { Some(&*(self.ptr.add(idx))) }
//         } else {
//             None
//         }
//     }
//     pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
//         if idx < self.len {
//             unsafe { Some(&mut *(self.ptr.add(idx))) }
//         } else {
//             None
//         }
//     }
//     pub fn pop(&mut self) -> Option<T> {
//         if self.is_empty() {
//             None
//         } else {
//             self.len -= 1;
//             unsafe { Some(std::ptr::read(self.ptr.add(self.len))) }
//         }
//     }
//     pub fn len(&self) -> usize {
//         self.len
//     }
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
//     pub fn capacity(&self) -> usize {
//         self.capacity
//     }
//     unsafe fn layout(size: usize) -> Layout {
//         Layout::from_size_align_unchecked(size, std::mem::size_of::<T>())
//     }
// }

// impl<T> Drop for Vector<T> {
//     fn drop(&mut self) {
//         unsafe { dealloc(self.ptr as *mut u8, Self::layout(self.capacity)) };
//     }
// }

// impl<T> std::ops::Index<usize> for Vector<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         self.get(index).unwrap()
//     }
// }
// impl<T> std::ops::IndexMut<usize> for Vector<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.get_mut(index).unwrap()
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     #[test]
// //     fn test_vector() {
// //         let mut v = Vector::<char>::new();
// //         assert_eq!(v.len(), 0);
// //         assert_eq!(v.capacity(), DEFAULT_CAPACITY);
// //         v.push('1');
// //         assert_eq!(v.len(), 1);
// //         assert_eq!(v.capacity(), DEFAULT_CAPACITY);
// //         assert_eq!(v[0], '1');
// //         v.push('2');
// //         v.push('3');
// //         v.push('4');
// //         v.push('5');
// //         v[4] = 'A';
// //         assert_eq!(v.len(), 5);
// //         assert_eq!(v.capacity(), DEFAULT_CAPACITY * 2);
// //         assert_eq!(v[4], 'A');
// //         let x = v.pop();
// //         assert_eq!(x, Some('A'));
// //         assert_eq!(v.len(), 4);
// //     }
// // }
