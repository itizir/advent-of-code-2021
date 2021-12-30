#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day21() {
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

    let mut p1 = (lines.next().unwrap().bytes().last().unwrap() - b'0') as u64;
    let mut p2 = (lines.next().unwrap().bytes().last().unwrap() - b'0') as u64;

    let mut die = 6;
    let mut rolls = 0;

    let mut s1 = 0;
    let mut s2 = 0;

    let result = loop {
        p1 = ((p1 - 1 + die) % 10) + 1;
        s1 += p1;
        rolls += 3;
        die += 9;
        if s1 >= 1000 {
            break s2 * rolls;
        }
        p2 = ((p2 - 1 + die) % 10) + 1;
        s2 += p2;
        rolls += 3;
        die += 9;
        if s2 >= 1000 {
            break s1 * rolls;
        }
    };

    println!("{}", result);
}

pub fn part2() {
    let input = input();
    let mut lines = input.lines();

    let p1 = (lines.next().unwrap().bytes().last().unwrap() - b'0') as u64;
    let p2 = (lines.next().unwrap().bytes().last().unwrap() - b'0') as u64;

    let mut w1 = 0;
    let mut w2 = 0;

    next(&mut w1, &mut w2, p1, p2, 0, 0, 1);

    println!("{}", if w1 > w2 { w1 } else { w2 });
}

fn next(
    w_current: &mut u64,
    w_other: &mut u64,
    p_current: u64,
    p_other: u64,
    s_current: u64,
    s_other: u64,
    m: u64,
) {
    for d in 3..=9 {
        let m = m * match d {
            3 | 9 => 1,
            4 | 8 => 3,
            5 | 7 => 6,
            6 => 7,
            _ => 0,
        };
        let p = ((p_current - 1 + d) % 10) + 1;
        let s = s_current + p;
        if s >= 21 {
            *w_current += m;
        } else {
            next(w_other, w_current, p_other, p, s_other, s, m);
        }
    }
}
