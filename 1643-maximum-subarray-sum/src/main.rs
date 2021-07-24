use std::cmp::max;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    input.clear();
    input.reserve(n * 12);
    std::io::stdin().read_line(&mut input).unwrap();
    let mut xs: Vec<i64> = Vec::with_capacity(n);
    xs.extend(
        input
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap()),
    );

    let mut sum = 0;
    let result = xs.into_iter().fold(i64::MIN, |best, e| {
        sum = max(e, sum + e);
        max(best, sum)
    });

    println!("{}", result);
}
