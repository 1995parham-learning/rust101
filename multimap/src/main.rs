use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("phonebook.txt")?;
    let mut store: HashMap<i32, Vec<&str>> = HashMap::new();

    for line in content.lines() {
        let (number, name) = line
            .split_once(' ')
            .ok_or_else(|| format!("malformed line: {line:?}"))?;
        let key: i32 = number.parse()?;
        store.entry(key).or_default().push(name);

        println!("{name} -> {number}");
    }

    println!("{:?}", store.get(&99));
    println!("{:?}", store.get(&1));
    println!("{:?}", store.get(&2));

    Ok(())
}
