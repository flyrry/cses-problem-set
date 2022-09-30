fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let _count: usize = line.trim_end().parse().unwrap();

    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut sticks: Vec<usize> = line.trim_end().split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    sticks.sort();
    let median = sticks[sticks.len() / 2];
    let cost = sticks.iter().fold(0, |acc, value| {
        let diff = if value > &median {
            value - median
        } else {
            median - value
        };
        acc + diff
    });

    println!("{}", cost);
}
