use std::collections::HashMap;

pub trait Prioritiness {
    fn id(&self) -> usize;
    fn priority(&self) -> i32;
    fn change_priority(&mut self, p: i32);
}

pub struct MinHeap<T>
where
    T: Prioritiness + Copy,
{
    pub heap: Vec<T>,
    internal_map: HashMap<usize, usize>, // id -> index
}

impl<T: Prioritiness + Copy> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: vec![],
            internal_map: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn insert(&mut self, el: T) {
        self.heap.push(el);
        self.internal_map.insert(el.id(), self.heap.len() - 1);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn get_min(&self) -> T {
        self.heap[0]
    }

    pub fn extract_min(&mut self) -> T {
        let el = self.heap[0];
        self.internal_map.remove(&el.id());
        let len = self.heap.len();
        self.heap.swap(0, len - 1);
        self.heap.pop();
        if !self.heap.is_empty() {
            self.internal_map.insert(self.heap[0].id(), 0);
            self.sift_down(0);
        }
        el
    }

    pub fn change_priority(&mut self, i: usize, new_p: i32) {
        let index = self.internal_map[&i];
        let old_p = self.heap[index].priority();
        self.heap[index].change_priority(new_p);
        if new_p < old_p {
            self.sift_up(index);
        } else {
            self.sift_down(index);
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 && self.heap[self.parent(i)].priority() > self.heap[i].priority() {
            let parent = self.parent(i);

            self.internal_map.insert(self.heap[i].id(), parent);
            self.internal_map.insert(self.heap[parent].id(), i);

            self.heap.swap(parent, i);
            i = parent;
        }
    }

    fn sift_down(&mut self, i: usize) {
        let mut min_index = i;
        let left = self.left_child(i);
        if left < self.heap.len() && self.heap[left].priority() < self.heap[min_index].priority() {
            min_index = left;
        }
        let right = self.right_child(i);
        if right < self.heap.len() && self.heap[right].priority() < self.heap[min_index].priority()
        {
            min_index = right;
        }
        if i != min_index {
            self.internal_map.insert(self.heap[i].id(), min_index);
            self.internal_map.insert(self.heap[min_index].id(), i);
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

    #[derive(Copy, Clone)]
    struct Example {
        priority: i32,
        id: usize,
    }

    impl Prioritiness for Example {
        fn priority(&self) -> i32 {
            self.priority
        }
        fn change_priority(&mut self, p: i32) {
            self.priority = p;
        }
        fn id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn test_priority_queue() {
        let mut p: MinHeap<Example> = MinHeap::new();
        p.insert(Example { priority: 5, id: 3 });
        p.insert(Example { priority: 3, id: 4 });
        p.insert(Example { priority: 4, id: 5 });
        p.insert(Example { priority: 1, id: 6 });

        assert_eq!(p.get_min().priority, 1);
        p.change_priority(6, -4);
        assert_eq!(p.get_min().priority, -4);
        assert_eq!(p.get_min().id(), 6);
        p.change_priority(6, 10);
        assert_eq!(p.get_min().priority, 3);
        assert_eq!(p.get_min().id(), 4);
        p.extract_min();
        p.change_priority(5, -5);
        assert_eq!(p.get_min().priority, -5);
        assert_eq!(p.get_min().id(), 5);
    }

    #[test]
    fn test_priority_queue2() {
        let mut p: MinHeap<Example> = MinHeap::new();
        p.insert(Example {
            priority: 10,
            id: 0,
        });
        p.insert(Example {
            priority: 20,
            id: 1,
        });
        assert_eq!(p.get_min().priority, 10);
        p.extract_min();
        assert_eq!(p.get_min().priority, 20);
        p.insert(Example {
            priority: 30,
            id: 2,
        });
        p.change_priority(2, 0);
        assert_eq!(p.get_min().priority, 0);
        assert_eq!(p.get_min().id(), 2);
    }

    #[test]
    fn test_priority_queue3() {
        let mut p: MinHeap<Example> = MinHeap::new();
        p.insert(Example {
            priority: 10,
            id: 0,
        });
        p.insert(Example {
            priority: 20,
            id: 1,
        });
        p.extract_min();
        p.extract_min();
    }
}
