fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut start = 0;
    for _ in 1..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let (a, b) = read_a_b(&mut input);
        adj[a].push(b);
        adj[b].push(a);
        start = b;
    }

    let (farthest, _) = dfs(&adj[..], start, 0);
    let (_, diameter) = dfs(&adj[..], farthest, 0);

    println!("{}", diameter - 1);
}

fn dfs(adj: &[Vec<usize>], from: usize, prev: usize) -> (usize, usize) {
    let (mut max_node, mut max_distance) = (from, 0);
    for &next in &adj[from] {
        if next != prev {
            let (node, distance) = dfs(adj, next, from);
            if distance > max_distance {
                max_node = node;
                max_distance = distance;
            }
        }
    }
    (max_node, max_distance + 1)
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
