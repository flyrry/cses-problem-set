use std::cmp;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let (_, _, n) = line
        .trim()
        .bytes()
        .fold((b'X', 0, 0), |(ch, times, max), c| {
            if ch == c {
                (ch, times + 1, cmp::max(max, times + 1))
            } else {
                (c, 1, cmp::max(max, 1))
            }
        });

    println!("{}", n);
}
