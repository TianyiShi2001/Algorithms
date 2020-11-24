use std::alloc::{alloc, dealloc, realloc, Layout};

const DEFAULT_CAPACITY: usize = 4;

pub struct IntVector {
    ptr: *mut i32,
    len: usize,
    capacity: usize,
}

impl IntVector {
    pub fn new() -> Self {
        let ptr = unsafe {
            let layout = Self::layout(DEFAULT_CAPACITY);
            alloc(layout) as *mut i32
        };
        Self {
            ptr,
            len: 0,
            capacity: DEFAULT_CAPACITY,
        }
    }
    pub fn push(&mut self, v: i32) {
        unsafe {
            *self.ptr.add(self.len) = v;
            self.len += 1;
            if self.len == self.capacity {
                self.ptr = realloc(
                    self.ptr as *mut u8,
                    Self::layout(self.capacity),
                    self.capacity * 2,
                ) as *mut i32;
                self.capacity *= 2;
            }
        }
    }
    pub fn get(&self, idx: usize) -> Option<&i32> {
        if idx < self.len {
            unsafe { Some(&*(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn get_mut(&self, idx: usize) -> Option<&mut i32> {
        if idx < self.len {
            unsafe { Some(&mut *(self.ptr.add(idx))) }
        } else {
            None
        }
    }
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let res = Some(self[self.len() - 1]);
            self.len -= 1;
            res
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    unsafe fn layout(size: usize) -> Layout {
        Layout::from_size_align_unchecked(size, 4)
    }
}

impl Drop for IntVector {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr as *mut u8, Self::layout(self.capacity)) };
    }
}

impl std::ops::Index<usize> for IntVector {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl std::ops::IndexMut<usize> for IntVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_int() {
        let mut v = IntVector::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), DEFAULT_CAPACITY);
        v.push(1);
        assert_eq!(v.len(), 1);
        assert_eq!(v.capacity(), DEFAULT_CAPACITY);
        assert_eq!(v[0], 1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
        v[4] = 100;
        assert_eq!(v.len(), 5);
        assert_eq!(v.capacity(), DEFAULT_CAPACITY * 2);
        assert_eq!(v[4], 100);
        let x = v.pop();
        assert_eq!(x, Some(100));
        assert_eq!(v.len(), 4);
    }
}
