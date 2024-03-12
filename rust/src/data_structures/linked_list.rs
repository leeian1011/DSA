use std::{borrow::BorrowMut, rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node {
    item: u32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(item: u32, next: Option<Box<Node>>) -> Self {
        Self {
            item,
            next: None
        }
    }

    fn next_mut(&mut self) -> &mut Option<Box<Self>> {
        &mut self.next
    }
    
    fn next(&self) -> &Option<Box<Self>> {
        &self.next
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StackLinkedList {
    head: Option<Box<Node>>,
}

#[allow(dead_code)]
impl StackLinkedList {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn better_pop(&mut self) -> Option<u32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn better_insert(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));

        self.head = Some(new_node);
    }

    pub fn insert(&mut self, value: u32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(value, None)))
        } else {
            let mut next = self.head.as_mut().unwrap().next_mut();
            loop {
                if next.is_none() {
                    break;
                } else {
                    next = next.as_mut().unwrap().next_mut();
                }
            }

            *next = Some(Box::new(Node::new(value, None)));
        }
    }

    pub fn pop_front(&mut self) {
        let head = self.head.borrow_mut();
        if head.is_none() {
            return;
        } else {
            let next = head.as_mut().unwrap().next_mut();
            if next.is_none() {
                *head = None;
                return;
            } else {
                *head = Some(next.take().unwrap())
            }
        }
    }

    pub fn index(&self, index: usize) -> Option<u32> {
        let head = &self.head;
        if head.is_none() {
            return None;
        }

        if index == 0 {
            return Some(head.as_deref().unwrap().item)
        }

        let mut iteration = 1;

        let mut next = head.as_ref().unwrap().next();

        loop {
            if next.is_none() {
                return None;
            }
            if iteration == index {
                return Some(next.as_ref().unwrap().item)
            }

            next = next.as_ref().unwrap().next();
            iteration += 1;
        }
    }
}

pub struct QueueLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>
}

impl QueueLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    pub fn insert(&mut self, item: u32) {
        let mut item_ptr = Box::new(Node::new(item));

        if self.head.is_none() && self.tail.is_none() {
        }
        
        if head_next.is_none() {
            *head_next = Some(item_ptr);
            self.tail = Some(head_next.as_ref().unwrap());
            return;
        } else {
            // item ---head--> curr-head
            // head -> item -> ex-head
            // 
        }
    }

}
