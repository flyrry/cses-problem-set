use std::io::{BufWriter, Write};

fn read_y_x(input: &mut String) -> (usize, usize) {
    let mut a_b = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let a = a_b.next().unwrap();
    let b = a_b.next().unwrap();
    (a, b)
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let tests: usize = line.trim_end().parse().unwrap();

    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut out = BufWriter::with_capacity(12 * tests, lock);
    for _ in 0..tests {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let (y, x) = read_y_x(&mut line);
        let n = y.max(x);
        let middle = n * n - n + 1;
        let result = match n % 2 {
            0 => middle - (n - y) + (n - x),
            _ => middle + (n - y) - (n - x)
        };
        writeln!(out, "{}", result).unwrap();
    }
}
