use nalgebra::base::{Matrix6, Vector6};

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(", ");
        Vector {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn cross(self, other: Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn minus(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn iter_with(self, other: Self) -> impl Iterator<Item = f64> {
        [self.x, self.y, self.z, other.x, other.y, other.z]
            .into_iter()
            .map(|i| i as f64)
    }
}

fn make_matrix(
    p1: Vector,
    p2: Vector,
    p3: Vector,
    v1: Vector,
    v2: Vector,
    v3: Vector,
) -> Matrix6<f64> {
    let matrix = [
        [0, v2.z - v1.z, v1.y - v2.y, 0, p1.z - p2.z, p2.y - p1.y],
        [v1.z - v2.z, 0, v2.x - v1.x, p2.z - p1.z, 0, p1.x - p2.x],
        [v2.y - v1.y, v1.x - v2.x, 0, p1.y - p2.y, p2.x - p1.x, 0],
        [0, v3.z - v1.z, v1.y - v3.y, 0, p1.z - p3.z, p3.y - p1.y],
        [v1.z - v3.z, 0, v3.x - v1.x, p3.z - p1.z, 0, p1.x - p3.x],
        [v3.y - v1.y, v1.x - v3.x, 0, p1.y - p3.y, p3.x - p1.x, 0],
    ];
    Matrix6::from_row_iterator(matrix.into_iter().flatten().map(|i| i as f64))
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Vector,
    vel: Vector,
}

impl Hailstone {
    fn parse(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" @ ").unwrap();
        Hailstone {
            pos: Vector::parse(lhs),
            vel: Vector::parse(rhs),
        }
    }

    fn intersection_2d(self, other: Self) -> Option<(f64, f64)> {
        let dx = other.pos.x - self.pos.x;
        let dy = other.pos.y - self.pos.y;
        let det = other.vel.x * self.vel.y - other.vel.y * self.vel.x;
        if det == 0 {
            None
        } else {
            let u = (dy * other.vel.x - dx * other.vel.y) as f64 / det as f64;
            let v = (dy * self.vel.x - dx * self.vel.y) as f64 / det as f64;
            if u >= 0.0 && v >= 0.0 {
                Some((
                    self.pos.x as f64 + u * self.vel.x as f64,
                    self.pos.y as f64 + u * self.vel.y as f64,
                ))
            } else {
                None
            }
        }
    }
}

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

pub fn part1(input: &str) -> u64 {
    let stones: Vec<_> = input.lines().map(Hailstone::parse).collect();
    let mut count = 0;
    for (i, a) in stones.iter().enumerate() {
        for b in &stones[..i] {
            if let Some((x, y)) = a.intersection_2d(*b) {
                if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u64 {
    let mut stones = input.lines().map(Hailstone::parse);
    let stone1 = stones.next().unwrap();
    let stone2 = stones.next().unwrap();
    let stone3 = stones.next().unwrap();
    let matrix = make_matrix(
        stone1.pos, stone2.pos, stone3.pos, stone1.vel, stone2.vel, stone3.vel,
    );
    let a = stone1.pos.cross(stone1.vel);
    let b = stone1.pos.cross(stone1.vel);
    let c = stone2.pos.cross(stone2.vel).minus(a);
    let d = stone3.pos.cross(stone3.vel).minus(b);
    let v = Vector6::from_iterator(c.iter_with(d));
    let inverse = matrix.try_inverse().unwrap();
    let result = inverse * v;
    (result.index(0) + result.index(1) + result.index(2)).round() as u64
}
