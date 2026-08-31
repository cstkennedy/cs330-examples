use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::{Rc, Weak};

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default, Debug)]
pub(crate) struct Node {
    pub data: i64,
    pub next: Option<NodeHandle>,
}

impl Node {
    fn new(data: i64) -> Self {
        Self {
            data: data,
            next: None,
        }
    }
}

pub struct LLIterator {
    pub(crate) current_focus: Option<NodeHandle>,
}


impl Iterator for LLIterator {
    // This should really be a borrow (i.e., &i64)
    type Item = i64;

    // Gemini was used for this function (next)... This impl block with three (3) prompts
    //
    // Prompt 1...
    //
    // rust impl linked list in safe rust
    //
    // Prompt 2...
    //
    // What if i changed to RC<RefCell
    //
    // Prompt 3...
    //
    // How would I impl an Iterator?
    //
    fn next(&mut self) -> Option<Self::Item> {
        self.current_focus.take().map(|node| {
            self.current_focus = node.borrow().next.clone();

            node.borrow().data
        })
    }
}

#[derive(Default, Debug)]
pub struct LinkedList {
    head: Option<NodeHandle>,
    tail: Option<NodeHandle>,
    current_size: usize,
}

impl LinkedList {
    pub fn size(&self) -> usize {
        self.current_size
    }

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
            self.current_size += 1;
        }
    }

    pub fn iter(&self) -> LLIterator {
        LLIterator {
            current_focus: self.head.clone(),
        }
    }
}

impl PartialEq for LinkedList {
    fn eq(&self, rhs: &Self) -> bool {
        if self.size() != rhs.size() {
            return false;
        }

        for (lhs_data, rhs_data) in self.iter().zip(rhs.iter()) {
            if lhs_data != rhs_data {
                return false;
            }
        }

        true
    }
}

impl Eq for LinkedList {}

impl Display for LinkedList {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, entry) in self.iter().enumerate() {
            writeln!(f, "Node # {idx:4} - {entry:4}")?;
        }

        Ok(())
    }
}
