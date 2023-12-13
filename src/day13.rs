use std::ops::BitAnd;

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

fn parse_rows(grid: &str) -> (Vec<u64>, usize) {
    let mut rows = Vec::new();
    for line in grid.lines() {
        let mut row = 0;
        for c in line.chars().rev() {
            row <<= 1;
            row += u64::from(c == '#');
        }
        rows.push(row);
    }
    (rows, grid.find('\n').unwrap())
}

fn parse_cols(grid: &str) -> (Vec<u64>, usize) {
    let grid: Vec<_> = grid.lines().map(str::as_bytes).collect();
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

fn get_mirror(grid: &str) -> Option<Mirror> {
    todo!()
}

fn get_score(grid: &str) -> u64 {
    let (rows, len) = parse_rows(grid);
    let result = rows
        .into_iter()
        .map(|row| get_symmetries(row, len))
        .fold(u64::MAX, u64::bitand);

    if result == 0 {
        let (cols, len) = parse_cols(grid);
        let result = cols
            .into_iter()
            .map(|col| get_symmetries(col, len))
            .fold(u64::MAX, u64::bitand);
        100 * result.trailing_zeros() as u64
    } else {
        result.trailing_zeros() as u64
    }
}

pub fn part1(input: &str) -> u64 {
    input.split("\n\n").map(get_score).sum()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
