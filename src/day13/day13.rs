#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day13() {
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
use std::collections::HashSet;

fn fold(dots: HashMap<u64, HashSet<u64>>, line: (u64, u64)) -> HashMap<u64, HashSet<u64>> {
    if line.1 == 0 {
        fold_x(dots, line.0)
    } else {
        fold_y(dots, line.1)
    }
}

fn fold_y(dots: HashMap<u64, HashSet<u64>>, line: u64) -> HashMap<u64, HashSet<u64>> {
    let mut new_dots: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (key, xs) in dots {
        let mut y = key;
        if y >= line {
            y = 2 * line - y;
        }
        if new_dots.contains_key(&y) {
            new_dots.get_mut(&y).unwrap().extend(xs);
        } else {
            new_dots.insert(y, xs);
        }
    }
    new_dots
}

fn fold_x(dots: HashMap<u64, HashSet<u64>>, line: u64) -> HashMap<u64, HashSet<u64>> {
    let mut new_dots: HashMap<u64, HashSet<u64>> = HashMap::new();
    for (y, xs) in dots {
        let mut new_line = HashSet::new();
        for key in xs.iter() {
            let mut x = *key;
            if x >= line {
                x = 2 * line - x;
            }
            new_line.insert(x);
        }
        new_dots.insert(y, new_line);
    }
    new_dots
}

fn parse_input() -> (HashMap<u64, HashSet<u64>>, Vec<(u64, u64)>) {
    let input = input();

    let mut dots: HashMap<u64, HashSet<u64>> = HashMap::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.len() == 0 {
            break;
        }
        let coords: Vec<u64> = line.split(",").map(|n| n.parse().unwrap()).collect();
        dots.entry(coords[1])
            .or_insert(HashSet::from([coords[0]]))
            .insert(coords[0]);
    }

    let mut folds: Vec<(u64, u64)> = Vec::new();
    for line in lines {
        let line = line.strip_prefix("fold along ").unwrap();
        let parts: Vec<&str> = line.split("=").collect();
        if parts[0] == "x" {
            folds.push((parts[1].parse().unwrap(), 0));
        } else {
            folds.push((0, parts[1].parse().unwrap()));
        }
    }

    (dots, folds)
}

pub fn part1() {
    let (mut dots, folds) = parse_input();

    dots = fold(dots, folds[0]);
    let mut n = 0;
    for (_, d) in dots.iter() {
        n += d.len();
    }
    println!("{}", n);
}

pub fn part2() {
    let (mut dots, folds) = parse_input();

    for f in folds {
        dots = fold(dots, f);
    }

    for y in 0..=*dots.keys().max().unwrap() {
        let line = match dots.get(&y) {
            Some(l) => l,
            None => continue,
        };
        for x in 0..=*line.iter().max().unwrap() {
            if line.contains(&x) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
