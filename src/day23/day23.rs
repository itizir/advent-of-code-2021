#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day23() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}
const PART_2: [&str; 2] = ["  #D#C#B#A#", "  #D#B#A#C#"];

use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

fn calc_dist(i: usize, j: usize, n: usize) -> usize {
    if i == j {
        return 0;
    }
    let (i, j) = (min(i, j), max(i, j));
    let c = 4 * n;
    if i < c && j < c {
        if i / n == j / n {
            j - i
        } else {
            (2 * n - j % n - i % n) + 2 * (j / n - i / n)
        }
    } else if i < c {
        if j >= i / n + (c + 2) {
            (n - i % n) + 2 * (j - i / n - (c + 1)) - 1 - (j == c + 6) as usize
        } else {
            (n - i % n) + 2 * (i / n + (c + 2) - j) - 1 - (j == c) as usize
        }
    } else {
        2 * (j - i) - (i == c) as usize - (j == c + 6) as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Occupant {
    A,
    B,
    C,
    D,
    None,
}

impl std::fmt::Display for Occupant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Occupant::A => 'A',
                Occupant::B => 'B',
                Occupant::C => 'C',
                Occupant::D => 'D',
                Occupant::None => '.',
            }
        )
    }
}

impl Occupant {
    fn empty(&self) -> bool {
        *self == Occupant::None
    }

    fn goal_id(&self) -> usize {
        match self {
            Occupant::A => 0,
            Occupant::B => 1,
            Occupant::C => 2,
            Occupant::D => 3,
            _ => panic!("should not call this on empty cell"),
        }
    }

    fn cost(&self) -> u64 {
        match self {
            Occupant::A => 1,
            Occupant::B => 10,
            Occupant::C => 100,
            Occupant::D => 1000,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Burrow<const N: usize> {
    cells: [Occupant; N],
    cost: u64,
    h: u64,
}

impl<const N: usize> Ord for Burrow<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.h).cmp(&(self.cost + self.h))
    }
}

impl<const N: usize> PartialOrd for Burrow<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> std::fmt::Display for Burrow<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = (N - 7) / 4;
        let mut st = format!(
            "\n#############\n#{}{}.{}.{}.{}.{}{}#\n###{}#{}#{}#{}###\n",
            self.cells[N - 7],
            self.cells[N - 6],
            self.cells[N - 5],
            self.cells[N - 4],
            self.cells[N - 3],
            self.cells[N - 2],
            self.cells[N - 1],
            self.cells[n - 1],
            self.cells[2 * n - 1],
            self.cells[3 * n - 1],
            self.cells[4 * n - 1]
        );
        for i in 0..n - 1 {
            st += format!(
                "  #{}#{}#{}#{}#\n",
                self.cells[n - 2 - i],
                self.cells[2 * n - 2 - i],
                self.cells[3 * n - 2 - i],
                self.cells[4 * n - 2 - i],
            )
            .as_str();
        }
        st += format!(
            "  #########\ncost: {}\nmin to goal: {}\n",
            self.cost, self.h
        )
        .as_str();
        write!(f, "{}", st)
    }
}

impl<const N: usize> Burrow<N> {
    fn new(a: &Vec<usize>, b: &Vec<usize>, c: &Vec<usize>, d: &Vec<usize>) -> Burrow<N> {
        let mut cells = [Occupant::None; N];
        for a in a {
            cells[*a] = Occupant::A;
        }
        for b in b {
            cells[*b] = Occupant::B;
        }
        for c in c {
            cells[*c] = Occupant::C;
        }
        for d in d {
            cells[*d] = Occupant::D;
        }
        let mut b = Burrow {
            cells,
            cost: 0,
            h: 0,
        };
        b.h = b.heuristics(&Burrow::distances());
        b
    }

    fn distances() -> [[u64; N]; N] {
        let mut distances = [[0; N]; N];
        for i in 0..N {
            for j in 0..N {
                let d = calc_dist(i, j, (N - 7) / 4) as u64;
                distances[i][j] = d;
                distances[j][i] = d;
            }
        }
        distances
    }

    fn is_parked(&self, i: usize) -> bool {
        if i >= N - 7 {
            return false;
        }
        let n = (N - 7) / 4;
        if self.cells[i].goal_id() != i / n {
            return false;
        }
        for j in i - i % n..i {
            if self.cells[j] != self.cells[j + 1] {
                return false;
            }
        }
        true
    }

    fn destinations(&self, i: usize) -> Option<Vec<usize>> {
        if self.cells[i].empty() || self.is_parked(i) {
            return None;
        }
        let mut dest = Vec::new();
        if i < N - 7 {
            let n = (N - 7) / 4;
            for j in 1..n - i % n {
                if !self.cells[i + j].empty() {
                    return None;
                }
            }
            self.push_free_corridor(i, &mut dest);
        } else {
            self.push_free_goal(i, &mut dest);
        }
        if dest.len() == 0 {
            None
        } else {
            Some(dest)
        }
    }

    fn push_free_corridor(&self, i: usize, dest: &mut Vec<usize>) {
        let n = (N - 7) / 4;
        for c in (N - 7..N - 5 + i / n).rev() {
            if !self.cells[c].empty() {
                break;
            }
            dest.push(c);
        }
        for c in N - 5 + i / n..N {
            if !self.cells[c].empty() {
                break;
            }
            dest.push(c);
        }
    }

    fn push_free_goal(&self, i: usize, dest: &mut Vec<usize>) {
        let n = (N - 7) / 4;

        let mut goal = n * self.cells[i].goal_id();
        for g in goal + 1..goal + n {
            if !self.cells[goal].empty() {
                if self.cells[goal] != self.cells[i] {
                    return;
                }
                goal += 1;
            } else if !self.cells[g].empty() {
                return;
            }
        }
        if !self.cells[goal].empty() {
            return;
        }

        let entrance = goal / n + N - 5;
        let corridor = if i < entrance {
            i + 1..entrance
        } else {
            entrance..i
        };
        for c in corridor {
            if !self.cells[c].empty() {
                return;
            }
        }

        dest.push(goal);
    }

    fn mov(&self, i: usize, j: usize, dist: &[[u64; N]; N]) -> Burrow<N> {
        let cost = self.cells[i].cost() * dist[i][j];
        let mut b = self.clone();
        b.cells[j] = b.cells[i];
        b.h += b.single_h(j, dist);
        b.h -= b.single_h(i, dist);
        b.cells[i] = Occupant::None;
        b.cost += cost;
        b
    }

    fn done(&self) -> bool {
        for i in 0..N {
            if !self.cells[i].empty() && !self.is_parked(i) {
                return false;
            }
        }
        true
    }

    fn heuristics(&self, dist: &[[u64; N]; N]) -> u64 {
        let mut h = 0;
        for i in 0..N {
            if self.cells[i].empty() {
                continue;
            }
            h += self.single_h(i, dist);
        }
        let n = (N as u64 - 7) / 4;
        h -= 1111 * (n - 1) * n / 2;
        h
    }

    fn single_h(&self, i: usize, dist: &[[u64; N]; N]) -> u64 {
        let n = (N - 7) / 4;
        let j = n * self.cells[i].goal_id();
        self.cells[i].cost() * dist[i][j]
    }

    fn solve() {
        assert_eq!(true, N == 15 || N == 23);

        let input = input();
        let mut lines: Vec<_> = input.lines().collect();
        if N == 23 {
            for p in PART_2.iter().rev() {
                lines.insert(3, p);
            }
        }
        let mut lines = lines.iter();

        lines.next();
        lines.next();

        let mut rooms: Vec<Vec<_>> = Vec::new();
        for line in lines {
            let line: Vec<_> = line
                .trim()
                .trim_matches('#')
                .split('#')
                .map(|s| s.chars().next().unwrap_or('#'))
                .collect();
            if line.len() < 4 {
                break;
            }
            rooms.push(line);
        }
        let n = rooms.len();
        assert_eq!(n, (N - 7) / 4);

        let mut start: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, r) in rooms.iter().rev().enumerate() {
            for (j, c) in r.iter().enumerate() {
                start.entry(*c).or_insert(Vec::new()).push(n * j + i);
            }
        }

        let b = Burrow::<N>::new(
            start.get(&'A').unwrap(),
            start.get(&'B').unwrap(),
            start.get(&'C').unwrap(),
            start.get(&'D').unwrap(),
        );

        let mut burrows = BinaryHeap::new();
        let mut seen = HashSet::new();
        let dist = Burrow::distances();

        burrows.push(b);

        let mut min = u64::MAX;
        while let Some(b) = burrows.pop() {
            if b.cost + b.h > min {
                continue;
            }
            if seen.contains(&b) {
                continue;
            }
            seen.insert(b);
            for i in 0..N {
                match b.destinations(i) {
                    Some(dest) => {
                        for d in dest {
                            let b = b.mov(i, d, &dist);
                            if b.cost + b.h < min {
                                if b.done() {
                                    min = b.cost + b.h;
                                } else {
                                    burrows.push(b);
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
        }

        println!("{}", min);
    }
}

pub fn part1() {
    Burrow::<15>::solve();
}

pub fn part2() {
    Burrow::<23>::solve();
}
