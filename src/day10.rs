use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn step(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::East => (row, col + 1),
            Direction::West => (row, col - 1),
        }
    }

    fn lhs(self, row: usize, col: usize, rows: usize, cols: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => Some((row, col.checked_sub(1)?)),
            Direction::South => (col + 1 < cols).then_some((row, col + 1)),
            Direction::East => Some((row.checked_sub(1)?, col)),
            Direction::West => (row + 1 < rows).then_some((row + 1, col)),
        }
    }

    fn rhs(self, row: usize, col: usize, rows: usize, cols: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => (col + 1 < cols).then_some((row, col + 1)),
            Direction::South => Some((row, col.checked_sub(1)?)),
            Direction::East => (row + 1 < rows).then_some((row + 1, col)),
            Direction::West => Some((row.checked_sub(1)?, col)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn parse(c: char) -> Option<Self> {
        match c {
            '|' => Some(Pipe::NorthSouth),
            '-' => Some(Pipe::EastWest),
            'L' => Some(Pipe::NorthEast),
            'J' => Some(Pipe::NorthWest),
            '7' => Some(Pipe::SouthWest),
            'F' => Some(Pipe::SouthEast),
            _ => None,
        }
    }

    fn other_dir(self, dir: Direction) -> Direction {
        match (self, dir) {
            (Pipe::NorthSouth, Direction::North) => Direction::South,
            (Pipe::NorthSouth, Direction::South) => Direction::North,
            (Pipe::EastWest, Direction::East) => Direction::West,
            (Pipe::EastWest, Direction::West) => Direction::East,
            (Pipe::NorthEast, Direction::North) => Direction::East,
            (Pipe::NorthEast, Direction::East) => Direction::North,
            (Pipe::NorthWest, Direction::North) => Direction::West,
            (Pipe::NorthWest, Direction::West) => Direction::North,
            (Pipe::SouthWest, Direction::South) => Direction::West,
            (Pipe::SouthWest, Direction::West) => Direction::South,
            (Pipe::SouthEast, Direction::South) => Direction::East,
            (Pipe::SouthEast, Direction::East) => Direction::South,
            _ => panic!(),
        }
    }

    fn contains(self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Pipe::NorthSouth, Direction::North)
                | (Pipe::NorthSouth, Direction::South)
                | (Pipe::EastWest, Direction::East)
                | (Pipe::EastWest, Direction::West)
                | (Pipe::NorthEast, Direction::North)
                | (Pipe::NorthEast, Direction::East)
                | (Pipe::NorthWest, Direction::North)
                | (Pipe::NorthWest, Direction::West)
                | (Pipe::SouthWest, Direction::South)
                | (Pipe::SouthWest, Direction::West)
                | (Pipe::SouthEast, Direction::South)
                | (Pipe::SouthEast, Direction::East)
        )
    }
}

fn get_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if *x == 'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn get_starting_dir(
    start_row: usize,
    start_col: usize,
    grid: &[Vec<char>],
) -> (Direction, Direction) {
    let mut dir1 = None;
    for dir in Direction::ALL {
        let (row, col) = dir.step(start_row, start_col);
        if let Some(pipe) = Pipe::parse(grid[row][col]) {
            if pipe.contains(dir.opposite()) {
                if let Some(dir1) = dir1 {
                    return (dir1, dir);
                } else {
                    dir1 = Some(dir);
                }
            }
        }
    }
    unreachable!()
}

fn find_path(grid: &[Vec<char>]) -> Vec<(usize, usize, Direction, Direction)> {
    let (mut row, mut col) = get_start(grid);
    let (start_dir, last_dir) = get_starting_dir(row, col, grid);
    let (mut path, mut dir) = (vec![(row, col, last_dir, start_dir)], start_dir);

    loop {
        (row, col) = dir.step(row, col);
        let Some(pipe) = Pipe::parse(grid[row][col]) else {
            break;
        };
        let old_dir = dir;
        dir = pipe.other_dir(dir.opposite());
        path.push((row, col, old_dir, dir));
    }

    path
}

fn flood_fill(
    mut stack: Vec<(usize, usize)>,
    path: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> HashSet<(usize, usize)> {
    let mut seen = HashSet::new();

    while let Some((row, col)) = stack.pop() {
        if seen.contains(&(row, col)) {
            continue;
        }
        seen.insert((row, col));
        if row > 0 && !path.contains(&(row - 1, col)) {
            stack.push((row - 1, col));
        }
        if row + 1 < rows && !path.contains(&(row + 1, col)) {
            stack.push((row + 1, col));
        }
        if col > 0 && !path.contains(&(row, col - 1)) {
            stack.push((row, col - 1));
        }
        if col + 1 < cols && !path.contains(&(row, col + 1)) {
            stack.push((row, col + 1));
        }
    }

    seen
}

pub fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let path = find_path(&grid);
    (path.len() / 2) as u64
}

pub fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let path = find_path(&grid);
    let path_set: HashSet<_> = path.iter().map(|(r, c, _, _)| (*r, *c)).collect();
    let (mut lhs, mut rhs) = (HashSet::new(), HashSet::new());

    for (row, col, dir1, dir2) in &path {
        lhs.extend(dir1.lhs(*row, *col, rows, cols));
        lhs.extend(dir2.lhs(*row, *col, rows, cols));
        rhs.extend(dir1.rhs(*row, *col, rows, cols));
        rhs.extend(dir2.rhs(*row, *col, rows, cols));
    }

    let lhs: Vec<_> = lhs.difference(&path_set).copied().collect();
    let rhs: Vec<_> = rhs.difference(&path_set).copied().collect();

    let lhs = flood_fill(lhs, &path_set, rows, cols);
    let rhs = flood_fill(rhs, &path_set, rows, cols);

    if lhs.contains(&(0, 0)) {
        rhs.len() as u64
    } else {
        lhs.len() as u64
    }
}
