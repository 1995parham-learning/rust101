use csv::Error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Row {
    year: String,
    make: String,
    model: String,
    description: String,
}

fn main() -> Result<(), Error> {
    let contents =
        fs::read_to_string("src/sample.csv").expect("Should have been able to read the file");

    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    let mut matrix: Vec<Vec<String>> = vec![];

    for record in reader.deserialize() {
        let row: Row = record?;

        matrix.push(vec![row.year, row.make, row.model, row.description]);
    }

    println!("{matrix:?}");

    Ok(())
}
