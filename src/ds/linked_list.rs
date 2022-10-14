use std::{cell::RefCell, rc::Rc};
pub struct Node<T>
where
    T: Copy,
{
    pub val: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }
}
