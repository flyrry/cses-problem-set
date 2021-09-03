#[derive(Default, Clone, Debug, Copy)]
struct Path {
    len: usize,
    via: usize
}
type AltPathPairs = Vec<(Path, Path)>;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim_end().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut dist: AltPathPairs = vec![(Default::default(), Default::default()); n + 1];
    let mut start = 0;
    for _ in 1..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let (a, b) = read_a_b(&mut input);
        adj[a].push(b);
        adj[b].push(a);
        start = b;
    }

    dfs(&adj[..], &mut dist, start, 0);
    dfs2(&adj[..], &mut dist, start, 0);
    for i in 1..=n {
        print!("{} ", dist[i].0.len);
    }
    println!();
}

// calculate two max paths via child nodes from every node (truly max, and alternative)
fn dfs(adj: &[Vec<usize>], dist: &mut AltPathPairs, from: usize, prev: usize) -> usize {
    for &next in &adj[from] {
        if next != prev {
            let distance = dfs(adj, dist, next, from) + 1;
            if distance > dist[from].0.len {
                dist[from].1 = dist[from].0;
                dist[from].0 = Path {len: distance, via: next};
            } else if distance > dist[from].1.len {
                dist[from].1 = Path {len: distance, via: next};
            }
        }
    }
    dist[from].0.len
}

// max distance is either
// max path via child
// ...or via parent + 1 (unless it would go through this child node)
// ...or via alt parent + 1
fn dfs2(adj: &[Vec<usize>], dist: &mut AltPathPairs, from: usize, prev: usize) {
    for &next in &adj[from] {
        if next != prev {
            if dist[from].0.via == next {
                if dist[next].0.len < dist[from].1.len + 1 {
                    dist[next].1 = dist[next].0;
                    dist[next].0 = Path {len: dist[from].1.len + 1, via: from};
                } else if dist[next].1.len < dist[from].1.len + 1 {
                    dist[next].1 = Path {len: dist[from].1.len + 1, via: from};
                }
            } else {
                dist[next].1 = dist[next].0;
                dist[next].0 = Path {len: dist[from].0.len + 1, via: from};
            }
            dfs2(adj, dist, next, from);
        }
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
