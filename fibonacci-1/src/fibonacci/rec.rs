use crate::fibonacci::{Fibonacci, Iter};

pub struct Recursive {}

impl Fibonacci for Recursive {
    fn fibonacci(&mut self, n: usize) -> i64 {
        if n == 0 || n == 1 {
            1
        } else {
            self.fibonacci(n - 1) + self.fibonacci(n - 2)
        }
    }
}

impl<'a> IntoIterator for &'a mut Recursive {
    type Item = i64;

    type IntoIter = Iter<'a, Recursive>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            n: 0,
            generator: self,
        }
    }
}
