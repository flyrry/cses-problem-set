use std::fmt::Write;

fn read_a_b(input: &mut String) -> (usize, usize) {
    let mut a_b = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let a = a_b.next().unwrap();
    let b = a_b.next().unwrap();
    (a, b)
}

fn main() {
    let mut input = String::with_capacity(32);
    std::io::stdin().read_line(&mut input).unwrap();
    let (n, q) = read_a_b(&mut input);

    let mut buf = String::with_capacity(11 * n);
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut xs: Vec<u64> = Vec::with_capacity(n + 1);
    xs.push(0);
    let _ = buf
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .enumerate()
        .for_each(|(i, x)| xs.push(x + xs[i]));

    buf.clear();
    for _ in 0..q {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let (a, b) = read_a_b(&mut input);
        writeln!(buf, "{}", xs[b] - xs[a - 1]).unwrap();
    }
    print!("{}", buf);
}
