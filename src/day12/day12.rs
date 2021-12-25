#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day12() {
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
    solve(0);
}

pub fn part2() {
    solve(1);
}

fn solve(small_cave_allowance: u8) {
    let input = input();

    let mut caves: Vec<&str> = Vec::new();
    for line in input.lines() {
        let link: Vec<&str> = line.split("-").collect();
        caves.extend(link);
    }
    caves.sort_by_key(|k| match k {
        &"start" => "\u{126}",
        &"end" => "\u{127}",
        _ => *k,
    });
    caves.dedup();
    caves.reverse();

    let mut big_id = 0;
    for (i, &n) in caves.iter().enumerate() {
        if String::from(n).to_lowercase() != *n {
            big_id = i;
            break;
        }
    }

    let mut input = input.clone();
    for (i, n) in caves.iter().enumerate() {
        input = input.replace(n, &i.to_string());
    }

    let mut tunnels: Vec<Vec<usize>> = Vec::new();
    for _ in caves {
        tunnels.push(Vec::new());
    }

    for line in input.lines() {
        let c: Vec<usize> = line.split("-").map(|c| c.parse().unwrap()).collect();
        tunnels[c[0]].push(c[1]);
        tunnels[c[1]].push(c[0]);
    }

    let paths: Vec<Vec<usize>> = explore(1, big_id, vec![1], &tunnels, small_cave_allowance);

    println!("{}", paths.len());
}

fn explore(
    id: usize,
    big_id: usize,
    so_far: Vec<usize>,
    tunnels: &Vec<Vec<usize>>,
    small_cave_allowance: u8,
) -> Vec<Vec<usize>> {
    let mut paths: Vec<Vec<usize>> = Vec::new();
    for &c in tunnels[id].iter() {
        let mut sca = small_cave_allowance;
        if c < big_id && so_far.contains(&c) {
            if sca > 0 && c > 1 {
                sca -= 1;
            } else {
                continue;
            }
        }
        let mut so_far = so_far.clone();
        so_far.push(c);
        if c == 0 {
            paths.push(so_far);
        } else {
            paths.extend(explore(c, big_id, so_far, tunnels, sca));
        }
    }
    return paths;
}
