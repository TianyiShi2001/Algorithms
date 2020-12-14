pub mod max_flow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub flow: i32,
    pub capacity: i32,
    pub residual: Weak<RefCell<Edge>>,
}
impl Edge {
    pub fn is_residual(&self) -> bool {
        self.capacity == 0
    }
    pub fn reamaining_capacity(&self) -> i32 {
        self.capacity - self.flow
    }
    pub fn augment(&mut self, bottleneck: i32) {
        self.flow += bottleneck;
        self.residual.upgrade().unwrap().borrow_mut().flow -= bottleneck;
    }
}

#[derive(Debug)]
pub struct NetworkFlowAdjacencyList {
    edges: Vec<Vec<Rc<RefCell<Edge>>>>,
}

impl NetworkFlowAdjacencyList {
    /// Initialize an empty adjacency list that can hold up to n nodes.
    pub fn with_size(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        let e1 = Rc::new(RefCell::new(Edge {
            to,
            capacity,
            flow: 0,
            residual: Weak::default(),
        }));
        let e2 = Rc::new(RefCell::new(Edge {
            to: from,
            capacity: 0,
            flow: 0,
            residual: Weak::default(),
        }));
        e1.borrow_mut().residual = Rc::downgrade(&e2);
        e2.borrow_mut().residual = Rc::downgrade(&e1);

        self.edges[from].push(e1);
        self.edges[to].push(e2);
    }
    pub fn from_edges(size: usize, edges: &[(usize, usize, i32)]) -> Self {
        let mut graph = Self::with_size(size);
        for &(a, b, c) in edges.iter() {
            graph.add_edge(a, b, c);
        }
        graph
    }
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, i32, i32)> + '_ {
        self.edges.iter().enumerate().flat_map(|(a, edges)| {
            edges
                .iter()
                .map(move |b| (a, b.borrow().to, b.borrow().flow, b.borrow().capacity))
        })
    }
    pub fn edges_count(&self) -> usize {
        self.edges().count()
    }
    pub fn vertices(&self) -> impl Iterator<Item = (usize, &Vec<Rc<RefCell<Edge>>>)> {
        self.edges.iter().enumerate()
    }
    pub fn vertices_count(&self) -> usize {
        self.edges.len()
    }
}

impl std::ops::Index<usize> for NetworkFlowAdjacencyList {
    type Output = Vec<Rc<RefCell<Edge>>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl std::ops::IndexMut<usize> for NetworkFlowAdjacencyList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}
