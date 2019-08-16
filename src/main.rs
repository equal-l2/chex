use std::io;
use std::io::Read;
use std::fmt::Write;

const BYTES_PER_LINE: usize = 16;

fn output_line(elems: &[u8], empty_char: char) {
    let rem = BYTES_PER_LINE - elems.len();

    // graphical expression
    let width = 1;
    print!("\x1b[31m│\x1b[0m");
    for b in elems {
        print!("\x1b[48;5;{}m{}", b, " ".repeat(width));
    }
    print!("\x1b[0m");
    if rem != 0 {
        print!("{}", " ".repeat(rem*width));
    }
    print!("\x1b[31m│\x1b[0m");

    // bytes (hex)
    let mut s1 = String::with_capacity(3*BYTES_PER_LINE);
    for b in elems {
        write!(s1, "{:02X} ", b).unwrap();
    }
    if rem != 0 {
        write!(s1, "{}", " ".repeat(rem*3)).unwrap();
    }
    s1.pop();
    print!("{}\x1b[31m│\x1b[0m", s1);

    // chars
    for b in elems {
        let c = if *b >= 32 && *b <= 126 { *b as char } else { empty_char };
        print!("{}", c);
    }
    if rem != 0 {
        print!("{}", " ".repeat(rem));
    }
    print!("\x1b[31m│\x1b[0m");
    println!();
}

fn run(reader: impl Read, empty_char: char) {
    let mut v = Vec::with_capacity(BYTES_PER_LINE);
    for b in reader.bytes() {
        v.push(b.unwrap());
        if v.len() == BYTES_PER_LINE {
            output_line(&v, empty_char);
            v.clear();
        }
    }
    if !v.is_empty() {
        output_line(&v, empty_char);
    }
}

fn main() {
    let fname = std::env::args().nth(1);
    let empty_char = std::env::args().nth(2);
    let empty_char = if let Some(i) = empty_char {
        i.chars().nth(0).unwrap()
    } else {
        '.'
    };

    if let Some(i) = fname {
        run(io::BufReader::new(std::fs::File::open(i).unwrap()), empty_char);
    } else {
        run(io::BufReader::new(io::stdin()), empty_char);
    };

}
