fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // This slice has the type &[i32]

    // assert_eq! is a macro that panics if the two arguments aren't equal.
    // It's useful for testing.
    assert_eq!(slice, &[2, 3]);
}
