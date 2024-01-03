struct Number {
    v: i32,
}

impl<T: Into<i32>> From<T> for Number {
    fn from(value: T) -> Self {
        Self { v: value.into() }
    }
}

fn main() {
    let n = Number::from(10);
    println!("here is the {}", n.v);

    let n: Number = 1378.into();
    println!("here is the {}", n.v);

    let v: i16 = 1373;
    let n: Number = v.into();
    println!("here is the {}", n.v);
}
