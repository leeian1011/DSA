use std::borrow::BorrowMut;

#[derive(Debug)]
struct Node {
    item: u32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(item: u32) -> Self {
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
pub struct LinkedList {
    head: Option<Box<Node>>,
}

#[allow(dead_code)]
impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn insert(&mut self, value: u32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(value)))
        } else {
            let mut next = self.head.as_mut().unwrap().next_mut();
            loop {
                if next.is_none() {
                    break;
                } else {
                    next = next.as_mut().unwrap().next_mut();
                }
            }

            *next = Some(Box::new(Node::new(value)));
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
