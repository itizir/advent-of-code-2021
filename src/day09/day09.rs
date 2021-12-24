#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day09() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

fn is_low(lines: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let v = lines[i][j];
    if i > 0 && v >= lines[i - 1][j] {
        return false;
    }
    if i < lines.len() - 1 && v >= lines[i + 1][j] {
        return false;
    }
    if j > 0 && v >= lines[i][j - 1] {
        return false;
    }
    if j < lines[i].len() - 1 && v >= lines[i][j + 1] {
        return false;
    }
    return true;
}

pub fn part1() {
    let input = input();

    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();

    let mut risk = 0;
    for (i, l) in lines.iter().enumerate() {
        for (j, v) in l.iter().enumerate() {
            if is_low(&lines, i, j) {
                risk += 1 + *v as u64;
            }
        }
    }

    println!("{}", risk);
}

pub fn part2() {
    let input = input();

    let w = input.lines().next().unwrap().len();
    let lines: Vec<u8> = input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .flatten()
        .collect();

    let mut ids: Vec<i64> = vec![-1; lines.len()];
    let mut sizes: Vec<i64> = vec![0; lines.len()];

    for (i, c) in lines.iter().enumerate() {
        if *c == 9 {
            continue;
        }
        union(&mut ids, &mut sizes, w, i);
    }

    let mut max = (0, 0, 0);
    for s in sizes {
        if s > max.0 {
            max.2 = max.1;
            max.1 = max.0;
            max.0 = s;
        } else if s > max.1 {
            max.2 = max.1;
            max.1 = s;
        } else if s > max.2 {
            max.2 = s;
        }
    }

    println!("{}", max.0 * max.1 * max.2);
}

fn find(ids: &mut Vec<i64>, pt: usize) -> i64 {
    if ids[pt] == -1 {
        return -1;
    }
    let mut id = pt as i64;
    while id != ids[id as usize] {
        id = ids[id as usize];
    }

    let mut prev = pt as i64;
    while prev != id {
        let tmp = ids[prev as usize];
        ids[prev as usize] = id;
        prev = tmp;
    }

    id
}

fn union(ids: &mut Vec<i64>, sizes: &mut Vec<i64>, w: usize, i: usize) {
    let id_a = if i % w == 0 { -1 } else { find(ids, i - 1) };
    let id_b = if i / w == 0 { -1 } else { find(ids, i - w) };

    if id_a == -1 && id_b == -1 {
        ids[i] = i as i64;
        sizes[i] = 1;
    } else if id_a == -1 || id_a == id_b {
        ids[i] = id_b;
        sizes[id_b as usize] += 1;
    } else if id_b == -1 {
        ids[i] = id_a;
        sizes[id_a as usize] += 1;
    } else if sizes[id_a as usize] < sizes[id_b as usize] {
        ids[id_a as usize] = id_b;
        ids[i] = id_b;
        sizes[id_b as usize] += sizes[id_a as usize] + 1;
    } else {
        ids[id_b as usize] = id_a;
        ids[i] = id_a;
        sizes[id_a as usize] += sizes[id_b as usize] + 1;
    }
}
