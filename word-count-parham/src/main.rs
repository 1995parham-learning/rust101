#[derive(Debug)]
struct Options {
    delimiter: String,
}

fn main() {
    let mut option = Options {
        delimiter: "".to_string(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();

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
        }
    }

    println!("{:?}", option);
}
