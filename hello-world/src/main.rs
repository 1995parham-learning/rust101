use std::fmt;
use std::ops::Add;

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    return a + b;
}

#[derive(Debug)]
struct Person<T: AsRef<str>> {
    name: T,
    age: u8,
}

impl<T: AsRef<str> + fmt::Display> Person<T> {
    fn hello(&self) -> String {
        return format!("Hello {}", self.name);
    }
}

// T is the inner type of the Person to store the name, here
// we want it to have an implementation for display.
impl<T: AsRef<str> + fmt::Display> fmt::Display for Person<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "There is a person with name: {} and age {}",
            self.name, self.age
        );
    }
}

fn main() {
    println!("Hello, world!, because it is a hello world example");

    let _h: &str = "Hello";

    // create a person using an AsRef
    let mut p = Person {
        name: "Parham Alvani",
        age: 25,
    };

    println!("{}", p);

    {
        let name: String = String::from("Elahe Dastan");
        p.name = name.as_str();

        println!("{}", p);
    }

    // uncomment the following line to get a nice compile error
    // because it __moves__ into inner context and name doesn't live
    // enough
    // println!("{}", p);

    let mut p = Person {
        name: String::from("Raha Dastan"),
        age: 23,
    };

    p.name.push('!');

    println!("{}", p);

    // learn about clourses
    let p = Person {
        name: "Parham Alvani",
        age: 25,
    };

    let parham_namer = || {
        println!("{}", p.name);
    };

    parham_namer();
    parham_namer();
}
