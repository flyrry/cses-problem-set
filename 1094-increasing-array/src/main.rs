fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut moves = 0;
    let mut numbers = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let first = numbers.next().unwrap();
    numbers
        .fold(first, |a, e| {
        if e < a {
            moves += a - e;
            a
        } else {
            e
        }
    });

    println!("{}", moves);
}
