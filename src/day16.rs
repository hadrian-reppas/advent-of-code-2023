use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    UpMirror,
    DownMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::UpMirror,
            '\\' => Tile::DownMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => panic!(),
        }
    }

    fn handle(
        self,
        Ray { row, col, dir }: Ray,
        rows: usize,
        cols: usize,
    ) -> impl Iterator<Item = Ray> {
        use Direction::*;
        use Tile::*;

        macro_rules! ray {
            ($row:expr, $col:expr, $dir:expr) => {
                [Some(Ray::new($row, $col, $dir)), None]
            };
        }

        let rays = match (self, dir) {
            (Empty, Up) if row > 0 => ray!(row - 1, col, Up),
            (Empty, Down) if row + 1 < rows => ray!(row + 1, col, Down),
            (Empty, Left) if col > 0 => ray!(row, col - 1, Left),
            (Empty, Right) if col + 1 < cols => ray!(row, col + 1, Right),
            (UpMirror, Up) if col + 1 < cols => ray!(row, col + 1, Right),
            (UpMirror, Down) if col > 0 => ray!(row, col - 1, Left),
            (UpMirror, Left) if row + 1 < rows => ray!(row + 1, col, Down),
            (UpMirror, Right) if row > 0 => ray!(row - 1, col, Up),
            (DownMirror, Up) if col > 0 => ray!(row, col - 1, Left),
            (DownMirror, Down) if col + 1 < cols => ray!(row, col + 1, Right),
            (DownMirror, Left) if row > 0 => ray!(row - 1, col, Up),
            (DownMirror, Right) if row + 1 < rows => ray!(row + 1, col, Down),
            (VerticalSplitter, Up) if row > 0 => ray!(row - 1, col, Up),
            (VerticalSplitter, Down) if row + 1 < rows => ray!(row + 1, col, Down),
            (VerticalSplitter, Left | Right) => [
                (row > 0).then(|| Ray::new(row - 1, col, Up)),
                (row + 1 < rows).then_some(Ray::new(row + 1, col, Down)),
            ],
            (HorizontalSplitter, Up | Down) => [
                (col > 0).then(|| Ray::new(row, col - 1, Left)),
                (col + 1 < cols).then_some(Ray::new(row, col + 1, Right)),
            ],
            (HorizontalSplitter, Left) if col > 0 => ray!(row, col - 1, Left),
            (HorizontalSplitter, Right) if col + 1 < cols => ray!(row, col + 1, Right),
            _ => [None, None],
        };
        rays.into_iter().flatten()
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Ray {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Ray {
    fn new(row: usize, col: usize, dir: Direction) -> Self {
        Ray { row, col, dir }
    }
}

fn count_energized_tiles(grid: &[Vec<Tile>], ray: Ray) -> u64 {
    let mut stack = vec![ray];
    let mut seen = HashSet::new();
    while let Some(ray) = stack.pop() {
        if seen.contains(&ray) {
            continue;
        }
        seen.insert(ray);
        stack.extend(grid[ray.row][ray.col].handle(ray, grid.len(), grid[0].len()));
    }

    let coords: HashSet<_> = seen
        .iter()
        .map(|Ray { row, col, .. }| (*row, *col))
        .collect();
    coords.len() as u64
}

pub fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    count_energized_tiles(&grid, Ray::new(0, 0, Direction::Right))
}

pub fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let horizontal_rays = (0..rows).flat_map(|row| {
        [
            Ray::new(row, 0, Direction::Right),
            Ray::new(row, cols - 1, Direction::Left),
        ]
    });
    let vertical_rays = (0..cols).flat_map(|col| {
        [
            Ray::new(0, col, Direction::Down),
            Ray::new(rows - 1, col, Direction::Up),
        ]
    });

    horizontal_rays
        .chain(vertical_rays)
        .map(|ray| count_energized_tiles(&grid, ray))
        .max()
        .unwrap()
}
