#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day06() {
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

fn nb(days: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if days < 9 {
        return 0;
    }

    match cache.get(&days) {
        Some(val) => return *val,
        None => (),
    }

    let mut d = days - 2;
    let mut produced = (days - 2) / 7;
    while d >= 9 {
        d -= 7;
        produced += nb(d, cache);
    }

    cache.insert(days, produced);
    return produced;
}

pub fn solve(days: i64) {
    let input = input();

    let fishes: Vec<i64> = input
        .trim()
        .split(",")
        .map(|a| a.parse().unwrap())
        .collect();

    let mut tot = fishes.len() as i64;

    let mut collected: HashMap<i64, i64> = HashMap::new();
    for f in fishes {
        *collected.entry(f).or_insert(0) += 1;
    }

    let mut cache = HashMap::new();
    for (a, n) in collected {
        tot += n * nb(days + (8 - a), &mut cache);
    }

    println!("{}", tot);
}

pub fn part1() {
    solve(80);
}

pub fn part2() {
    solve(256);
}
