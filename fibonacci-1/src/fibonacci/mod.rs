use crate::fibonacci::rec::Recursive;

mod rec;

pub trait Fibonacci {
    fn fibonacci(&self, n: i32) -> i64;
}

pub fn choose(name: &str) -> Box<dyn Fibonacci> {
    match name {
        "recursive" => Box::new(Recursive {}),
        _ => panic!("no such a fibonacci implementation named {}", name),
    }
}
