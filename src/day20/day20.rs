#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day20() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

pub fn part1() {
    solve(2);
}

pub fn part2() {
    solve(50);
}

fn get_cell(grid: &Vec<Vec<bool>>, x: i64, y: i64, default: usize) -> usize {
    if 0 <= y && y < grid.len() as i64 {
        let row = &grid[y as usize];
        if 0 <= x && x < row.len() as i64 {
            return row[x as usize] as usize;
        }
    }
    default
}

fn cell_index(grid: &Vec<Vec<bool>>, x: i64, y: i64, default: usize) -> usize {
    256 * get_cell(grid, x - 1, y - 1, default)
        + 128 * get_cell(grid, x, y - 1, default)
        + 64 * get_cell(grid, x + 1, y - 1, default)
        + 32 * get_cell(grid, x - 1, y, default)
        + 16 * get_cell(grid, x, y, default)
        + 8 * get_cell(grid, x + 1, y, default)
        + 4 * get_cell(grid, x - 1, y + 1, default)
        + 2 * get_cell(grid, x, y + 1, default)
        + get_cell(grid, x + 1, y + 1, default)
}

fn solve(num_rounds: usize) {
    let input = input();
    let mut lines = input.lines();

    let alg: Vec<bool> = lines.next().unwrap().chars().map(|c| c == '#').collect();
    lines.next();

    assert_eq!(512, alg.len());
    assert_ne!(alg[0], alg[alg.len() - 1]);

    let flicker = alg[0];

    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
        grid.push(row);
    }

    let mut default = 0;
    for _ in 0..num_rounds {
        let mut next: Vec<Vec<bool>> = Vec::with_capacity(grid.len() + 2);
        for y in -1..(grid.len() as i64 + 1) {
            let mut row: Vec<bool> = Vec::with_capacity(grid[0].len() + 2);
            for x in -1..(grid[0].len() as i64 + 1) {
                row.push(alg[cell_index(&grid, x, y, default)]);
            }
            next.push(row);
        }
        grid = next;
        default = (flicker as usize) - default;
    }

    let mut tot = 0;
    for r in grid {
        for c in r {
            if c {
                tot += 1;
            }
        }
    }
    println!("{}", tot);
}
