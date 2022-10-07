struct Node<T>
where
    T: Copy,
{
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Copy,
{
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let mut node = Node::new(1);
        node.next = Some(Box::new(Node::new(2)));
        assert_eq!(node.val, 1);
        assert_eq!(node.next.as_ref().unwrap().val, 2);
        node.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
        assert_eq!(node.next.as_ref().unwrap().next.as_ref().unwrap().val, 3);
    }

    #[test]
    fn test_iterate_through_nodes_example() {
        let mut node = Node::new(1);
        node.next = Some(Box::new(Node::new(2)));
        node.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
        let mut node = Some(Box::new(node));
        while let Some(n) = node {
            println!("{}", n.val);
            node = n.next;
        }
    }
}
