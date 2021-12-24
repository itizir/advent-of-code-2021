#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day11() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

use std::collections::HashSet;

fn neighbours(i: usize, w: usize, h: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let left = i % w > 0;
    let right = i % w < w - 1;
    if i / w > 0 {
        if left {
            v.push(i - w - 1);
        }
        v.push(i - w);
        if right {
            v.push(i - w + 1);
        }
    }
    if left {
        v.push(i - 1);
    }
    if right {
        v.push(i + 1);
    }
    if i / w < h - 1 {
        if left {
            v.push(i + w - 1);
        }
        v.push(i + w);
        if right {
            v.push(i + w + 1);
        }
    }
    v
}

fn step(dumbos: &mut Vec<HashSet<usize>>, w: usize, h: usize) -> usize {
    dumbos.rotate_right(1);
    let l = dumbos.len();

    let mut count = 0;
    while dumbos[l - 1].len() != 0 {
        let flash: Vec<usize> = dumbos[l - 1].drain().collect();
        count += flash.len();
        for f in flash.iter() {
            for n in neighbours(*f, w, h) {
                for (i, d) in dumbos[1..l - 1].iter_mut().enumerate() {
                    match d.take(&n) {
                        Some(_) => {
                            dumbos[i + 2].insert(n);
                            break;
                        }
                        None => (),
                    }
                }
            }
        }
        dumbos[0].extend(flash);
    }
    count
}

fn dumbos() -> (Vec<HashSet<usize>>, usize, usize) {
    let input = input();

    let w = input.lines().next().unwrap().len();
    let starting: Vec<usize> = input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| (b - b'0') as usize)
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();
    let h = starting.len() / w;
    assert_eq!(0, starting.len() % w);

    let mut dumbos: Vec<HashSet<usize>> = Vec::new();

    for _ in 0..11 {
        dumbos.push(HashSet::new());
    }

    for (i, v) in starting.iter().enumerate() {
        dumbos[*v].insert(i);
    }

    (dumbos, w, h)
}

pub fn part1() {
    let (mut dumbos, w, h) = dumbos();

    let mut count = 0;
    for _ in 0..100 {
        count += step(&mut dumbos, w, h);
    }

    println!("{}", count);
}

pub fn part2() {
    let (mut dumbos, w, h) = dumbos();

    let mut i = 0;
    loop {
        i += 1;
        if step(&mut dumbos, w, h) == w * h {
            break;
        }
    }

    println!("{}", i);
}
