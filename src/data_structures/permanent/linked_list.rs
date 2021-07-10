use std::sync::Arc;

pub struct PermanentList<T> {
    heads: Vec<Link<T>>,
}

// Can use `Rc` if single-threaded
// `lazy_static` used in tests requires `Arc` to be used.
type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> PermanentList<T> {
    pub fn new() -> Self {
        Self { heads: vec![None] }
    }
    pub fn version(&self, version: usize) -> &Link<T> {
        &self.heads[version]
    }
    pub fn latest(&self) -> &Link<T> {
        &self.heads[self.heads.len() - 1]
    }
    pub fn tail(&self, version: usize) -> &Link<T> {
        let mut link = self.version(version);
        let mut prev = link;
        while let Some(node) = link {
            prev = link;
            link = &node.next;
        }
        prev
    }
    pub fn tail_latest(&self) -> &Link<T> {
        self.tail(self.heads.len() - 1)
    }
    pub fn push_front(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.latest().clone(),
        };
        self.heads.push(Some(Arc::new(new_node)));
    }
    pub fn extend_front(&mut self, list: Link<T>) {}
    pub fn iter(&self, version: usize) -> Iter<'_, T> {
        Iter {
            next: self.version(version).as_deref(),
        }
    }
    pub fn iter_latest(&self) -> Iter<'_, T> {
        Iter {
            next: self.latest().as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref LIST_835: PermanentList<i32> = {
            let mut list = PermanentList::new();
            list.push_front(5);
            list.push_front(3);
            list.push_front(8);
            list
        };
        static ref LIST_726: PermanentList<i32> = {
            let mut list = PermanentList::new();
            list.push_front(6);
            list.push_front(2);
            list.push_front(7);
            list
        };
    }

    #[test]
    fn iter() {
        let mut iter0 = LIST_835.iter(0);
        assert_eq!(iter0.next(), None);
        let mut iter1 = LIST_835.iter(1);
        assert_eq!(iter1.next(), Some(&5));
        let mut iter2 = LIST_835.iter(2);
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&5));
        let mut iter3 = LIST_835.iter_latest();
        assert_eq!(iter3.next(), Some(&8));
        assert_eq!(iter3.next(), Some(&3));
        assert_eq!(iter3.next(), Some(&5));
    }

    #[test]
    fn head_tail() {
        assert!(LIST_835.tail(0).is_none());
        assert_eq!(LIST_835.tail(1).as_deref().unwrap().elem, 5);
        assert_eq!(LIST_835.version(1).as_deref().unwrap().elem, 5);
        assert_eq!(LIST_835.tail(2).as_deref().unwrap().elem, 5);
        assert_eq!(LIST_835.version(2).as_deref().unwrap().elem, 3);
        assert_eq!(LIST_835.tail(3).as_deref().unwrap().elem, 5);
        assert_eq!(LIST_835.version(3).as_deref().unwrap().elem, 8);
    }
}
