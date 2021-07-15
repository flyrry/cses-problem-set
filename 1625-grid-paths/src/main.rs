type Grid = [[bool; 7]; 7];

fn main() {
    let mut line = String::with_capacity(16);
    std::io::stdin().read_line(&mut line).unwrap();

    let mut grid: Grid = [[false; 7]; 7];

    println!("{}", paths(line.trim().as_bytes(), &mut grid, 0, 0));
}

fn splitting_grid_in_two(grid: &mut Grid, x: usize, y: usize) -> bool {
    let m = grid.len() - 1;
    if y == m || grid[x][y + 1] == true {
        // border is below
        if x > 0 && x < m && grid[x - 1][y] == false && grid[x + 1][y] == false {
            if y > 0 && grid[x][y - 1] == true {
                // and we came from above
                return true;
            }
        }
    }
    if x == m || grid[x + 1][y] == true {
        // border is to the right
        if y > 0 && y < m && grid[x][y - 1] == false && grid[x][y + 1] == false {
            if x > 0 && grid[x - 1][y] == true {
                // and we came from the left
                return true;
            }
        }
    }
    if x == 0 || grid[x - 1][y] == true {
        // border is to the left
        if y > 0 && y < m && grid[x][y - 1] == false && grid[x][y + 1] == false {
            if x < m && grid[x + 1][y] == true {
                // and we came from the right
                return true;
            }
        }
    }
    if y == 0 || grid[x][y - 1] == true {
        // border is above
        if x > 0 && x < m && grid[x - 1][y] == false && grid[x + 1][y] == false {
            if y < m && grid[x][y + 1] == true {
                // and we came from below
                return true;
            }
        }
    }
    return false;
}

fn paths(trail: &[u8], grid: &mut Grid, x: usize, y: usize) -> usize {
    // we reached lower left corner, but still have path steps left
    if x == 0 && y == grid.len() - 1 && trail.len() > 0 {
        return 0;
    }

    grid[x][y] = true;

    // early bail out if grid is split in two by this move
    if splitting_grid_in_two(grid, x, y) {
        grid[x][y] = false;
        return 0;
    }

    let count = match trail.iter().next() {
        None => 1,
        Some(direction) => match direction {
            b'?' => [b'D', b'R', b'L', b'U'].iter().fold(0usize, |acc, c| {
                acc + try_path(*c, trail, grid, x, y)
            }),
            c => try_path(*c, trail, grid, x, y),
        },
    };
    grid[x][y] = false;
    count
}

fn try_path(direction: u8, trail: &[u8], grid: &mut Grid, x: usize, y: usize) -> usize {
    let remaining = &trail[1..];
    let m = grid.len() - 1;
    match direction {
        b'U' => {
            if y == 0 || grid[x][y - 1] == true {
                0
            } else {
                paths(remaining, grid, x, y - 1)
            }
        }
        b'L' => {
            if x == 0 || grid[x - 1][y] == true {
                0
            } else {
                paths(remaining, grid, x - 1, y)
            }
        }
        b'D' => {
            if y == m || grid[x][y + 1] == true {
                0
            } else {
                paths(remaining, grid, x, y + 1)
            }
        }
        b'R' => {
            if x == m || grid[x + 1][y] == true {
                0
            } else {
                paths(remaining, grid, x + 1, y)
            }
        }
        _ => 0,
    }
}
