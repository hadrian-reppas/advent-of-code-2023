use std::ops::BitAnd;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mirror {
    Vertical(u64),
    Horizontal(u64),
}

impl Mirror {
    fn summarize(self) -> u64 {
        match self {
            Mirror::Vertical(cols) => cols.trailing_zeros() as u64,
            Mirror::Horizontal(rows) => 100 * rows.trailing_zeros() as u64,
        }
    }

    fn ignore(self, other: Option<Self>) -> Self {
        match (self, other) {
            (Mirror::Vertical(s), Some(Mirror::Vertical(o))) => Mirror::Vertical(s & !o),
            (Mirror::Horizontal(s), Some(Mirror::Horizontal(o))) => Mirror::Horizontal(s & !o),
            _ => self,
        }
    }
}

fn get_symmetries(mut row: u64, len: usize) -> u64 {
    let mut rev = row.reverse_bits() >> (64 - len);
    let mut rev_mask = (1 << len) - 1;
    row <<= len - 2;
    let row_mask = rev_mask << len - 2;

    let (mut out, mut bit) = (0, 2);
    for _ in 0..len - 1 {
        if rev & row_mask == row & rev_mask {
            out |= bit;
        }
        bit <<= 1;
        rev <<= 2;
        rev_mask <<= 2;
    }
    out
}

fn parse_rows(grid: &[u8]) -> (Vec<u64>, usize) {
    let mut rows = Vec::new();
    for line in grid.split(|b| *b == b'\n') {
        let mut row = 0;
        for b in line.iter().rev() {
            row <<= 1;
            row += u64::from(*b == b'#');
        }
        rows.push(row);
    }
    (rows, grid.iter().position(|b| *b == b'\n').unwrap())
}

fn parse_cols(grid: &[u8]) -> (Vec<u64>, usize) {
    let grid: Vec<_> = grid.split(|b| *b == b'\n').collect();
    let mut cols = Vec::new();
    for j in 0..grid[0].len() {
        let mut col = 0;
        for i in (0..grid.len()).rev() {
            col <<= 1;
            col += u64::from(grid[i][j] == b'#');
        }
        cols.push(col);
    }
    (cols, grid.len())
}

fn get_mirror(grid: &[u8], ignore: Option<Mirror>) -> Option<Mirror> {
    let (rows, len) = parse_rows(grid);
    let result = rows
        .into_iter()
        .map(|row| get_symmetries(row, len))
        .fold(u64::MAX, u64::bitand);

    let vertical = Mirror::Vertical(result);
    if result == 0 || Some(vertical) == ignore {
        let (cols, len) = parse_cols(grid);
        let result = cols
            .into_iter()
            .map(|col| get_symmetries(col, len))
            .fold(u64::MAX, u64::bitand);
        let horizontal = Mirror::Horizontal(result);
        if result == 0 || Some(horizontal) == ignore {
            None
        } else {
            Some(horizontal.ignore(ignore))
        }
    } else {
        Some(vertical.ignore(ignore))
    }
}

fn get_smudge_mirror(grid: &mut [u8], ignore: Mirror) -> Mirror {
    for i in 0..grid.len() {
        if grid[i] == b'#' {
            grid[i] = b'.';
            if let Some(mirror) = get_mirror(grid, Some(ignore)) {
                return mirror;
            }
            grid[i] = b'#';
        } else if grid[i] == b'.' {
            grid[i] = b'#';
            if let Some(mirror) = get_mirror(grid, Some(ignore)) {
                return mirror;
            }
            grid[i] = b'.';
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(str::as_bytes)
        .map(|grid| get_mirror(grid, None))
        .map(Option::unwrap)
        .map(Mirror::summarize)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for grid in input.split("\n\n") {
        let mut grid = grid.as_bytes().to_vec();
        let ignore = get_mirror(&grid, None).unwrap();
        sum += get_smudge_mirror(&mut grid, ignore).summarize();
    }
    sum
}
