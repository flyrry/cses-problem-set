fn main() {
    let mut input = String::with_capacity(18);
    std::io::stdin().read_line(&mut input).unwrap();
    let mut n_x = input.trim_end().split_ascii_whitespace();
    let n = n_x.next().unwrap().parse::<usize>().unwrap();
    let x = n_x.next().unwrap().parse::<u32>().unwrap();

    input.clear();
    input.reserve(n * 11);
    std::io::stdin().read_line(&mut input).unwrap();
    let mut aa: Vec<(usize, u32)> = Vec::with_capacity(n);
    aa.extend(
        input
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .enumerate(),
    );
    aa.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

    let mut b = 0;
    let mut e = aa.len() - 1;

    while b < e {
        let sum = aa[b].1 + aa[e].1;
        if sum < x {
            b += 1;
        } else if sum > x {
            e -= 1;
        } else {
            println!("{} {}", aa[b].0 + 1, aa[e].0 + 1);
            return;
        }
    }
    println!("IMPOSSIBLE");
}
