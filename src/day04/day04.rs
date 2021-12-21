#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day04() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    rows: Vec<HashSet<u8>>,
    columns: Vec<HashSet<u8>>,
    all: HashSet<u8>,
}
impl Board {
    fn mark(self: &mut Self, num: u8) -> bool {
        if !self.all.remove(&num) {
            return false;
        }
        for row in self.rows.iter_mut() {
            row.remove(&num);
            if row.len() == 0 {
                return true;
            }
        }
        for column in self.columns.iter_mut() {
            column.remove(&num);
            if column.len() == 0 {
                return true;
            }
        }
        return false;
    }
    fn sum(self: &Self) -> u64 {
        let mut sum = 0;
        for num in self.all.iter() {
            sum += *num as u64;
        }
        return sum;
    }
}

fn parse() -> (Vec<u8>, Vec<Board>) {
    let input = input();
    let mut lines = input.lines();

    let mut called = Vec::new();
    for num in lines.next().unwrap().split(",") {
        called.push(num.parse().unwrap());
    }
    lines.next();

    let mut boards = Vec::new();
    loop {
        let mut board = Board {
            rows: Vec::new(),
            columns: Vec::new(),
            all: HashSet::new(),
        };
        for i in 0..5 {
            let mut row = HashSet::new();
            for (j, cell) in lines.next().unwrap().split_whitespace().enumerate() {
                let num = u8::from_str_radix(cell.trim(), 10).unwrap();
                board.all.insert(num);
                row.insert(num);
                if i == 0 {
                    board.columns.push(HashSet::new());
                }
                board.columns[j].insert(num);
            }
            board.rows.push(row);
        }
        boards.push(board);
        if lines.next().is_none() {
            break;
        }
    }

    (called, boards)
}

pub fn part1() {
    let (called, mut boards) = parse();
    for num in called.iter() {
        for b in boards.iter_mut() {
            if b.mark(*num) {
                println!("{}", *num as u64 * b.sum());
                return;
            }
        }
    }
}

pub fn part2() {
    let (called, mut boards) = parse();
    for num in called.iter() {
        let boards_left = boards.len();
        let mut won = Vec::new();
        for (i, b) in boards.iter_mut().enumerate() {
            if b.mark(*num) {
                if boards_left == 1 {
                    println!("{}", *num as u64 * b.sum());
                    return;
                }
                won.push(i);
            }
        }
        won.reverse();
        for i in won {
            boards.remove(i);
        }
    }
}
