use std::{cell::RefCell, rc::Rc};

struct Node<T>
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
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

struct Queue<T>
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
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn enqueue(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(tail) = self.tail.as_ref() {
            tail.borrow_mut().next = Some(Rc::clone(&new_node));
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                self.head = head.borrow_mut().next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(head.borrow().val)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.borrow().val),
            None => None,
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
    fn test_enqueue_dequeue_and_peek() {
        let mut q = Queue::new();
        q.enqueue(4);
        assert_eq!(q.dequeue().unwrap(), 4);
        q.enqueue(5);
        q.dequeue();
        assert_eq!(q.dequeue().unwrap_or(10), 10);
        q.enqueue(6);
        assert_eq!(q.peek().unwrap(), 6);
        assert_eq!(q.dequeue().unwrap(), 6);
    }
}
