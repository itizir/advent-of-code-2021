#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day17() {
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

fn _x(v: i64, t: i64) -> i64 {
    let t = if t > v + 1 { v + 1 } else { t };
    t * v - t * (t - 1) / 2
}

fn y(v: i64, t: i64) -> i64 {
    t * v - t * (t - 1) / 2
}

fn vy_ok(v: i64, ymin: i64, ymax: i64) -> bool {
    let (t0, t1) = ty(v, ymin, ymax);
    t0 <= t1
}

fn ty(v: i64, ymin: i64, ymax: i64) -> (i64, i64) {
    let v = v as f64;
    let ymin = ymin as f64;
    let ymax = ymax as f64;
    let t = |c: f64| (v + 0.5) + ((v + 0.5).powi(2) - 2. * c).sqrt();
    (t(ymax).ceil() as i64, t(ymin).floor() as i64)
}

fn vx(t: i64, xmin: i64, xmax: i64) -> (i64, i64) {
    let t = t as f64;
    let xmin = xmin as f64;
    let xmax = xmax as f64;
    let v = |x: f64| {
        let mut v = x / t + 0.5 * (t - 1.);
        if v <= t - 1. {
            v = 0.5 * (1. + 8. * x).sqrt() - 0.5;
        }
        v
    };
    (v(xmin).ceil() as i64, v(xmax).floor() as i64)
}

fn target() -> (i64, i64, i64, i64) {
    let input = input();
    let mut line = input.lines().next().unwrap();

    line = line.strip_prefix("target area: x=").unwrap();
    let bounds: Vec<Vec<i64>> = line
        .split(", y=")
        .map(|r| {
            r.split("..")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    (bounds[0][0], bounds[0][1], bounds[1][0], bounds[1][1])
}

pub fn part1() {
    let (_, _, ymin, ymax) = target();
    let lim = 2 * ymin.abs();

    let mut vmax = 0;
    for v in 0..lim {
        if vy_ok(v, ymin, ymax) {
            vmax = v;
        }
    }

    println!("{}", y(vmax, vmax));
}

pub fn part2() {
    let (xmin, xmax, ymin, ymax) = target();
    let lim = 2 * ymin.abs();

    let mut nb_init = 0;
    for v in -lim..lim {
        let (ty0, ty1) = ty(v, ymin, ymax);
        let mut unique: HashSet<i64> = HashSet::new();
        for t in ty0..=ty1 {
            let (vx0, vx1) = vx(t, xmin, xmax);
            for vx in vx0..=vx1 {
                unique.insert(vx);
            }
        }
        nb_init += unique.len();
    }

    println!("{}", nb_init);
}
