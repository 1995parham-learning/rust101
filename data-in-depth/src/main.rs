fn main() {
    print_16bit_integers();
    print_32bit_float();
    // count_to_infinity();
    split_float32();
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

fn _count_to_infinity() {
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

fn split_float32() {
    let n: f32 = 1378.1378;

    let n_bits: u32 = n.to_bits();

    let sign_bit = n_bits >> 31;
    println!("{}", sign_bit);

    let exponent = (((n_bits >> 23) & 0xff) as i32) - 127;
    println!("{}", exponent);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;

        if one_at_bit_i != 0 {
            mantissa += 2f32.powf(i as f32 - 23.0);
        }
    }
    println!("{}", mantissa);
}
