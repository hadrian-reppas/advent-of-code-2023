use std::cmp::{max, min};
use std::collections::BTreeSet;

fn adjusted_distance(
    (row1, col1): (usize, usize),
    (row2, col2): (usize, usize),
    empty_rows: &BTreeSet<usize>,
    empty_cols: &BTreeSet<usize>,
    expansion: usize,
) -> u64 {
    let distance = row1.abs_diff(row2) + col1.abs_diff(col2);
    let delta_rows = empty_rows.range(row2..row1).count();
    let delta_cols = empty_cols.range(min(col1, col2)..max(col1, col2)).count();

    (expansion * delta_rows + expansion * delta_cols + distance) as u64
}

fn sum_distances(input: &str, expansion_factor: usize) -> u64 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let empty_rows: BTreeSet<_> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|x| !x).then_some(i))
        .collect();
    let empty_cols: BTreeSet<_> = (0..grid[0].len())
        .filter(|col| grid.iter().all(|row| !row[*col]))
        .collect();
    let galaxies: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, x)| x.then_some((i, j)))
        })
        .collect();

    let mut sum = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[..i].iter() {
            sum += adjusted_distance(
                *galaxy1,
                *galaxy2,
                &empty_rows,
                &empty_cols,
                expansion_factor,
            );
        }
    }
    sum
}

pub fn part1(input: &str) -> u64 {
    sum_distances(input, 1)
}

pub fn part2(input: &str) -> u64 {
    sum_distances(input, 999_999)
}
