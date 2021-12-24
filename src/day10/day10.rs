#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day10() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

fn is_open(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn is_matching(open: char, close: char) -> bool {
    (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
        || (open == '<' && close == '>')
}

fn corrupt_points(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_points(remain: String) -> u64 {
    let mut remain = remain.clone();
    remain = remain.replace("(", "1");
    remain = remain.replace("[", "2");
    remain = remain.replace("{", "3");
    remain = remain.replace("<", "4");
    remain = String::from_iter(remain.chars().rev());
    u64::from_str_radix(remain.as_str(), 5).unwrap()
}

fn validate(line: &str) -> (char, String) {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else if stack.len() == 0 || !is_matching(stack.pop().unwrap(), c) {
            return (c, String::new());
        }
    }

    return (' ', String::from_iter(stack));
}

pub fn part1() {
    let input = input();

    let mut score = 0;
    for line in input.lines() {
        let (c, _) = validate(line);
        score += corrupt_points(c);
    }

    println!("{}", score);
}

pub fn part2() {
    let input = input();

    let mut scores = Vec::new();
    for line in input.lines() {
        let (_, remain) = validate(line);
        if remain.len() == 0 {
            continue;
        }
        scores.push(incomplete_points(remain));
    }
    scores.sort();

    println!("{}", scores[scores.len() / 2]);
}
