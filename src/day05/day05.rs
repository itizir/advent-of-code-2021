#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day05() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

use std::collections::HashMap;

fn parse_line(line: &str) -> (i64, i64) {
    let vec: Vec<i64> = line
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|a| a.parse().unwrap())
        .collect();
    (vec[0], vec[1])
}
fn order(a: i64, b: i64) -> (i64, i64, bool) {
    if a < b {
        (a, b, false)
    } else {
        (b, a, true)
    }
}

fn solve(with_diagonals: bool) {
    let input = input();

    let mut grid = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(" -> ").collect();
        let from = parse_line(split[0]);
        let to = parse_line(split[1]);

        if from.0 == to.0 {
            let x = from.0;
            let (ymin, ymax, _) = order(from.1, to.1);
            for y in ymin..=ymax {
                grid.insert((x, y), 1 + grid.get(&(x, y)).unwrap_or(&0));
            }
        } else if from.1 == to.1 || with_diagonals {
            let (xmin, xmax, swapped) = order(from.0, to.0);
            let mut y = if swapped { to.1 } else { from.1 };
            let yto = if swapped { from.1 } else { to.1 };
            let step = if !with_diagonals || y == yto {
                0
            } else if y < yto {
                1
            } else {
                -1
            };
            for x in xmin..=xmax {
                grid.insert((x, y), 1 + grid.get(&(x, y)).unwrap_or(&0));
                y += step;
            }
        }
    }

    let mut count = 0;
    for (_, val) in grid.iter() {
        if val > &1 {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn part1() {
    solve(false);
}

pub fn part2() {
    solve(true);
}
