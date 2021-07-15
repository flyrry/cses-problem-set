use std::collections::VecDeque;

#[derive(Clone, Debug, Copy)]
enum Tile {
    Wall,
    Empty,
    Monster(usize),
    Player {turns: usize, from: Option<Point>, dir: char}
}

type Point = (usize, usize);

struct Step {
    x: usize,
    y: usize,
    turns: usize,
}

fn main() {
    let mut line = String::with_capacity(16);
    std::io::stdin().read_line(&mut line).unwrap();
    let mut dimensions = line.split_whitespace();
    let n = dimensions.next().unwrap().parse::<usize>().unwrap();
    let m = dimensions.next().unwrap().parse::<usize>().unwrap();

    let mut maze = vec![vec![Tile::Empty; m]; n];
    let mut monsters: VecDeque<Step> = VecDeque::with_capacity(m * n);
    let mut player_x: usize = 0;
    let mut player_y: usize = 0;
    line.reserve(n + 2);
    for y in 0..n {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut chars = line.chars();
        for x in 0..m {
            maze[y][x] = match chars.next().unwrap() {
                '#' => Tile::Wall,
                'A' => {
                    player_y = y;
                    player_x = x;
                    Tile::Empty
                },
                'M' => {
                    monsters.push_back(Step {y, x, turns: 0});
                    //Tile::Monster(0)
                    Tile::Empty
                },
                _ => Tile::Empty,
            }
        }
    }
    //print_maze(&maze);

    build_heatmap(&mut maze, &mut monsters);

    //print_maze(&maze);

    match run_player(&mut maze, (player_y, player_x)) {
        Some(tile) => {
            match tile {
                Tile::Player {turns, ..} => {
                    println!("YES\n{}\n{}", turns, path_to(&maze, &tile))
                },
                _ => panic!("WOT"),
            }
        },
        _ => println!("NO"),
    }

    //print_maze(&maze);
}

fn path_to<'a>(maze: &'a Vec<Vec<Tile>>, exit: &Tile) -> String {
    let (h, w) = dimensions(maze);
    let mut path = String::with_capacity(h * w / 2);
    let mut tile = exit;

    while let Tile::Player {from, dir, ..} = tile {
        if let Some((y, x)) = from {
            path.push(*dir);
            tile = &maze[*y][*x];
        } else {
            break;
        }
    }

    path.chars().rev().collect()
}

fn run_player(
    maze: &mut Vec<Vec<Tile>>,
    (origin_y, origin_x): Point
) -> Option<Tile> {
    let (h, w) = dimensions(maze);
    let mut steps: VecDeque<Step> = VecDeque::with_capacity(h * w);
    steps.push_back(Step {y: origin_y, x: origin_x, turns: 0});
    maze[origin_y][origin_x] = Tile::Player {turns: 0, from: None, dir: 'X'};
    while let Some(Step {y, x, turns}) = steps.pop_front() {

        if is_edge((y, x), (h, w)) {
            return Some(maze[y][x]);
        }

        walk((y, x), dimensions(maze), |(ny, nx), dir| {
            if match maze[ny][nx] {
                Tile::Empty => true,
                Tile::Monster(value) if turns + 1 < value => true,
                _ => false,
            } {
                maze[ny][nx] = Tile::Player {turns: turns + 1, from: Some((y, x)), dir};
                steps.push_back(Step {y: ny, x: nx, turns: turns + 1});
            }
        });
    }
    None
}

fn is_edge((y, x): Point, (h, w): Point) -> bool {
    y == 0 || x == 0 || y == h - 1 || x == w - 1
}

fn dimensions(maze: &Vec<Vec<Tile>>) -> Point {
    (maze.len(), maze[0].len())
}

fn walk<F: FnMut(Point, char)>(
    (y, x): Point,
    (h, w): Point, // strictly speaking not a point >.>
    mut f: F
) {
    if y > 0 { f((y - 1, x), 'U'); }
    if x > 0 { f((y, x - 1), 'L'); }
    if y + 1 < h { f((y + 1, x), 'D'); }
    if x + 1 < w { f((y, x + 1), 'R'); }
}

fn build_heatmap(maze: &mut Vec<Vec<Tile>>, monsters: &mut VecDeque<Step>) {
    while let Some(Step {y, x, turns, ..}) = monsters.pop_front() {
        if match maze[y][x] {
            Tile::Empty => true,
            Tile::Monster(value) if turns < value => true,
            _ => false,
        } {
            maze[y][x] = Tile::Monster(turns);
            walk((y, x), dimensions(maze), |(ny, nx), _| {
                monsters.push_back(Step {
                    y: ny,
                    x: nx,
                    turns: turns + 1,
                });
            });
        }
    }
}

/*
fn print_maze(maze: &Vec<Vec<Tile>>) {
    for line in maze.iter() {
        println!("{:?}", line);
    }
}
*/
