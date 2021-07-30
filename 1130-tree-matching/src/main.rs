fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited = vec![false; n + 1];
    let mut start = 0;
    for _ in 1..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let (a, b) = read_a_b(&mut input);
        adj[a].push(b);
        adj[b].push(a);
        start = b;
    }

    let mut count = 0;
    visited[0] = true;
    dfs(&adj[..], &mut visited, start, 0, &mut count);
    println!("{}", count);
}

fn dfs(adj: &[Vec<usize>], visited: &mut Vec<bool>, from: usize, prev: usize, count: &mut usize) {
    for &next in &adj[from] {
        if next != prev {
            dfs(adj, visited, next, from, count);
        }
    }
    if !visited[from] && !visited[prev] {
        *count += 1;
        visited[from] = true;
        visited[prev] = true;
    }
}

fn read_a_b(input: &mut String) -> (usize, usize) {
    let mut a_b = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let a = a_b.next().unwrap();
    let b = a_b.next().unwrap();
    (a, b)
}
