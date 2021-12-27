#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day16() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

fn parse_id(it: &mut dyn Iterator<Item = u8>) -> u64 {
    let mut id = 0;
    for d in it.take(3) {
        id *= 2;
        id += d as u64;
    }
    id
}

fn parse_len(it: &mut dyn Iterator<Item = u8>) -> (u64, u64) {
    let mut bits = 15;
    if it.next() == Some(1) {
        bits = 11;
    }
    let mut len = 0;
    for d in it.take(bits) {
        len *= 2;
        len += d as u64;
    }
    if bits == 15 {
        (len, 0)
    } else {
        (0, len)
    }
}

fn parse_literal(it: &mut dyn Iterator<Item = u8>) -> u64 {
    let mut num = 0;
    loop {
        let more = it.next() == Some(1);
        for d in it.take(4) {
            num *= 2;
            num += d as u64;
        }
        if !more {
            break;
        }
    }
    num
}

fn parse_packet(it: &mut dyn Iterator<Item = u8>) -> u64 {
    parse_id(it);
    let id = parse_id(it);
    if id == 4 {
        return parse_literal(it);
    }
    let mut values: Vec<u64> = Vec::new();
    let (len, nb_packets) = parse_len(it);
    if len > 0 {
        let mut it = it.take(len as usize);
        while it.size_hint().0 > 0 {
            values.push(parse_packet(&mut it));
        }
    } else {
        for _ in 0..nb_packets {
            values.push(parse_packet(it));
        }
    }

    match id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        4 => return parse_literal(it),
        5 => (values[0] > values[1]) as u64,
        6 => (values[0] < values[1]) as u64,
        7 => (values[0] == values[1]) as u64,
        _ => panic!("oops"),
    }
}

fn bits() -> Vec<u8> {
    let input = input();

    let line = input.lines().next().unwrap();

    let mut bits: Vec<u8> = Vec::new();
    for c in line.split_inclusive(|_| true) {
        let n = u8::from_str_radix(c, 16).unwrap();
        bits.extend(vec![(n & 8) / 8, (n & 4) / 4, (n & 2) / 2, n & 1]);
    }

    bits
}

pub fn part1() {
    let mut it = bits().into_iter();

    let mut tot_version = 0;
    while it.len() > 0 {
        tot_version += parse_id(&mut it);
        if parse_id(&mut it) == 4 {
            parse_literal(&mut it);
        } else {
            parse_len(&mut it);
        }
    }

    println!("{}", tot_version);
}

pub fn part2() {
    let mut it = bits().into_iter();
    println!("{}", parse_packet(&mut it));
}
