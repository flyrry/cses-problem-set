use std::io::{BufWriter, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    input.clear();
    input.reserve(n * 7);
    std::io::stdin().read_line(&mut input).unwrap();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut count = vec![0_usize; n];

    input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap() - 1)
        .zip(1_usize..)
        .for_each(|(boss, emp)| adj[boss].push(emp));

    dfs(&mut count[..], &adj[..], 0);
    print(&count);
}

fn dfs(count: &mut [usize], adj: &[Vec<usize>], e: usize) {
    for &s in &adj[e] {
        dfs(count, adj, s);
        count[e] += count[s] + 1;
    }
}

fn print(count: &[usize]) {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut out = BufWriter::with_capacity(7 * count.len(), lock);

    for &s in count {
        write!(out, "{} ", s).unwrap();
    }
    writeln!(out).unwrap();
}
