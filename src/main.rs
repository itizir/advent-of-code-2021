use reqwest::header::COOKIE;
use std::fs;

mod secret;

mod day01;
mod day02;
mod day03;
mod day04;

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
