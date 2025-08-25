use crate::fibonacci::{dynamic::Dynamic, rec::Recursive};

mod dynamic;
mod rec;

pub trait Fibonacci {
    fn fibonacci(&mut self, n: usize) -> i64;
}

pub struct Iter<'a, T: Fibonacci> {
    n: usize,
    generator: &'a mut T,
}

impl<'a, T: Fibonacci> Iterator for Iter<'a, T> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        Some(self.generator.fibonacci(self.n - 1))
    }
}

pub fn choose(name: &str) -> Box<dyn Fibonacci> {
    match name {
        "recursive" => Box::new(Recursive {}),
        "dynamic" => Box::new(Dynamic::new()),
        _ => panic!("no such a fibonacci implementation named {}", name),
    }
}
