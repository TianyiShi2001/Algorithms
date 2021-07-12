use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<*mut Node<T>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
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

impl<T> List<T> {
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
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // In the `pop_front` method each head is re-`Box`ed and thus
        // its deallocation is automatically managed by `Box`
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
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
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
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
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

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

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
