use std::{cell::RefCell, rc::Rc};

pub struct Node<T>
where
    T: Copy,
{
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

pub struct Queue<T>
where
    T: Copy,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        match &self.tail {
            Some(_) => {
                self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let val = self.head.as_ref()?.borrow().val;
        let next = self.head.as_ref()?.borrow().next.clone();
        self.head = next;
        if self.head.is_none() {
            self.tail = None;
        }
        Some(val)
    }

    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.borrow().val),
            None => None,
        }
    }

    pub fn empty(&self) -> bool {
        match &self.head {
            Some(_) => false,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let mut node = Node::new(1);
        node.next = Some(Rc::new(RefCell::new(Node::new(2))));
        assert_eq!(node.val, 1);
        assert_eq!(node.next.as_ref().unwrap().borrow().val, 2);
        node.next.as_ref().unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(3))));
        assert_eq!(
            node.next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
    }

    #[test]
    fn test_enqueue() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);
        q.enqueue(9);
        assert_eq!(q.peek().unwrap(), 4);
    }

    #[test]
    fn test_dequeue() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);
        q.enqueue(9);
        assert_eq!(q.dequeue().unwrap(), 4);
        assert_eq!(q.dequeue().unwrap(), 5);
        assert_eq!(q.dequeue().unwrap(), 6);
        assert_eq!(q.dequeue().unwrap(), 9);
        assert_eq!(q.dequeue(), None);
        assert_eq!(q.empty(), true);
        q.enqueue(5);
        assert_eq!(q.dequeue().unwrap(), 5);
        assert_eq!(q.dequeue(), None);
        q.enqueue(7);
        q.enqueue(8);
        assert_eq!(q.dequeue().unwrap(), 7);
        assert_eq!(q.dequeue().unwrap(), 8);
        assert_eq!(q.dequeue(), None);
    }
}
