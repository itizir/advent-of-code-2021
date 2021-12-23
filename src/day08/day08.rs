#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day08() {
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
    let input = input();

    let mut count = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" | ").collect();
        let lengths: Vec<usize> = parts[1].split_whitespace().map(|a| a.len()).collect();
        for l in lengths {
            count += match l {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }
    }

    println!("{}", count);
}

pub fn part2() {
    let input = input();

    let mut total = 0;
    for line in input.lines() {
        total += decode(line);
    }

    println!("{}", total);
}

fn decode(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(" | ").collect();

    let mut patterns: Vec<&str> = parts[0].split_whitespace().collect();
    patterns.sort_by_key(|p| p.len());
    let output: Vec<&str> = parts[1].split_whitespace().collect();

    let mut number = 0;
    for digit in output.iter() {
        number *= 10;
        number += match digit.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 => {
                if contains(digit, patterns[0]) {
                    3
                } else if contains(digit, difference(patterns[2], patterns[0]).as_str()) {
                    5
                } else {
                    2
                }
            }
            6 => {
                if contains(digit, patterns[2]) {
                    9
                } else if contains(digit, patterns[0]) {
                    0
                } else {
                    6
                }
            }
            _ => panic!("oops"),
        }
    }
    number
}

fn intersection(a: &str, b: &str) -> String {
    let mut tmp = String::from(a);
    tmp.retain(|c| b.contains(c));
    tmp
}

fn difference(a: &str, b: &str) -> String {
    let mut tmp = String::from(a);
    tmp.retain(|c| !b.contains(c));
    tmp
}

fn contains(a: &str, b: &str) -> bool {
    intersection(b, a).eq(b)
}
