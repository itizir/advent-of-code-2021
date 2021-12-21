#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day03() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

fn power() -> (u128, u128) {
    let input = input();
    let mut counter: u128 = 0;
    let mut num_lines: u128 = 0;
    for line in input.lines() {
        num_lines += 1;
        let num_bits = line.len();
        for (i, c) in line.bytes().enumerate() {
            counter += u128::from(c - b'0') << ((num_bits - i - 1) * 10)
        }
    }

    let majority = num_lines / 2;
    let mut i = 0;
    let mut gamma = 0;
    while counter > 0 {
        gamma += ((counter % 1024) / majority) << i;
        counter /= 1024;
        i += 1;
    }
    let mask = (1 << i) - 1;
    let epsilon = !gamma & mask;

    (gamma, epsilon)
}

pub fn part1() {
    let (gamma, epsilon) = power();
    println!("{}", gamma * epsilon);
}

pub fn part2() {
    let input = input();

    let mut values = Vec::new();
    let mut num_bits = 0;
    for line in input.lines() {
        num_bits = line.len();
        let num = u128::from_str_radix(line, 2).unwrap();
        values.push(num);
    }
    values.sort();

    let find = |comp: fn(usize, usize) -> bool| {
        let mut values = &values[..];
        let mut target = 1 << (num_bits - 1);
        for i in (0..num_bits).rev() {
            let pivot = match values.binary_search(&target) {
                Ok(i) => i,
                Err(i) => i,
            };
            if comp(pivot, values.len() / 2) {
                values = &values[..pivot];
                target -= 1 << i;
            } else {
                values = &values[pivot..];
            }
            if values.len() == 1 {
                return values[0];
            }
            target += 1 << (i - 1);
        }
        return 0;
    };

    let oxygen = find(|a, b| a > b);
    let co2 = find(|a, b| a <= b);

    println!("{}", oxygen * co2);
}
