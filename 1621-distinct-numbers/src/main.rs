use std::collections::HashSet;

fn main() {
    let mut line = String::with_capacity(200000 * 10 + 200001);
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    line.clear();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut res = HashSet::with_capacity(n);
    res.extend(line.split_whitespace());

    println!("{}", res.len());
}
