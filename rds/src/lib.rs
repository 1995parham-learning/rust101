pub trait Heap<T, P: PartialOrd> {
    fn top(&self) -> &T;
    fn peek(&mut self) -> T;
    fn insert(&mut self, element: T, priority: P);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
