use bimap::BiMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Cube,
    Rounded,
    None,
}

impl Rock {
    fn parse(c: char) -> Self {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Rounded,
            '.' => Rock::None,
            _ => panic!(),
        }
    }
}

fn move_north(grid: &mut [Vec<Rock>], row: usize, col: usize) {
    let mut to_row = row;
    while to_row > 0 && grid[to_row - 1][col] == Rock::None {
        to_row -= 1;
    }
    grid[row][col] = Rock::None;
    grid[to_row][col] = Rock::Rounded;
}

fn move_south(grid: &mut [Vec<Rock>], row: usize, col: usize) {
    let mut to_row = row;
    while to_row + 1 < grid.len() && grid[to_row + 1][col] == Rock::None {
        to_row += 1;
    }
    grid[row][col] = Rock::None;
    grid[to_row][col] = Rock::Rounded;
}

fn move_west(grid: &mut [Vec<Rock>], row: usize, col: usize) {
    let mut to_col = col;
    while to_col > 0 && grid[row][to_col - 1] == Rock::None {
        to_col -= 1;
    }
    grid[row][col] = Rock::None;
    grid[row][to_col] = Rock::Rounded;
}

fn move_east(grid: &mut [Vec<Rock>], row: usize, col: usize) {
    let mut to_col = col;
    while to_col + 1 < grid[0].len() && grid[row][to_col + 1] == Rock::None {
        to_col += 1;
    }
    grid[row][col] = Rock::None;
    grid[row][to_col] = Rock::Rounded;
}

fn tilt(grid: &mut [Vec<Rock>], mover: fn(&mut [Vec<Rock>], usize, usize), rev: bool) {
    if rev {
        for row in (0..grid.len()).rev() {
            for col in (0..grid[0].len()).rev() {
                if grid[row][col] == Rock::Rounded {
                    mover(grid, row, col);
                }
            }
        }
    } else {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == Rock::Rounded {
                    mover(grid, row, col);
                }
            }
        }
    }
}

fn do_cycle(grid: &mut [Vec<Rock>]) {
    tilt(grid, move_north, false);
    tilt(grid, move_west, false);
    tilt(grid, move_south, true);
    tilt(grid, move_east, true);
}

fn compute_load(grid: &[Vec<Rock>]) -> u64 {
    grid.iter()
        .rev()
        .zip(1..)
        .map(|(row, load)| row.iter().filter(|x| **x == Rock::Rounded).count() as u64 * load)
        .sum()
}

pub fn part1(input: &str) -> u64 {
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Rock::parse).collect())
        .collect();

    tilt(&mut grid, move_north, false);
    compute_load(&grid)
}

pub fn part2(input: &str) -> u64 {
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Rock::parse).collect())
        .collect();

    let mut map = BiMap::new();
    map.insert(grid.clone(), 0);
    let (cycle_start, cycle_end) = loop {
        do_cycle(&mut grid);
        if let Some(start) = map.get_by_left(&grid) {
            break (*start, map.len());
        }
        map.insert(grid.clone(), map.len());
    };

    let cycle_len = cycle_end - cycle_start;
    let cycle_pos = (1_000_000_000 - cycle_end) % cycle_len;
    let index = cycle_start + cycle_pos;
    let grid = map.get_by_right(&index).unwrap();
    compute_load(grid)
}
