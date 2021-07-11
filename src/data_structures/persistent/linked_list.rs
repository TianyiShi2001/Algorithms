use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct List<T: Clone> {
    heads: Vec<Link<T>>,
}

// Can use `Rc` if single-threaded
// `lazy_static` used in tests requires `Arc` to be used.
type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    elem: T,
    next: Link<T>,
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        Self { heads: vec![None] }
    }
    pub fn version(&self, version: usize) -> &Link<T> {
        &self.heads[version]
    }
    pub fn num_versions(&self) -> usize {
        self.heads.len()
    }
    pub fn latest(&self) -> &Link<T> {
        &self.heads[self.num_versions() - 1]
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
    /// O(1)
    pub fn push_front(&mut self, elem: T) {
        let new_node = Node {
            elem,
            next: self.latest().clone(),
        };
        self.heads.push(Some(Arc::new(new_node)));
    }
    /// O(n) because the tail is immutable i.e. needs
    /// to copy the entire list with a different tail
    pub fn push_back(&mut self, elem: T) {
        let tail_node = Arc::new(Node { elem, next: None });
        self.extend_back(Some(tail_node));
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let mut new_node = None;
        let res = match self.latest() {
            Some(node) => {
                new_node = node.next.clone();
                Some(node.elem.clone()) // TODO: not clone, return Option<&T>
            }
            None => None,
        };
        self.heads.push(new_node);
        res
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let self_items = self.to_vec_latest();
        let mut res = None;
        let new_link = if self_items.is_empty() {
            None
        } else {
            res = Some(self_items[self_items.len() - 1].clone());
            Self::link_from_slice(&self_items[..self_items.len() - 1], true, None)
        };
        self.heads.push(new_link);
        res
    }
    pub fn extend_front(&mut self, other: &Link<T>) {
        let other = Iter {
            next: other.as_deref(),
        };
        let other_elems: Vec<&T> = other.collect();
        let new = Self::link_from_slice(&other_elems, true, self.latest().clone());
        self.heads.push(new);
    }
    pub fn extend_back(&mut self, other: Link<T>) {
        let self_elems: Vec<&T> = self.iter_latest().collect();
        let new = Self::link_from_slice(self_elems.as_slice(), true, other);
        self.heads.push(new);
    }
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
    pub fn to_vec_latest(&self) -> Vec<&T> {
        self.iter_latest().collect()
    }
    pub fn to_cloned_vec_latest(&self) -> Vec<T> {
        self.iter_latest().cloned().collect()
    }
    pub fn link_from_slice(slc: &[&T], rev: bool, tail: Link<T>) -> Link<T> {
        let it: Box<dyn Iterator<Item = &&T>> = if rev {
            Box::new(slc.into_iter().rev())
        } else {
            Box::new(slc.into_iter())
        };
        let mut link = tail;
        for &e in it {
            link = Some(Arc::new(Node {
                elem: e.clone(),
                next: link,
            }));
        }
        link
    }
}

pub struct Iter<'a, T: Clone> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T: Clone + std::string::ToString> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter_latest()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("<-")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref LIST_835: List<i32> = {
            let mut list = List::new();
            list.push_front(5);
            list.push_front(3);
            list.push_front(8);
            list
        };
        static ref LIST_726: List<i32> = {
            let mut list = List::new();
            list.push_front(2);
            list.push_front(7);
            list.push_back(6);
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

    #[test]
    fn pop() {
        let mut list1 = LIST_835.clone();
        let x = list1.pop_back();
        assert_eq!(x, Some(5));
        let x = list1.pop_front();
        assert_eq!(x, Some(8));
        assert_eq!(list1.to_cloned_vec_latest(), vec![3]);
    }

    #[test]
    fn extend() {
        let mut list1 = LIST_835.clone();
        let mut list2 = LIST_726.clone();
        list1.extend_front(list2.latest());
        assert_eq!(list1.num_versions(), 5);
        assert_eq!(
            list1.iter_latest().cloned().collect::<Vec<_>>(),
            vec![7, 2, 6, 8, 3, 5]
        );
        list2.extend_back(list1.latest().clone());
        assert_eq!(list2.num_versions(), 5);
        assert_eq!(
            list2.iter_latest().cloned().collect::<Vec<_>>(),
            vec![7, 2, 6, 7, 2, 6, 8, 3, 5]
        );
    }
}
