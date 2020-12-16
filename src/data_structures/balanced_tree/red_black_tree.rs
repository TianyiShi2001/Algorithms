// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug, Copy, Clone)]
// enum Color {
//     Red,
//     Black,
// }

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

// impl<T: Ord + PartialEq + Eq + Clone> Node<T> {
//     fn new(value: T) -> Self {
//         Self {
//             value,
//             color: Color::Red,
//             left: None,
//             right: None,
//             parent: None,
//         }
//     }
//     pub fn append(&mut self, node: Rc<RefCell<Node<T>>>) {

//     }
// }
