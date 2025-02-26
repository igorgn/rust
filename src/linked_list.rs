use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
pub struct LinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    pub current: Option<Rc<RefCell<Node>>>,
}

#[derive(PartialEq)]
pub struct Node {
    pub data: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32, next: Option<Rc<RefCell<Node>>>) -> Self {
        Self { data, next }
    }
}
impl Deref for Node {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            current: None,
        }
    }

    pub fn insert_start(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(data, self.head.clone())));
        self.head = Some(Rc::clone(&new_node));
        if self.tail.is_none() {
            self.tail = Some(Rc::clone(&new_node))
        }
    }
    pub fn insert_end(&mut self, data: i32) {
        // if no head, so no tail as well
        if self.tail.is_none() {
            self.insert_start(data);
            return;
        }
        let new_node = Rc::new(RefCell::new(Node::new(data, None)));
        self.tail.as_mut().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
        self.tail = Some(Rc::clone(&new_node));
    }
    // pub fn insert_nth(&mut self, data: i32, position: usize) {
    //     todo!()
    // }

    pub fn search(&mut self, s: i32) -> Option<Rc<RefCell<Node>>> {
        for node in self.into_iter() {
            if node.borrow().data == s {
                return Some(Rc::clone(&node))
            }
        }
        None
    }
}

impl Iterator for LinkedList {
    type Item = Rc<RefCell<Node>>;

    //// AI generated (not working)
    // fn next(&mut self) -> Option<Self::Item> {
        // match self.iterator_current.take() {
        //     Some(current) => {
        //         self.iterator_current = current.borrow().next.clone();
        //         Some(current)
        //     }
        //     None => {
        //         if self.iterator_current.is_none() && self.head.is_some() {
        //             self.iterator_current = self.head.clone();
        //             self.head.clone()
        //         } else {
        //             None
        //         }
        //     }
        // }
    // }
    
    ////Manual (works correctly)
    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(current) => {
                self.current = current.borrow().next.clone();
                self.current.clone()
            }
            None => {
                self.current = self.head.clone();
                self.current.clone()
            }
        }
    }
}

