pub mod center;
pub mod height;
pub mod height1;
pub mod isomorphism;
pub mod lca;
pub mod rooting;
pub mod rooting1;
pub mod sum;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub id: usize,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            children: vec![],
        }
    }
}

pub struct Tree {
    pub root: Node,
    pub size: usize,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        Self { root, size: 1 }
    }
}

pub mod rc {
    pub use std::cell::RefCell;
    pub use std::rc::{Rc, Weak};
    #[derive(Debug)]
    pub struct Node {
        pub id: usize,
        // A `Weak` reference is required to prevent `Rc` from forming cycles
        // If `Rc` were used, it would cause a stack overflow, for example, when you try to print it.
        pub parent: Option<Weak<RefCell<Node>>>,
        pub children: Vec<Rc<RefCell<Node>>>,
    }

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
        pub fn new(id: usize, parent: Option<&Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Self {
                id,
                parent: parent.map(|parent| Rc::downgrade(&parent)),
                children: vec![],
            }))
        }
        pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
            child.borrow_mut().parent = Some(Rc::downgrade(parent));
            parent.borrow_mut().children.push(child.clone());
        }
    }
}
