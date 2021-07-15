fn main() {
    let mut line = String::with_capacity(16);
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();

    let total = (1..=n).fold(0, |sum, i| sum + i);

    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();

    let res = line
        .split_whitespace()
        .map(|numstr| numstr.parse::<usize>().unwrap())
        .fold(total, |remainder, num| remainder - num);

    println!("{}", res);
}
