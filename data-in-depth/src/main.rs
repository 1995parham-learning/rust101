fn main() {
    print_16bit_integers();
    print_32bit_float();
    count_to_infinity();
}

fn print_16bit_integers() {
    let a: u16 = 1378;
    let b: i16 = -1378;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let c: u16 = unsafe { std::mem::transmute(-1378i16) };

    println!("c: {:016b} {}", c, c);
}

fn print_32bit_float() {
    let f: f32 = 1378.1373;

    let fake: u32 = unsafe { std::mem::transmute(f) };

    println!("fake: {:032b} {}", fake, fake);
}

fn count_to_infinity() {
    let mut i: u16 = 0;
    print!("{}...", i);

    loop {
        i += 1000;
        print!("{}...", i);
        if i % 10000 == 0 {
            println!();
        }
    }
}
