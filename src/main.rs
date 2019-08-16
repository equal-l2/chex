use std::io::Read;

const BYTES_PER_LINE: usize = 16;

fn output_line(elems: &[u8]) {
    let rem = BYTES_PER_LINE - elems.len();

    print!("\x1b[31m|\x1b[0m");
    for b in elems {
        print!("\x1b[48;5;{}m ", b);
    }
    print!("\x1b[0m");
    if rem != 0 {
        print!("{}", " ".repeat(rem));
    }
    print!("\x1b[31m|\x1b[0m");

    for b in elems {
        print!(" {:02X}", b);
    }
    if rem != 0 {
        print!("{}", " ".repeat(rem*3));
    }
    print!(" \x1b[31m|\x1b[0m");

    for b in elems {
        let c = if *b >= 32 && *b <= 126 { *b as char } else { ' ' };
        print!("{}", c);
    }
    if rem != 0 {
        print!("{}", " ".repeat(rem));
    }
    print!("\x1b[31m|\x1b[0m");
    println!();
}

fn main() {
    let fname = std::env::args().nth(1).unwrap();
    let bytes = std::fs::File::open(fname).unwrap().bytes();

    let mut v = Vec::with_capacity(BYTES_PER_LINE);
    for b in bytes {
        v.push(b.unwrap());
        if v.len() == BYTES_PER_LINE {
            output_line(&v);
            v.clear();
        }
    }
    if !v.is_empty() {
        output_line(&v);
    }
}
