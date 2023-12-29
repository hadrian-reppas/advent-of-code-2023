#[derive(Clone, Copy, Debug)]
struct Dig {
    dir: Direction,
    len: i64,
}

impl Dig {
    fn part1(line: &str) -> Self {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (len, _) = rest.split_once(' ').unwrap();
        Dig {
            dir: Direction::parse(dir),
            len: len.parse().unwrap(),
        }
    }

    fn part2(line: &str) -> Self {
        let (_, rest) = line.split_once('#').unwrap();
        Dig {
            dir: Direction::parse(&rest[5..6]),
            len: i64::from_str_radix(&rest[..5], 16).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "R" | "0" => Direction::Right,
            "D" | "1" => Direction::Down,
            "L" | "2" => Direction::Left,
            "U" | "3" => Direction::Up,
            _ => panic!(),
        }
    }

    fn step(self, (x, y): (i64, i64), len: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x, y + len),
            Direction::Down => (x, y - len),
            Direction::Left => (x - len, y),
            Direction::Right => (x + len, y),
        }
    }
}

fn get_area(digs: impl Iterator<Item = Dig> + Clone) -> u64 {
    let mut pos = (0, 0);
    let mut vertices = vec![pos];
    let mut edge = 0;

    let mut fast = digs.clone();
    let first = fast.next().unwrap();

    for (cur, next) in digs.clone().zip(fast.chain(Some(first))) {
        pos = cur.dir.step(pos, cur.len);
        vertices.push(pos);
        match (cur.dir, next.dir) {
            (Direction::Down, Direction::Left) => edge += 1,
            (Direction::Down, Direction::Right) | (Direction::Right, _) => {}
            (Direction::Up, _) | (Direction::Left, Direction::Up) => edge += cur.len,
            (Direction::Left, Direction::Down) => edge += cur.len - 1,
            _ => panic!(),
        }
    }

    let area = vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .abs()
        / 2;

    (area + edge) as u64
}

pub fn part1(input: &str) -> u64 {
    get_area(input.lines().map(Dig::part1))
}

pub fn part2(input: &str) -> u64 {
    get_area(input.lines().map(Dig::part2))
}
