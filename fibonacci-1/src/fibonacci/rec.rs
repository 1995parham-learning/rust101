use crate::fibonacci::Fibonacci;

pub struct Recursive {}

impl Fibonacci for Recursive {
    fn fibonacci(&self, n: i32) -> i64 {
        if n == 0 || n == 1 {
            1
        } else {
            self.fibonacci(n - 1) + self.fibonacci(n - 2)
        }
    }
}
