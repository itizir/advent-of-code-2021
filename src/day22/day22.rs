#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day22() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

use std::cmp::max;
use std::cmp::min;

struct Cuboid {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
}

impl Cuboid {
    fn new(x0: i64, x1: i64, y0: i64, y1: i64, z0: i64, z1: i64) -> Cuboid {
        Cuboid {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        }
    }

    fn volume(&self) -> i64 {
        let v = (1 + self.x1 - self.x0) * (1 + self.y1 - self.y0) * (1 + self.z1 - self.z0);
        if v > 0 {
            v
        } else {
            0
        }
    }

    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        let x0 = max(self.x0, other.x0);
        let x1 = min(self.x1, other.x1);
        if x1 < x0 {
            return None;
        }
        let y0 = max(self.y0, other.y0);
        let y1 = min(self.y1, other.y1);
        if y1 < y0 {
            return None;
        }
        let z0 = max(self.z0, other.z0);
        let z1 = min(self.z1, other.z1);
        if z1 < z0 {
            return None;
        }
        Some(Cuboid::new(x0, x1, y0, y1, z0, z1))
    }

    fn is_init(&self) -> bool {
        let l = 50;
        self.x0.abs() <= l
            && self.x1.abs() <= l
            && self.y0.abs() <= l
            && self.y1.abs() <= l
            && self.z0.abs() <= l
            && self.z1.abs() <= l
    }
}

fn solve(init: bool) {
    let input = input();

    let mut pos: Vec<Cuboid> = Vec::new();
    let mut neg: Vec<Cuboid> = Vec::new();

    for line in input.lines() {
        let on = line.starts_with("on");
        let parts: Vec<&str> = line.split("=").collect();
        let x: Vec<i64> = parts[1]
            .strip_suffix(",y")
            .unwrap()
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect();
        let y: Vec<i64> = parts[2]
            .strip_suffix(",z")
            .unwrap()
            .split("..")
            .map(|n| n.parse().unwrap())
            .collect();
        let z: Vec<i64> = parts[3].split("..").map(|n| n.parse().unwrap()).collect();

        let c = Cuboid::new(x[0], x[1], y[0], y[1], z[0], z[1]);

        if init && !c.is_init() {
            break;
        }

        let ln = neg.len();
        for p in &pos {
            match c.intersection(p) {
                Some(i) => neg.push(i),
                None => (),
            }
        }
        for n in &neg[..ln] {
            match c.intersection(n) {
                Some(i) => pos.push(i),
                None => (),
            }
        }
        if on {
            pos.push(c);
        }
    }

    let mut tot = 0;
    for p in &pos {
        tot += p.volume();
    }
    for n in &neg {
        tot -= n.volume();
    }
    println!("{}", tot);
}

pub fn part1() {
    solve(true);
}

pub fn part2() {
    solve(false);
}
