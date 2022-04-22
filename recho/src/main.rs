use clap::{Arg, Command};

fn main() {
    let matches = Command::new("recho")
        .version("0.1.0")
        .author("Parham Alvani <parham.alvani@gmail.com>")
        .about("echo but this time in rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<&str> = matches.values_of("text").unwrap().collect();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
