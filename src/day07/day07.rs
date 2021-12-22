#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day07() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

fn crabs() -> Vec<i64> {
    let input = input();
    let mut crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|a| a.parse().unwrap())
        .collect();
    crabs.sort();
    crabs
}

pub fn part1() {
    let crabs = crabs();

    let mut fuel: i64 = crabs.iter().sum();
    let num_crabs = crabs.len() as i64;

    let mut prev = 0;
    let mut min_fuel = fuel;
    for (i, &c) in crabs.iter().enumerate() {
        fuel -= (num_crabs - 2 * i as i64) * (c - prev);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
        prev = c;
    }

    println!("{}", min_fuel);
}

pub fn part2() {
    let crabs = crabs();

    let calc_fuel = |pos: i64| {
        let mut fuel = 0;
        for c in crabs.iter() {
            let delta = (c - pos).abs();
            fuel += delta * (delta + 1) / 2;
        }
        fuel
    };

    let mut min_fuel = calc_fuel(0);
    // no need to narrow bounds down: runs fast enough...
    for pos in crabs[0]..crabs[crabs.len() - 1] {
        let fuel = calc_fuel(pos);
        if min_fuel > fuel {
            min_fuel = fuel;
        }
    }

    println!("{}", min_fuel);
}
