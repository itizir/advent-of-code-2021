#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day24() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

// decompile the provided code 'by hand'...
fn valid(digits: &[i64; 14], a: &[i64; 14], b: &[i64; 14]) -> bool {
    let mut z = 0;
    for i in 0..14 {
        let remainder = z % 26;
        if a[i] <= 0 {
            z /= 26;
        }
        if digits[i] != remainder + a[i] {
            z *= 26;
            z += digits[i] + b[i];
        }
    }
    z == 0
}

fn from_digits(digits: &[i64; 14]) -> i64 {
    let mut num = 0;
    for d in digits {
        num *= 10;
        num += d;
    }
    num
}

fn solve(for_min: bool) {
    let input = input();
    let lines: Vec<_> = input.lines().collect();

    let mut a: [i64; 14] = [0; 14];
    let mut b: [i64; 14] = [0; 14];
    for (i, chunk) in lines.chunks(18).enumerate() {
        a[i] = chunk[5].strip_prefix("add x ").unwrap().parse().unwrap();
        b[i] = chunk[15].strip_prefix("add y ").unwrap().parse().unwrap();
    }

    let mut digits = if for_min { [1; 14] } else { [9; 14] };
    let mut v = Vec::new();
    for i in 0..14 {
        if a[i] > 9 {
            v.push((i, b[i]));
        } else if a[i] <= 0 {
            let p = v.pop().unwrap();
            if for_min {
                while digits[p.0] + p.1 + a[i] < 1 {
                    digits[p.0] += 1;
                }
            } else {
                while digits[p.0] + p.1 + a[i] > 9 {
                    digits[p.0] -= 1;
                }
            }
            digits[i] = digits[p.0] + p.1 + a[i];
        } else {
            panic!("unexpected parameter value");
        }
    }

    assert_eq!(true, valid(&digits, &a, &b));

    println!("{}", from_digits(&digits));
}

pub fn part1() {
    solve(false);
}

pub fn part2() {
    solve(true);
}
