use std::io::{BufWriter, Write};

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let n: usize = line.trim_end().parse().unwrap();

    match n {
        1 => println!("1"),
        2 | 3 => println!("NO SOLUTION"),
        _ => print_beautiful(n)
    }
}

fn print_beautiful(n: usize) {
    let stdout = std::io::stdout();
    let mut out = BufWriter::with_capacity(n * 8, stdout.lock());

    for x in (2..=n).step_by(2) {
        write!(out, "{} ", x).unwrap();
    }
    for x in (1..=n).step_by(2) {
        write!(out, "{} ", x).unwrap();
    }
}
