use crate::fibonacci::choose;

mod fibonacci;

fn main() {
    let r = choose("recursive");

    println!("{}", r.fibonacci(10));
}
