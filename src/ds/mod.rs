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

pub struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { heap: vec![] }
    }
    pub fn insert(&mut self, el: i32) {
        self.heap.push(el);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn get_min(&self) -> i32 {
        self.heap[0]
    }

    pub fn extract_min(&mut self) -> i32 {
        let el = self.heap[0];
        let len = self.heap.len();
        self.heap.swap(0, len - 1);
        self.heap.pop();
        self.sift_down(0);
        el
    }

    pub fn change_priority(&mut self, i: usize, new_p: i32) {
        let old_p = self.heap[i];
        self.heap[i] = new_p;
        if new_p < old_p {
            self.sift_up(i);
        } else {
            self.sift_down(i);
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 && self.heap[self.parent(i)] > self.heap[i] {
            let parent = self.parent(i);
            self.heap.swap(parent, i);
            i = parent;
        }
    }

    fn sift_down(&mut self, i: usize) {
        let mut min_index = i;
        let left = self.left_child(i);
        if left < self.heap.len() && self.heap[left] < self.heap[min_index] {
            min_index = left;
        }
        let right = self.right_child(i);
        if right < self.heap.len() && self.heap[right] < self.heap[min_index] {
            min_index = right;
        }
        if i != min_index {
            self.heap.swap(i, min_index);
            self.sift_down(min_index);
        }
    }

    fn left_child(&self, i: usize) -> usize {
        2 * i + 1
    }
    fn right_child(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
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

    #[test]
    fn test_insert_priority_queue() {
        let mut p = MinHeap::new();
        p.insert(5);
        assert_eq!(p.get_min(), 5);
        p.insert(9);
        assert_eq!(p.get_min(), 5);
        p.insert(3);
        assert_eq!(p.get_min(), 3);
        p.insert(98);
        assert_eq!(p.get_min(), 3);
        p.insert(1);
        assert_eq!(p.get_min(), 1);
    }

    #[test]
    fn test_extract_min_priority_queue() {
        let mut p = MinHeap::new();
        p.insert(5);
        p.insert(9);
        p.insert(3);
        p.insert(98);
        p.insert(1);
        assert_eq!(p.extract_min(), 1);
        assert_eq!(p.extract_min(), 3);
        assert_eq!(p.extract_min(), 5);
        assert_eq!(p.extract_min(), 9);
        assert_eq!(p.extract_min(), 98);
    }

    #[test]
    fn test_change_priority_priority_queue() {
        let mut p = MinHeap::new();
        p.insert(5);
        p.insert(9);
        p.insert(3);
        p.insert(98);
        p.insert(1);
        p.change_priority(0, 100);
        assert_eq!(p.extract_min(), 3);
        assert_eq!(p.extract_min(), 5);
    }
}
