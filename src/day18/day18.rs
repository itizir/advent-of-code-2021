#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day18() {
        println!("part1:");
        part1();
        println!("part2:");
        part2();
    }
}

fn input() -> String {
    String::from(include_str!("input.txt"))
}

#[derive(Clone)]
enum Node {
    Pair { left: Box<Node>, right: Box<Node> },
    Number { value: u8 },
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Pair { left, right } => write!(f, "[{},{}]", left, right),
            Node::Number { value } => write!(f, "{}", value),
        }
    }
}

impl Node {
    fn get_value(&mut self) -> Option<&mut u8> {
        match self {
            Node::Number { ref mut value } => Some(value),
            _ => None,
        }
    }

    fn explode(&mut self, depth: u8) -> (u8, u8) {
        let depth_limit = 4;
        let carry_left;
        let carry_right;
        match self {
            Node::Pair {
                ref mut left,
                ref mut right,
            } => {
                if depth > depth_limit {
                    carry_left = *left.get_value().unwrap();
                    carry_right = *right.get_value().unwrap();
                } else {
                    let (cl, cr) = left.explode(depth + 1);
                    carry_left = cl;
                    right.add_left(cr);
                    let (cl, cr) = right.explode(depth + 1);
                    carry_right = cr;
                    left.add_right(cl);
                }
            }
            _ => return (0, 0),
        }
        if depth > depth_limit {
            *self = Node::Number { value: 0 };
        }
        (carry_left, carry_right)
    }

    fn add_left(&mut self, val: u8) {
        if val == 0 {
            return;
        }
        match self {
            Node::Pair { ref mut left, .. } => left.add_left(val),
            Node::Number { ref mut value } => *value += val,
        }
    }
    fn add_right(&mut self, val: u8) {
        if val == 0 {
            return;
        }
        match self {
            Node::Pair { ref mut right, .. } => right.add_right(val),
            Node::Number { ref mut value } => *value += val,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Node::Pair {
                ref mut left,
                ref mut right,
            } => left.split() || right.split(),
            Node::Number { value } => {
                if *value < 10 {
                    return false;
                }
                *self = new_pair(*value / 2, (*value + 1) / 2);
                true
            }
        }
    }

    fn reduce(&mut self) {
        self.explode(1);
        while self.split() {
            self.explode(1);
        }
    }

    fn add(&self, other: &Node) -> Node {
        let mut n = Node::Pair {
            left: Box::new(self.clone()),
            right: Box::new(other.clone()),
        };
        n.reduce();
        n
    }

    fn magnitude(self) -> u64 {
        match self {
            Node::Number { value } => value as u64,
            Node::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

fn new_pair(l: u8, r: u8) -> Node {
    Node::Pair {
        left: Box::new(Node::Number { value: l }),
        right: Box::new(Node::Number { value: r }),
    }
}

fn parse(it: &mut dyn Iterator<Item = char>) -> Node {
    let c = it.next().unwrap();
    if '0' <= c && c <= '9' {
        return Node::Number {
            value: c as u8 - b'0',
        };
    }
    if c != '[' {
        panic!("no starting bracket??");
    }
    let left = parse(it);
    if it.next().unwrap() != ',' {
        panic!("no separation comma??");
    }
    let right = parse(it);
    if it.next().unwrap() != ']' {
        panic!("no closing bracket??");
    }
    Node::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn part1() {
    let input = input();

    let mut lines = input.lines();
    let mut snailfish = parse(&mut lines.next().unwrap().chars());

    for line in lines {
        snailfish = snailfish.add(&parse(&mut line.chars()));
    }

    println!("{}", snailfish.magnitude());
}

pub fn part2() {
    let input = input();

    let mut numbers: Vec<Node> = Vec::new();
    for line in input.lines() {
        numbers.push(parse(&mut line.chars()));
    }

    let mut max = 0;
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }
            let m = a.add(b).magnitude();
            if m > max {
                max = m;
            }
        }
    }

    println!("{}", max);
}
