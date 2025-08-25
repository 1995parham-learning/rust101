use crate::fibonacci::choose;

mod fibonacci;

fn main() {
    let mut r = choose("dynamic");

    println!("{}", r.fibonacci(10));
}
