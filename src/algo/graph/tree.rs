pub mod center;
pub mod height;
pub mod height1;
pub mod isomorphism;
pub mod lca;
pub mod rooting;
pub mod rooting1;
pub mod sum;

/// Representation of a tree node, which has an `id` and a `Vec` of `children`.
/// `children` is empty if the node does not have children.
#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a new node without children
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: vec![],
        }
    }
}

pub mod rc {
    pub use std::cell::RefCell;
    pub use std::rc::{Rc, Weak};
    /// Representation of a tree node, which has an `id`, a `Vec` of `children`, as well as a `Weak` reference
    /// to its `parent`
    #[derive(Debug)]
    pub struct Node {
        pub id: usize,
        // A `Weak` reference is required to prevent `Rc` from forming cycles
        // If `Rc` were used, it would cause a stack overflow, for example, when you try to print it.
        pub parent: Option<Weak<RefCell<Node>>>,
        pub children: Vec<Rc<RefCell<Node>>>,
    }

    /// Two nodes are identical if their `id`s equal, have the same children, and have the same parent `id`.
    /// `#[derive(PartialEq, Eq)]` does not work with `Weak` references, so we need manual implementation.
    impl std::cmp::PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            self.id == other.id
                && self.children == other.children
                && match (self.parent.as_ref(), other.parent.as_ref()) {
                    (None, None) => true,
                    (Some(x), Some(y)) => {
                        x.upgrade().unwrap().borrow().id == y.upgrade().unwrap().borrow().id
                    }
                    _ => false,
                }
        }
    }
    impl std::cmp::Eq for Node {}

    impl Node {
        /// Creates a new node either with or without a parent.
        pub fn new(id: usize, parent: Option<&Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Self {
                id,
                parent: parent.map(|parent| Rc::downgrade(&parent)),
                children: vec![],
            }))
        }
        /// Add a child node to a parent node.
        /// First, a `Weak` reference to the parent is created and stored in the child.
        /// Then, a clone is pushed onto the parent's `Vec` of `children`
        pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
            child.borrow_mut().parent = Some(Rc::downgrade(parent));
            parent.borrow_mut().children.push(child.clone());
        }
    }
}
