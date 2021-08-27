use std::fmt::Display;

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
    pub fn new_heap2() -> DHeap<T, P> {
        DHeap::new(2)
    }

    pub fn new(d: usize) -> DHeap<T, P> {
        DHeap {
            pairs: std::vec::Vec::new(),
            d,
        }
    }

    // highest_priority_child for the given node returns its child index which has the
    // leargest priority.
    fn highest_priority_child(&self, index: usize) -> usize {
        let mut highest_priority_child = self.d * index + 1;
        for i in 1..self.d {
            let child_index = self.d * index + i + 1;
            match self.pairs.get(child_index) {
                Some(child) => {
                    if child.1 > self.pairs[highest_priority_child].1 {
                        highest_priority_child = child_index;
                    }
                }
                None => break,
            }
        }

        highest_priority_child
    }

    // first_leaf_index return the index of the first leaf node.
    fn first_leaf_index(&self) -> usize {
        match self.pairs.len() {
            0 => 0,
            1 => 0,
            _ => (self.pairs.len() - 2) / self.d + 1,
        }
    }

    // push_donw maintains the max heap property.
    // it moves an element from the top of heap to its position.
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

    // bubble_up maintains the max heap property.
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

impl<T: PartialEq, P: PartialOrd> DHeap<T, P> {
    pub fn update(&mut self, element: &T, priority: P) {
        let mut index = self.pairs.len();

        for i in 0..self.pairs.len() {
            if self.pairs[i].0 == *element {
                index = i;
            }
        }

        if index == self.pairs.len() {
            return;
        }

        let direction = self.pairs[index].1 < priority;

        self.pairs[index].1 = priority;
        if direction {
            self.bubble_up(index);
        } else {
            self.push_down(index);
        }
    }
}

impl<T, P> Display for DHeap<T, P>
where
    T: Display,
    P: PartialOrd,
    P: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();

        for (element, priority) in &self.pairs {
            r = if r.is_empty() {
                format!("({}, {})", element, priority)
            } else {
                format!("{} ({}, {})", r, element, priority)
            }
        }

        write!(f, "{}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap2_insert_peek() {
        let mut h = DHeap::<i32, i32>::new_heap2();

        h.insert(10, 10);
        h.insert(15, 20);
        h.insert(5, 5);

        assert_eq!(*h.top(), 15);
        assert_eq!(h.peek(), 15);
        assert_eq!(h.peek(), 10);
        assert_eq!(h.peek(), 5);
    }

    #[test]
    fn heap2_update() {
        let mut h = DHeap::<&str, i32>::new_heap2();

        h.insert("Elahe", 100);
        h.insert("Parham", 0);
        h.insert("Iman", 120);

        assert_eq!(*h.top(), "Iman");

        h.update(&"Iman", 80);

        assert_eq!(*h.top(), "Elahe");
    }

    #[test]
    fn heap2_print() {
        let mut h = DHeap::<&str, i32>::new_heap2();

        h.insert("Elahe", 200);
        h.insert("Parham", 0);
        h.insert("Iman", 120);

        assert_eq!(format!("{}", h), "(Elahe, 200) (Parham, 0) (Iman, 120)");
    }
}
