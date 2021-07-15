use std::cmp::min;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut n_x = line
        .trim_end()
        .split_ascii_whitespace();
    let n: usize = n_x.next().unwrap().parse().unwrap();
    let x: i32 = n_x.next().unwrap().parse().unwrap();
    line.clear();
    line.reserve(n * 8);
    std::io::stdin().read_line(&mut line).unwrap();
    let mut coins = Vec::with_capacity(n);
    coins.extend(
        line
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
    );
    coins.sort();

    let mut mins = [-1; 1_000_001];
    mins[0] = 0;

    for sum in 1..=x {
        let i: usize = sum as usize;
        mins[i] = -1;
        for c in &coins {
            let left = sum - *c;
            if left < 0 {
                break;
            }
            if mins[left as usize] != -1 {
                let current = match mins[i] {
                    -1 => i32::MAX,
                    n => n
                };
                mins[i] = min(mins[left as usize] + 1, current);
            }
        }
    }

    println!("{}", mins[x as usize]);
}
