#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day01() {
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
    let mut lines = input.lines();

    let mut counter = 0;
    let mut a: u64 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        let current = line.parse().unwrap();
        if current > a {
            counter += 1;
        }
        a = current;
    }
    println!("{}", counter);
}

pub fn part2() {
    let input = input();
    let mut lines = input.lines();

    let mut counter = 0;
    let mut a: u64 = lines.next().unwrap().parse().unwrap();
    let mut b: u64 = lines.next().unwrap().parse().unwrap();
    let mut c: u64 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        let current = line.parse().unwrap();
        if current > a {
            counter += 1;
        }
        a = b;
        b = c;
        c = current;
    }
    println!("{}", counter);
}
