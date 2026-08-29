use std::cell::RefCell;
use std::rc::{Rc, Weak};


type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default, Debug)]
struct Node {
    pub data: i64,
    pub next: Option<NodeHandle>,
}

impl Node {
    fn new(data: i64) -> Self {
        Self {
            data: data,
            next: None
        }
    }
}


#[derive(Default, Debug)]
pub struct LinkedList {
    head: Option<NodeHandle>,
    tail: Option<NodeHandle>,
    current_size: usize,
}

impl LinkedList {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, data: i64) {
        let to_add = Node::new(data);
        let to_add = Rc::new(RefCell::new(to_add));

        if self.is_empty() {
            self.head = Some(to_add.clone());
            self.tail = Some(to_add.clone());
            self.current_size = 1;

            return;
        }

        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(to_add.clone());
            self.tail = Some(to_add.clone());
            self.current_size +=1;
        }
    }

}
