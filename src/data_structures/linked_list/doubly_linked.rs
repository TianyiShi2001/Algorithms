use std::mem;

#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<*mut Node<T>>;

#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T: std::fmt::Debug> Node<T> {
    fn new(elem: T) -> *mut Self {
        // Heap allocation using `Box`
        let node = box Node {
            elem,
            prev: None,
            next: None,
        };
        Box::into_raw(node)
    }
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        unsafe {
            match self.head.take() {
                Some(old_head) => {
                    (*old_head).prev = Some(new_head);
                    (*new_head).next = Some(old_head);
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = Some(new_head);
                    self.head = Some(new_head);
                }
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        unsafe {
            match self.tail.take() {
                Some(old_tail) => {
                    (*old_tail).next = Some(new_tail);
                    (*new_tail).prev = Some(old_tail);
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = Some(new_tail);
                    self.tail = Some(new_tail);
                }
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.tail.take().map(|old_tail| {
                match (*old_tail).prev.take() {
                    Some(new_tail) => {
                        (*new_tail).next.take();
                        // i.e. converting the reference to the popped element to None
                        self.tail = Some(new_tail);
                    }
                    None => {
                        // if the element being popped is the only element,
                        // also empty the head.
                        self.head.take();
                    }
                }
                // Reconstructing the `Box` allows the allocated heap to be properly freed.
                let old_tail = Box::from_raw(old_tail);
                Box::into_inner(old_tail).elem
            })
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.take().map(|old_head| {
                match (*old_head).next.take() {
                    Some(new_head) => {
                        (*new_head).prev.take();
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                let old_head = Box::from_raw(old_head);
                Box::into_inner(old_head).elem
            })
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| unsafe { &(**node).elem })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| unsafe { &(**node).elem })
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_ref().map(|node| unsafe { &mut (**node).elem })
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_ref().map(|node| unsafe { &mut (**node).elem })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(&self.head, &self.tail)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut list = Self::new();
        list.extend_front_from_vec(v);
        list
    }
    pub fn extend_back(&mut self, other: Self) {
        match self.tail {
            None => *self = other,
            Some(old_tail) => unsafe {
                other.head.map(|node| (*node).prev = Some(old_tail));
                (*old_tail).next = other.head;
                self.tail = other.tail;
                // The new, extended list is responsible for freeing the "other" part.
                // If we retain `other` as-is, when it goes out of scope at the end of
                // this block the entire content of the list will be freed. Thus, a
                // simple work-around is to set `other.tail` and `other.head` to `None`
                other.assume_empty();
            },
        }
    }
    pub fn extend_front(&mut self, other: Self) {
        match self.head {
            None => *self = other,
            Some(old_head) => unsafe {
                other.tail.map(|node| (*node).prev = Some(old_head));
                (*old_head).prev = other.tail;
                self.head = other.head;
                other.assume_empty();
            },
        }
    }
    pub fn extend_front_from_vec(&mut self, mut v: Vec<T>) {
        while let Some(elem) = v.pop() {
            self.push_front(elem);
        }
    }
    pub fn extend_back_from_vec(&mut self, mut v: Vec<T>) {
        let mut right = Self::new();
        while let Some(elem) = v.pop() {
            right.push_front(elem);
        }
        self.extend_back(right);
    }

    /// Empty the head and tail without actually freeing
    /// the content of the list
    fn assume_empty(mut self) {
        self.head = None;
        self.tail = None;
    }
}

impl<T: std::fmt::Debug> Drop for List<T> {
    fn drop(&mut self) {
        // In the `pop_front` method each head is re-`Box`ed and thus
        // its deallocation is automatically managed by `Box`
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T: std::fmt::Debug>(List<T>);

impl<T: std::fmt::Debug> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T: std::fmt::Debug> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

pub struct Iter<'a, T: std::fmt::Debug>(&'a Link<T>, &'a Link<T>);

impl<'a, T: std::fmt::Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0.map(|node| unsafe {
            self.0 = &(*node).next;
            &(*node).elem
        });
        // if self.0 == self.1 {
        //     self.0 = &None;
        //     self.1 = &None;
        // }
        res
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.extend_front_from_vec(vec![3, 2, 1]);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        // list.extend_front_from_vec(vec![5, 4]);
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.extend_back_from_vec(vec![1, 2, 3]);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.extend_back_from_vec(vec![4, 5]);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.extend_front_from_vec(vec![1, 2, 3]);

        assert_eq!(&*list.peek_front().unwrap(), &1);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 1);
        assert_eq!(&*list.peek_back().unwrap(), &3);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 3);
    }

    #[test]
    fn into_iter() {
        let list = List::from_vec(vec![1, 2, 3]);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let list = List::from_vec(vec![1, 2, 3]);
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&1, &2, &3]);
        // let mut iter = list.iter();
        // assert_eq!(iter.next(), Some(&1));
        // assert_eq!(iter.next(), Some(&2));
    }
}
