#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day19() {
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

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn diffs(&self, other: &Self) -> (i64, i64, i64) {
        let mut d = vec![
            (self.x - other.x).abs(),
            (self.y - other.y).abs(),
            (self.z - other.z).abs(),
        ];
        d.sort();
        (d[2], d[1], d[0])
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromIterator<i64> for Point {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        let pt = Point {
            x: i.next().unwrap(),
            y: i.next().unwrap(),
            z: i.next().unwrap(),
        };
        assert_eq!(true, i.next().is_none());
        pt
    }
}

#[derive(Debug)]
struct Points {
    v: Vec<Point>,
    diffs: HashMap<(i64, i64, i64), (usize, usize)>,
}

impl std::ops::Index<usize> for Points {
    type Output = Point;
    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i]
    }
}

impl std::ops::IndexMut<usize> for Points {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

impl std::ops::AddAssign<Point> for &mut Points {
    fn add_assign(&mut self, pt: Point) {
        for i in 0..self.len() {
            self[i] += pt;
        }
    }
}

impl Points {
    fn new() -> Points {
        Points {
            v: Vec::new(),
            diffs: HashMap::new(),
        }
    }

    fn push(&mut self, pt: Point) {
        self.v.push(pt);
        for i in 0..self.len() - 1 {
            self.diffs.insert(pt.diffs(&self[i]), (i, self.len() - 1));
        }
    }

    fn len(&self) -> usize {
        self.v.len()
    }

    fn overlap(&self, other: &Self) -> Option<(usize, usize, usize, usize)> {
        let min_match = 12;
        let mut matches = 0;
        let mut first = (0, 0, 0, 0);

        for (k, v) in &self.diffs {
            match other.diffs.get(k) {
                Some(w) => {
                    if matches == 0 {
                        first = (v.0, v.1, w.0, w.1);
                    }
                    matches += 1;
                }
                None => (),
            }
        }

        if matches >= min_match * (min_match - 1) / 2 {
            return Some(first);
        }
        None
    }

    fn align(&self, mut other: &mut Self) -> (bool, Point) {
        let ov = match self.overlap(other) {
            Some(val) => val,
            None => return (false, Point::new(0, 0, 0)),
        };

        let d = self[ov.1] - self[ov.0];
        let e = other[ov.3] - other[ov.2];

        for &r in ROTATIONS {
            if d == r(e) {
                other.rotate(r);
                let pt = self[ov.0] - other[ov.2];
                other += pt;
                return (true, pt);
            }
        }

        let e = other[ov.2] - other[ov.3];
        for &r in ROTATIONS {
            if d == r(e) {
                other.rotate(r);
                let pt = self[ov.0] - other[ov.3];
                other += pt;
                return (true, pt);
            }
        }

        panic!("should have found the match");
    }

    fn rotate(&mut self, f: fn(Point) -> Point) {
        for i in 0..self.len() {
            self[i] = f(self[i]);
        }
    }
}

static ROTATIONS: &[fn(Point) -> Point] = &[
    |pt: Point| Point::new(pt.x, pt.y, pt.z),
    |pt: Point| Point::new(pt.x, -pt.y, -pt.z),
    |pt: Point| Point::new(pt.x, pt.z, -pt.y),
    |pt: Point| Point::new(pt.x, -pt.z, pt.y),
    |pt: Point| Point::new(pt.y, pt.z, pt.x),
    |pt: Point| Point::new(pt.y, -pt.z, -pt.x),
    |pt: Point| Point::new(pt.y, pt.x, -pt.z),
    |pt: Point| Point::new(pt.y, -pt.x, pt.z),
    |pt: Point| Point::new(pt.z, pt.x, pt.y),
    |pt: Point| Point::new(pt.z, -pt.x, -pt.y),
    |pt: Point| Point::new(pt.z, pt.y, -pt.x),
    |pt: Point| Point::new(pt.z, -pt.y, pt.x),
    |pt: Point| Point::new(-pt.x, -pt.y, pt.z),
    |pt: Point| Point::new(-pt.x, pt.y, -pt.z),
    |pt: Point| Point::new(-pt.x, -pt.z, -pt.y),
    |pt: Point| Point::new(-pt.x, pt.z, pt.y),
    |pt: Point| Point::new(-pt.y, -pt.z, pt.x),
    |pt: Point| Point::new(-pt.y, pt.z, -pt.x),
    |pt: Point| Point::new(-pt.y, -pt.x, -pt.z),
    |pt: Point| Point::new(-pt.y, pt.x, pt.z),
    |pt: Point| Point::new(-pt.z, -pt.x, pt.y),
    |pt: Point| Point::new(-pt.z, pt.x, -pt.y),
    |pt: Point| Point::new(-pt.z, -pt.y, -pt.x),
    |pt: Point| Point::new(-pt.z, pt.y, pt.x),
];

fn get_two(v: &mut Vec<Points>, i: usize, j: usize) -> (&mut Points, &mut Points) {
    if i < j {
        let (a, b) = v.split_at_mut(j);
        return (&mut a[i], &mut b[0]);
    }
    let (a, b) = v.split_at_mut(i);
    (&mut b[0], &mut a[j])
}

fn solve() -> (Vec<Points>, Vec<Point>) {
    let input = input();
    let mut lines = input.lines();

    let mut scans: Vec<Points> = Vec::new();
    while lines.next().is_some() {
        let mut beacons = Points::new();
        for line in &mut lines {
            if line == "" {
                break;
            }
            let pt: Point = line.split(",").map(|n| n.parse().unwrap()).collect();
            beacons.push(pt);
        }
        scans.push(beacons);
    }

    let mut refs = vec![0];
    let mut i = 0;

    let mut scan_pos = vec![Point::new(0, 0, 0)];

    loop {
        if i >= refs.len() {
            break;
        }
        for j in 0..scans.len() {
            if refs.contains(&j) {
                continue;
            };
            let (a, b) = get_two(&mut scans, refs[i], j);
            let (aligned, pt) = a.align(b);
            if aligned {
                refs.push(j);
                scan_pos.push(pt);
            }
        }
        i += 1;
    }

    (scans, scan_pos)
}

pub fn part1() {
    let (scans, _) = solve();

    let mut hs = HashSet::new();
    for s in scans {
        for pt in s.v {
            hs.insert(pt);
        }
    }

    println!("{}", hs.len());
}

pub fn part2() {
    let (_, pos) = solve();

    let mut max = 0;
    for i in 0..pos.len() - 1 {
        for j in i + 1..pos.len() {
            let d = pos[j] - pos[i];
            let m = d.x.abs() + d.y.abs() + d.z.abs();
            if m > max {
                max = m;
            }
        }
    }

    println!("{}", max);
}
