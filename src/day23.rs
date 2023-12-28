use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Neighbor {
    row: usize,
    col: usize,
    distance: u64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self, row: usize, col: usize) -> [(usize, usize, Direction); 3] {
        match self {
            Direction::Up => [
                (row - 1, col, Direction::Up),
                (row, col - 1, Direction::Left),
                (row, col + 1, Direction::Right),
            ],
            Direction::Down => [
                (row + 1, col, Direction::Down),
                (row, col - 1, Direction::Left),
                (row, col + 1, Direction::Right),
            ],
            Direction::Left => [
                (row, col - 1, Direction::Left),
                (row - 1, col, Direction::Up),
                (row + 1, col, Direction::Down),
            ],
            Direction::Right => [
                (row, col + 1, Direction::Right),
                (row - 1, col, Direction::Up),
                (row + 1, col, Direction::Down),
            ],
        }
    }
}

fn trace_segment(
    mut row: usize,
    mut col: usize,
    mut dir: Direction,
    grid: &[Vec<char>],
) -> (usize, usize, u64) {
    let mut distance = 1;
    (row, col, _) = dir.next(row, col)[0];
    loop {
        for (next_row, next_col, next_dir) in dir.next(row, col) {
            if next_row == grid.len() {
                return (row, col, distance);
            }
            match grid[next_row][next_col] {
                '.' => {
                    distance += 1;
                    (row, col, dir) = (next_row, next_col, next_dir);
                }
                '>' | '<' | 'v' | '^' => {
                    (row, col, _) = next_dir.next(next_row, next_col)[0];
                    return (row, col, distance + 2);
                }
                _ => {}
            }
        }
    }
}

fn outgoing(row: usize, col: usize, grid: &[Vec<char>]) -> impl Iterator<Item = Direction> {
    let up = (grid[row - 1][col] == '^').then_some(Direction::Up);
    let down = (grid[row + 1][col] == 'v').then_some(Direction::Down);
    let left = (grid[row][col - 1] == '<').then_some(Direction::Left);
    let right = (grid[row][col + 1] == '>').then_some(Direction::Right);
    up.into_iter().chain(down).chain(left).chain(right)
}

fn parse(input: &str) -> (HashMap<(usize, usize), Vec<Neighbor>>, usize, usize) {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut graph = HashMap::from([((0, 1), Vec::new())]);
    let mut stack = vec![(0, 1, Direction::Down)];
    let mut seen = HashSet::new();

    while let Some((row, col, dir)) = stack.pop() {
        if seen.contains(&(row, col, dir)) {
            continue;
        }
        seen.insert((row, col, dir));
        let (next_row, next_col, distance) = trace_segment(row, col, dir, &grid);
        graph.entry((next_row, next_col)).or_insert(Vec::new());
        graph.get_mut(&(row, col)).unwrap().push(Neighbor {
            row: next_row,
            col: next_col,
            distance,
        });
        if next_row < grid.len() - 1 {
            for dir in outgoing(next_row, next_col, &grid) {
                stack.push((next_row, next_col, dir));
            }
        }
    }

    (graph, grid.len(), grid[0].len())
}

fn make_undirected(graph: &mut HashMap<(usize, usize), Vec<Neighbor>>) {
    let mut inverse: HashMap<_, _> = graph.keys().map(|n| (*n, Vec::new())).collect();
    for (node, neighbors) in graph.iter() {
        for Neighbor { row, col, distance } in neighbors {
            inverse.get_mut(&(*row, *col)).unwrap().push(Neighbor {
                row: node.0,
                col: node.1,
                distance: *distance,
            });
        }
    }
    for (node, mut neighbors) in inverse {
        graph.get_mut(&node).unwrap().append(&mut neighbors);
    }
}

fn longest_path(
    row: usize,
    col: usize,
    graph: &HashMap<(usize, usize), Vec<Neighbor>>,
    seen: &mut HashSet<(usize, usize)>,
    end_row: usize,
    end_col: usize,
) -> u64 {
    if row == end_row && col == end_col {
        1_000_000_000
    } else {
        graph[&(row, col)]
            .iter()
            .filter_map(|n| {
                if seen.contains(&(n.row, n.col)) {
                    None
                } else {
                    seen.insert((n.row, n.col));
                    let distance =
                        n.distance + longest_path(n.row, n.col, graph, seen, end_row, end_col);
                    seen.remove(&(n.row, n.col));
                    Some(distance)
                }
            })
            .max()
            .unwrap_or(0)
    }
}

pub fn part1(input: &str) -> u64 {
    let (graph, rows, cols) = parse(input);
    longest_path(0, 1, &graph, &mut HashSet::new(), rows - 1, cols - 2) - 1_000_000_000
}

pub fn part2(input: &str) -> u64 {
    let (mut graph, rows, cols) = parse(input);
    make_undirected(&mut graph);
    longest_path(0, 1, &graph, &mut HashSet::new(), rows - 1, cols - 2) - 1_000_000_000
}
