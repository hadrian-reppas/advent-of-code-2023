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

impl Position {
    fn new(row: usize, col: usize, dir: Direction, run: u8) -> Self {
        Position { row, col, dir, run }
    }

    fn up(self) -> Option<Self> {
        if self.row == 0
            || self.dir == Direction::Down
            || self.dir == Direction::Up && self.run == 3
        {
            None
        } else if self.dir == Direction::Up {
            Some(Position::new(
                self.row - 1,
                self.col,
                Direction::Up,
                self.run + 1,
            ))
        } else {
            Some(Position::new(self.row - 1, self.col, Direction::Up, 1))
        }
    }

    fn down(self, rows: usize) -> Option<Self> {
        if self.row + 1 == rows
            || self.dir == Direction::Up
            || self.dir == Direction::Down && self.run == 3
        {
            None
        } else if self.dir == Direction::Down {
            Some(Position::new(
                self.row + 1,
                self.col,
                Direction::Down,
                self.run + 1,
            ))
        } else {
            Some(Position::new(self.row + 1, self.col, Direction::Down, 1))
        }
    }

    fn left(self) -> Option<Self> {
        if self.col == 0
            || self.dir == Direction::Right
            || self.dir == Direction::Left && self.run == 3
        {
            None
        } else if self.dir == Direction::Left {
            Some(Position::new(
                self.row,
                self.col - 1,
                Direction::Left,
                self.run + 1,
            ))
        } else {
            Some(Position::new(self.row, self.col - 1, Direction::Left, 1))
        }
    }

    fn right(self, cols: usize) -> Option<Self> {
        if self.col + 1 == cols
            || self.dir == Direction::Left
            || self.dir == Direction::Right && self.run == 3
        {
            None
        } else if self.dir == Direction::Right {
            Some(Position::new(
                self.row,
                self.col + 1,
                Direction::Right,
                self.run + 1,
            ))
        } else {
            Some(Position::new(self.row, self.col + 1, Direction::Right, 1))
        }
    }

    fn neighbors(self, rows: usize, cols: usize) -> impl Iterator<Item = Position> {
        self.up()
            .into_iter()
            .chain(self.down(rows))
            .chain(self.left())
            .chain(self.right(cols))
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

pub fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u64 - 48).collect())
        .collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut done = HashSet::new();
    let mut distances = HashMap::new();
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
        distances.insert(position, heat_loss);

        for neighbor in position.neighbors(rows, cols) {
            if done.contains(&neighbor) {
                continue;
            }
            let new_heat_loss = heat_loss + grid[neighbor.row][neighbor.col];
            if new_heat_loss < distances.get(&neighbor).copied().unwrap_or(u64::MAX) {
                distances.insert(neighbor, new_heat_loss);
                queue.push(Pair {
                    heat_loss: new_heat_loss,
                    position: neighbor,
                });
            }
        }
    }

    distances
        .iter()
        .filter_map(|(pos, heat)| (pos.row == rows - 1 && pos.col == cols - 1).then_some(*heat))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
