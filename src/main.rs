use reqwest::header::COOKIE;
use std::fs;

mod secret;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    println!("Welcome to Advent of Code 2021!");
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 && args[1] == "gen" {
        gen();
        return;
    }
    run_all();
}

fn run_all() {
    println!("day01-part1:");
    day01::day01::part1();
    println!("day01-part2:");
    day01::day01::part2();
    println!("day02-part1:");
    day02::day02::part1();
    println!("day02-part2:");
    day02::day02::part2();
    println!("day03-part1:");
    day03::day03::part1();
    println!("day03-part2:");
    day03::day03::part2();
    println!("day04-part1:");
    day04::day04::part1();
    println!("day04-part2:");
    day04::day04::part2();
    println!("day05-part1:");
    day05::day05::part1();
    println!("day05-part2:");
    day05::day05::part2();
    println!("day06-part1:");
    day06::day06::part1();
    println!("day06-part2:");
    day06::day06::part2();
    println!("day07-part1:");
    day07::day07::part1();
    println!("day07-part2:");
    day07::day07::part2();
    println!("day08-part1:");
    day08::day08::part1();
    println!("day08-part2:");
    day08::day08::part2();
    println!("day09-part1:");
    day09::day09::part1();
    println!("day09-part2:");
    day09::day09::part2();
    println!("day10-part1:");
    day10::day10::part1();
    println!("day10-part2:");
    day10::day10::part2();
    println!("day11-part1:");
    day11::day11::part1();
    println!("day11-part2:");
    day11::day11::part2();
    println!("day12-part1:");
    day12::day12::part1();
    println!("day12-part2:");
    day12::day12::part2();
    println!("day13-part1:");
    day13::day13::part1();
    println!("day13-part2:");
    day13::day13::part2();
    println!("day14-part1:");
    day14::day14::part1();
    println!("day14-part2:");
    day14::day14::part2();
    println!("day15-part1:");
    day15::day15::part1();
    println!("day15-part2:");
    day15::day15::part2();
    println!("day16-part1:");
    day16::day16::part1();
    println!("day16-part2:");
    day16::day16::part2();
    println!("day17-part1:");
    day17::day17::part1();
    println!("day17-part2:");
    day17::day17::part2();
    println!("day18-part1:");
    day18::day18::part1();
    println!("day18-part2:");
    day18::day18::part2();
    println!("day19-part1:");
    day19::day19::part1();
    println!("day19-part2:");
    day19::day19::part2();
    println!("day20-part1:");
    day20::day20::part1();
    println!("day20-part2:");
    day20::day20::part2();
    println!("day21-part1:");
    day21::day21::part1();
    println!("day21-part2:");
    day21::day21::part2();
    println!("day22-part1:");
    day22::day22::part1();
    println!("day22-part2:");
    day22::day22::part2();
    println!("day23-part1:");
    day23::day23::part1();
    println!("day23-part2:");
    day23::day23::part2();
    println!("day24-part1:");
    day24::day24::part1();
    println!("day24-part2:");
    day24::day24::part2();
    println!("day25-part1:");
    day25::day25::part1();
    println!("day25-part2:");
    day25::day25::part2();
}

fn gen() {
    println!("...generating skeleton for next day...");

    let mut main_file = String::from_utf8(fs::read("src/main.rs").unwrap()).unwrap();

    let mut i = main_file.find("mod day").unwrap();
    let mut day = 1;

    for j in (i..).step_by(11) {
        let mod_line = &main_file[j..j + 11];
        day = match mod_line.strip_prefix("mod day") {
            Some(val) => {
                let raw = val.strip_suffix(";\n").unwrap();
                raw.parse().unwrap()
            }
            None => {
                i = j - 11;
                break;
            }
        }
    }
    day += 1;

    if day > 25 {
        println!("all days done! ho-ho-ho");
        return;
    }

    let mut mid = main_file.split_off(i + 11);
    main_file.extend(format!("mod day{:02};\n", day).chars());

    i = 16
        + mid
            .find(format!("day{:02}::part2();", day - 1).as_str())
            .unwrap();
    let tail = mid.split_off(i);
    main_file.extend(mid.chars());
    main_file.extend(
        format!(
            r#"    println!("day{0:02}-part1:");
    day{0:02}::day{0:02}::part1();
    println!("day{0:02}-part2:");
    day{0:02}::day{0:02}::part2();
"#,
            day
        )
        .chars(),
    );

    main_file.extend(tail.chars());
    fs::write("src/main.rs", main_file).unwrap();

    let dirname = &format!("src/day{:02}", day);
    fs::create_dir(dirname).unwrap_or_default();
    fs::write(
        format!("{}/mod.rs", dirname),
        format!("pub mod day{:02};\n", day),
    )
    .unwrap();
    fs::write(
        format!("{}/day{:02}.rs", dirname, day),
        format!(
            r###"#[cfg(test)]
mod tests {{
    use super::*;
    #[test]
    fn day{:02}() {{
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }}
}}

fn input() -> String {{
    String::from(include_str!("input.txt"))
}}

pub fn part1() {{}}

pub fn part2() {{}}
"###,
            day
        ),
    )
    .unwrap();

    println!("...downloading input...");

    let client = reqwest::blocking::Client::new();
    let input = client
        .get(format!("https://adventofcode.com/2021/day/{}/input", day))
        .header(COOKIE, format!("session={}", secret::SESSION_COOKIE))
        .send()
        .unwrap()
        .text()
        .unwrap();

    fs::write(format!("{}/input.txt", dirname), input).unwrap();

    println!("done");
}
