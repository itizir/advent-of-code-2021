#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day25() {
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

    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut moves = 0;
    let mut moved = true;
    while moved {
        moves += 1;
        moved = false;

        let mut i = 0;
        while i < h {
            let mut first = false;
            let mut j = 0;
            while j < w {
                let c = lines[i][j];
                if c == '>' && lines[i][(j + 1) % w] == '.' {
                    if j == 0 {
                        first = true;
                    } else if first && j == w - 1 {
                        break;
                    }
                    lines[i][(j + 1) % w] = '>';
                    lines[i][j] = '.';
                    j += 1;
                    moved = true;
                }
                j += 1;
            }
            i += 1;
        }
        let mut j = 0;
        while j < w {
            let mut first = false;
            let mut i = 0;
            while i < h {
                let c = lines[i][j];
                if c == 'v' && lines[(i + 1) % h][j] == '.' {
                    if i == 0 {
                        first = true;
                    } else if first && i == h - 1 {
                        break;
                    }
                    lines[(i + 1) % h][j] = 'v';
                    lines[i][j] = '.';
                    i += 1;
                    moved = true;
                }
                i += 1;
            }
            j += 1;
        }
    }

    println!("{}", moves);
}

use std::collections::HashSet;

pub fn part2() {
    let input = input();

    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut right = HashSet::new();
    let mut down = HashSet::new();

    for i in 0..h {
        for j in 0..w {
            let c = lines[i][j];
            if c == '>' && lines[i][(j + 1) % w] == '.' {
                right.insert((i, j));
            } else if c == 'v' && lines[(i + 1) % h][j] == '.' {
                down.insert((i, j));
            }
        }
    }

    let mut count = 1;

    while right.len() > 0 || down.len() > 0 {
        count += 1;

        let mut next = HashSet::new();
        for pt in right {
            let i = pt.0;
            let j = pt.1;
            lines[i][j] = '.';
            lines[i][(j + 1) % w] = '>';
            if lines[i][(j + 2) % w] == '.' {
                next.insert((i, (j + 1) % w));
            }
            if lines[i][(j + w - 1) % w] == '>' {
                next.insert((i, (j + w - 1) % w));
            }
            if lines[(i + h - 1) % h][j] == 'v' {
                down.insert(((i + h - 1) % h, j));
            }
            down.remove(&((i + h - 1) % h, (j + 1) % w));
        }
        right = next;

        let mut next = HashSet::new();
        for pt in down {
            let i = pt.0;
            let j = pt.1;
            lines[i][j] = '.';
            lines[(i + 1) % h][j] = 'v';
            if lines[(i + 2) % h][j] == '.' {
                next.insert(((i + 1) % h, j));
            }
            if lines[(i + h - 1) % h][j] == 'v' {
                next.insert(((i + h - 1) % h, j));
            }
            if lines[i][(j + w - 1) % w] == '>' {
                right.insert((i, (j + w - 1) % w));
            }
            right.remove(&((i + 1) % h, (j + w - 1) % w));
        }
        down = next;
    }

    println!("{}", count);
}
