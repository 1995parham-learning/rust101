struct Person {
    name: String,
    age: i32,
}

fn print(p: &Person) {
    println!("{} {}", p.name, p.age);
}

fn older(p: &mut Person, years: i32) {
    p.age += years;
}

fn main() {
    let p = Person{
        name: String::from("Elahe Dastan"),
        age: 30,
    };

    print(&p);
    
    p;
}
