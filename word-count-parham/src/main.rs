use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Options {
    delimiter: String,
}

fn main() {
    let mut option = Options {
        delimiter: "".to_string(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut file_names: Vec<String> = Vec::new();

    for i in 0..args.len() {
        if args[i].starts_with("--") {
            match args[i].trim_start_matches("--") {
                "delimiter" => match args.get(i + 1) {
                    Some(value) => option.delimiter = value.clone(),
                    None => panic!("delimiter needs a value"),
                },
                &_ => {
                    panic!("option {} not supported", args[i])
                }
            }
        } else {
            file_names.push(args[i].clone());
        }
    }

    println!("{:?}", option);

    let mut count: usize = 0;
    for file_name in file_names {
        count += count_words_from_file(&file_name)
            .unwrap_or_else(|err| panic!("cannot open {} {}", file_name, err))
    }

    println!("number of words: {}", count);
}

fn count_words_from_file(file_name: &str) -> Result<usize, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.split_whitespace().count())
}
