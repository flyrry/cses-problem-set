fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().unwrap();
    let m: u32 = 1_000_000_007;

    println!("{}", (1..n).fold(2, |acc, _| (acc * 2) % m));
}
