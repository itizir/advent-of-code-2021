#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day14() {
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

fn solve(steps: u64) {
    let input = input();
    let mut lines = input.lines();

    let polymer = String::from(lines.next().unwrap());
    lines.next();

    let mut pairs: HashMap<&str, u64> = HashMap::new();
    for i in 1..polymer.len() {
        *pairs.entry(&polymer[i - 1..i + 1]).or_insert(0) += 1;
    }

    let mut rules: HashMap<&str, (String, String)> = HashMap::new();
    for line in lines {
        let rule: Vec<&str> = line.split(" -> ").collect();
        let first = format!("{}{}", &rule[0][0..1], rule[1]);
        let second = format!("{}{}", rule[1], &rule[0][1..2]);
        rules.insert(rule[0], (first, second));
    }

    for _ in 0..steps {
        let mut next: HashMap<&str, u64> = HashMap::new();
        for (p, n) in pairs {
            let r = rules.get(p).unwrap();
            *next.entry(r.0.as_str()).or_insert(0) += n;
            *next.entry(r.1.as_str()).or_insert(0) += n;
        }
        pairs = next;
    }

    let mut counts: HashMap<char, u64> = HashMap::new();
    for (p, n) in pairs {
        for c in p.chars() {
            *counts.entry(c).or_insert(0) += n;
        }
    }
    *counts.get_mut(&polymer.chars().next().unwrap()).unwrap() += 1;
    *counts.get_mut(&polymer.chars().last().unwrap()).unwrap() += 1;

    let max = counts.iter().max_by_key(|c| c.1).unwrap().1 / 2;
    let min = counts.iter().min_by_key(|c| c.1).unwrap().1 / 2;
    println!("{}", max - min);
}

pub fn part1() {
    solve(10);
}

pub fn part2() {
    solve(40);
}
