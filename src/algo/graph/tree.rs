pub mod center;
pub mod height;
pub mod height1;
pub mod isomorphism;
pub mod lca;
pub mod rooting;
pub mod rooting1;
pub mod sum;

pub mod tree {

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
}
