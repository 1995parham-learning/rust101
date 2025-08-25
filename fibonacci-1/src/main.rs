fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 0;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("{}", fibonacci(10));
}
