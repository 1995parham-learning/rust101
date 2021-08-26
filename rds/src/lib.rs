pub trait Heap<T, P: PartialOrd> {
    fn top(&self) -> &T;
    fn peek(&mut self) -> T;
    fn insert(&mut self, element: T, priority: P);
}

pub struct DHeap<T, P: PartialOrd> {
    pairs: std::vec::Vec<(T, P)>,
    d: usize,
}

impl<T, P: PartialOrd> DHeap<T, P> {
    pub fn new_2heap() -> DHeap<T, P> {
        DHeap::new(2)
    }

    pub fn new(d: usize) -> DHeap<T, P> {
        DHeap {
            pairs: std::vec::Vec::new(),
            d,
        }
    }

    fn highest_priority_child(&self, index: usize) -> usize {
        let mut highest_priority_child = self.d * index + 1;
        for i in 1..self.d {
            let child_index = self.d * index + i + 1;
            match self.pairs.get(child_index) {
                Some(child) => {
                    if child.1 > self.pairs[highest_priority_child].1 {
                        highest_priority_child = index;
                    }
                }
                None => break,
            }
        }

        highest_priority_child
    }

    fn first_leaf_index(&self) -> usize {
        match self.pairs.len() {
            0 => 0,
            1 => 0,
            _ => (self.pairs.len() - 2) / self.d + 1,
        }
    }

    fn push_down(&mut self, index: usize) {
        let mut current_index = index;

        while current_index < self.first_leaf_index() {
            let child_index = self.highest_priority_child(current_index);

            if self.pairs[child_index].1 > self.pairs[current_index].1 {
                self.pairs.swap(current_index, child_index);
                current_index = child_index;
            } else {
                break;
            }
        }
    }

    // bubble_up implements a heapify to maintain the max heap property.
    // it moves an element from the button of heap to its position.
    fn bubble_up(&mut self, index: usize) {
        let mut parent_index = index;
        while parent_index > 0 {
            let current_index = parent_index;
            parent_index = (parent_index - 1) / self.d;

            if self.pairs[parent_index].1 < self.pairs[current_index].1 {
                self.pairs.swap(parent_index, current_index);
            } else {
                break;
            }
        }
    }
}

impl<T, P: PartialOrd> Heap<T, P> for DHeap<T, P> {
    fn top(&self) -> &T {
        &self.pairs[0].0
    }

    fn peek(&mut self) -> T {
        let (element, _) = self.pairs.remove(0);

        self.push_down(0);

        element
    }

    fn insert(&mut self, element: T, priority: P) {
        self.pairs.push((element, priority));

        self.bubble_up(self.pairs.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut h = DHeap::<i32, i32>::new_2heap();

        h.insert(10, 10);
        h.insert(15, 20);
        h.insert(5, 5);

        assert_eq!(*h.top(), 15);
        assert_eq!(h.peek(), 15);
        assert_eq!(h.peek(), 10);
        assert_eq!(h.peek(), 5);
    }
}
