use csv::Error;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Row {
    year: String,
    make: String,
    model: String,
    description: String,
}

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("src/sample.csv")
    .expect("Should have been able to read the file");

    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    let mut matrix: Vec<Vec<String>>;

    for record in reader.deserialize() {
        let row: Row = record?;

        let mut new_row = vec![vec![
                row.year
            ]
        ];
        matrix.append(
            &mut new_row,
        );
    }

    println!("{:?}", matrix);

    Ok(())
}

