use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
    dir: Direction,
    run: u8,
}

macro_rules! neighbor {
    ($self:expr, $border:expr, $dir:ident, $opposite:ident, $new_row:expr, $new_col:expr, $min_run:expr, $max_run:expr) => {
        if $border || $self.dir == Direction::$opposite {
            None
        } else if $self.dir == Direction::$dir {
            ($self.run < $max_run).then_some(Position::new(
                $new_row,
                $new_col,
                Direction::$dir,
                $self.run + 1,
            ))
        } else {
            ($self.run >= $min_run).then_some(Position::new($new_row, $new_col, Direction::$dir, 1))
        }
    };
}

impl Position {
    fn new(row: usize, col: usize, dir: Direction, run: u8) -> Self {
        Position { row, col, dir, run }
    }

    fn up(self, min_run: u8, max_run: u8) -> Option<Self> {
        neighbor!(
            self,
            self.row == 0,
            Up,
            Down,
            self.row - 1,
            self.col,
            min_run,
            max_run
        )
    }

    fn down(self, rows: usize, min_run: u8, max_run: u8) -> Option<Self> {
        neighbor!(
            self,
            self.row == rows - 1,
            Down,
            Up,
            self.row + 1,
            self.col,
            min_run,
            max_run
        )
    }

    fn left(self, min_run: u8, max_run: u8) -> Option<Self> {
        neighbor!(
            self,
            self.col == 0,
            Left,
            Right,
            self.row,
            self.col - 1,
            min_run,
            max_run
        )
    }

    fn right(self, cols: usize, min_run: u8, max_run: u8) -> Option<Self> {
        neighbor!(
            self,
            self.col == cols - 1,
            Right,
            Left,
            self.row,
            self.col + 1,
            min_run,
            max_run
        )
    }

    fn neighbors(
        self,
        rows: usize,
        cols: usize,
        min_run: u8,
        max_run: u8,
    ) -> impl Iterator<Item = Position> {
        self.up(min_run, max_run)
            .into_iter()
            .chain(self.down(rows, min_run, max_run))
            .chain(self.left(min_run, max_run))
            .chain(self.right(cols, min_run, max_run))
    }

    fn at_end(self, rows: usize, cols: usize, min_run: u8) -> bool {
        self.row == rows - 1 && self.col == cols - 1 && self.run >= min_run
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    heat_loss: u64,
    position: Position,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

fn find_min_heat_loss(input: &str, min_run: u8, max_run: u8) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u64 - 48).collect())
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut done = HashSet::new();
    let mut min_heat_loss = HashMap::new();
    let mut queue = BinaryHeap::from([
        Pair {
            heat_loss: 0,
            position: Position::new(0, 0, Direction::Down, 0),
        },
        Pair {
            heat_loss: 0,
            position: Position::new(0, 0, Direction::Right, 0),
        },
    ]);
    while let Some(Pair {
        heat_loss,
        position,
    }) = queue.pop()
    {
        if done.contains(&position) {
            continue;
        }
        done.insert(position);
        min_heat_loss.insert(position, heat_loss);

        for neighbor in position.neighbors(rows, cols, min_run, max_run) {
            if done.contains(&neighbor) {
                continue;
            }
            let new_heat_loss = heat_loss + grid[neighbor.row][neighbor.col];
            if new_heat_loss < min_heat_loss.get(&neighbor).copied().unwrap_or(u64::MAX) {
                min_heat_loss.insert(neighbor, new_heat_loss);
                queue.push(Pair {
                    heat_loss: new_heat_loss,
                    position: neighbor,
                });
            }
        }
    }

    min_heat_loss
        .iter()
        .filter_map(|(pos, heat)| pos.at_end(rows, cols, min_run).then_some(*heat))
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> u64 {
    find_min_heat_loss(input, 0, 3)
}

pub fn part2(input: &str) -> u64 {
    find_min_heat_loss(input, 4, 10)
}
