mod parsemath;

use parsemath::tokenizer::Tokenizer;

fn main() {
    let tz = Tokenizer::new("1+2/3");

    for t in tz {
        println!("{:?}", t)
    }
}
