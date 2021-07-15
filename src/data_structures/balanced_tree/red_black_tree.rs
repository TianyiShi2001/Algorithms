use crate::data_structures::bit::BitOpts;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem;

pub trait RbTreeItem = Ord + Debug + PartialEq + Eq + Clone;
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node<T: RbTreeItem> {
    pub value: T,
    pub color: Color,
    // pub left: Link<T>,
    // pub right: Link<T>,
    pub children: [Link<T>; 2],
}

#[derive(Debug)]
pub struct RbTree<T: RbTreeItem> {
    pub root: Link<T>,
    pub len: usize,
}

impl<T: RbTreeItem> RbTree<T> {
    fn new() -> Self {
        Self { root: None, len: 0 }
    }
    fn max_height(&self) -> usize {
        (self.len + 1).log2() * 2
    }
    fn insert(&mut self, v: T) {
        let mh = self.max_height();
        let mut parents: Vec<*mut Box<Node<T>>> = Vec::with_capacity(mh);
        let mut directions = Vec::with_capacity(mh);
        let mut p: *mut Link<T> = &mut self.root;
        // parents.push(p.as_deref().unwrap());
        // directions.push(0u8);
        let mut direction;
        unsafe {
            while let Some(node) = &mut *p {
                direction = (&v > &(*node).value) as u8 as usize;
                p = &mut (*node).children[direction] as *mut Link<T>;
                directions.push(direction);
                parents.push(&mut *node); // TODO: use index
            }
            *p = Some(Node::new(v));
            let mut k = parents.len();
            let mut uncle;
            let mut parent;
            let mut grandparent;
            let mut gp_dir;
            while k >= 3 && (*parents[k - 1]).color == Color::Red {
                gp_dir = directions[k - 2];
                parent = parents[k - 1];
                grandparent = parents[k - 2];
                uncle = (*grandparent).children[1 - gp_dir].as_deref_mut();
                if let Some(unc) = uncle {
                    if unc.color == Color::Red {
                        // Case 1: q's uncle y is red.
                        //         Just re-color parent, grandparent and uncle.
                        (*parent).color = Color::Black;
                        unc.color = Color::Black;
                        (*grandparent).color = Color::Red;
                        k -= 2;
                        continue;
                    }
                }
                // x
                if directions[k - 1] != gp_dir {
                    // Case 3: uncle is black; q, its parent and grandparent forms a triangle
                    Self::rotate(&mut (*parent), gp_dir);
                }
                // Case 2: uncle is black; q, its parent and grandparent form a straight line.
                // Just rotate the grandparent to the 'opposite direction' and re-color
                (*grandparent).color = Color::Red;
                (*parent).color = Color::Black;
                Self::rotate(&mut (*grandparent), 1 - gp_dir);
                break;
                // No need to progress further up because neither the subtree's black-height
                // nor its root's color have changed
            }
        }
        self.root.as_deref_mut().unwrap().color = Color::Black;
        self.len += 1;
    }

    /// Rotate a node.
    /// If direction = 0, rotate left
    /// If direction = 1, rotate right
    fn rotate(node: &mut Box<Node<T>>, direction: usize) {
        let c = node.children[1 - direction].as_mut().unwrap().children[direction].take();
        let y = mem::replace(&mut node.children[1 - direction], c).unwrap();
        let x = mem::replace(node, y);
        node.children[direction] = Some(x);
    }

    /// Traverse the tree to find the height (depth). Used for testing.
    fn height(&self) -> usize {
        fn dfs<T: RbTreeItem>(parent: &Link<T>, depth: usize) -> usize {
            match parent {
                None => depth,
                Some(node) => {
                    let (r, l) = (node.right_as_ref(), node.left_as_ref());
                    std::cmp::max(dfs(l, depth + 1), dfs(r, depth + 1))
                }
            }
        }
        dfs(&self.root, 0)
    }
}

// fn is_red<T: BitOpts>(num: T) -> bool {
//     num.get_bit(std::mem::size_of::<T>() * 8 - 1)
// }
// fn set_red<T: BitOpts>(num: &mut T) {
//     num.set_bit(std::mem::size_of::<T>() * 8 - 1)
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

// impl Color {
//     fn opposite(self) -> Self {
//         match self {
//             Self::Red => Self::Black,
//             Self::Black => Self::Red,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct Node<T: Ord + PartialEq + Eq + Clone> {
//     value: T,
//     color: Color,
//     left: Option<Rc<RefCell<Node<T>>>>,
//     right: Option<Rc<RefCell<Node<T>>>>,
//     parent: Option<Rc<RefCell<Node<T>>>>,
// }

impl<T: RbTreeItem> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Self {
            value,
            color: Color::Red,
            // left: None,
            // right: None,
            children: [None, None],
        })
    }
    pub fn left_as_deref_mut(&mut self) -> Option<&mut Node<T>> {
        self.children[0].as_deref_mut()
    }
    pub fn right_as_deref_mut(&mut self) -> Option<&mut Node<T>> {
        self.children[1].as_deref_mut()
    }
    pub fn left_as_mut(&mut self) -> &mut Link<T> {
        &mut self.children[0]
    }
    pub fn right_as_mut(&mut self) -> &mut Link<T> {
        &mut self.children[1]
    }
    pub fn left_as_ref(&self) -> &Link<T> {
        &self.children[0]
    }
    pub fn right_as_ref(&self) -> &Link<T> {
        &self.children[1]
    }
    // pub fn insert(&mut self, node: Box<Node<T>>) {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn rbt() {
        let mut rbt = RbTree::new();

        for i in 0..1024 {
            rbt.insert(i);
        }
        assert!(rbt.height() <= rbt.max_height());
        let mut rng = thread_rng();
        for _ in 0..1024 {
            rbt.insert(rng.gen_range(-2048..2048));
        }
        assert!(rbt.height() <= rbt.max_height());
        println!("len: {}; height: {}", rbt.len, rbt.height());
    }
}
