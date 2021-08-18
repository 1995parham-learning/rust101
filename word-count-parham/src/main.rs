use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Options {
    delimiter: String,
    debug: bool,
}

fn main() {
    let mut option = Options {
        delimiter: " ".to_string(),
        debug: false,
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut file_names: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        if args[i].starts_with("--") {
            match args[i].trim_start_matches("--") {
                "delimiter" => match args.get(i + 1) {
                    Some(value) => {
                        option.delimiter = value.clone();
                        i += 1;
                    }
                    None => panic!("delimiter needs a value"),
                },
                "debug" => match args.get(i + 1) {
                    Some(value) => {
                        match value.to_lowercase().as_str() {
                            "true" => option.debug = true,
                            "false" => option.debug = false,
                            _ => panic!("debug can be true or false"),
                        }
                        i += 1;
                    }
                    None => panic!("delimiter needs a value"),
                },
                &_ => {
                    panic!("option {} not supported", args[i])
                }
            }
        } else {
            file_names.push(args[i].clone());
        }
        i += 1
    }

    println!("{:?}", option);

    let mut count: usize = 0;
    for file_name in file_names {
        count += count_words_from_file(&file_name, &option)
            .unwrap_or_else(|err| panic!("cannot open {} {}", file_name, err))
    }

    println!("number of words: {}", count);
}

fn count_words_from_file(file_name: &str, option: &Options) -> Result<usize, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    if option.debug {
        println!(
            "{:?}",
            s.split(option.delimiter.as_str()).collect::<Vec<&str>>()
        );
    }

    Ok(s.split(option.delimiter.as_str()).count())
}
