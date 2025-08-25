use crate::fibonacci::{Fibonacci, Iter};

pub struct Dynamic {
    data: Vec<i64>,
}

impl Dynamic {
    pub fn new() -> Self {
        return Dynamic { data: vec![1, 1] };
    }
}

impl Fibonacci for Dynamic {
    fn fibonacci(&mut self, n: usize) -> i64 {
        match self.data.get(n) {
            Some(v) => *v,
            None => {
                let v = self.fibonacci(n - 1) + self.fibonacci(n - 2);
                self.data.insert(n, v);
                v
            }
        }
    }
}

impl<'a> IntoIterator for &'a mut Dynamic {
    type Item = i64;

    type IntoIter = Iter<'a, Dynamic>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            n: 0,
            generator: self,
        }
    }
}
