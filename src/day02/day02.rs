#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day02() {
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

    struct Position {
        depth: u64,
        distance: u64,
    }

    let mut pos = Position {
        depth: 0,
        distance: 0,
    };

    for line in input.lines() {
        let split = line.split_once(" ").unwrap();
        let step: u64 = split.1.parse().unwrap();

        match split.0 {
            "forward" => pos.distance += step,
            "down" => pos.depth += step,
            "up" => pos.depth -= step,
            _ => (),
        }
    }
    println!("{}", pos.depth * pos.distance);
}

pub fn part2() {
    let input = input();

    struct Position {
        depth: u64,
        distance: u64,
        aim: u64,
    }

    let mut pos = Position {
        depth: 0,
        distance: 0,
        aim: 0,
    };

    for line in input.lines() {
        let split = line.split_once(" ").unwrap();
        let step: u64 = split.1.parse().unwrap();

        match split.0 {
            "forward" => {
                pos.distance += step;
                pos.depth += step * pos.aim;
            }
            "down" => pos.aim += step,
            "up" => pos.aim -= step,
            _ => (),
        }
    }
    println!("{}", pos.depth * pos.distance);
}
