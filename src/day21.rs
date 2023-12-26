use std::collections::VecDeque;

fn distance_matrix(input: &str) -> Vec<Vec<Option<usize>>> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0, 0);
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (i, j, 0);
                break 'outer;
            }
        }
    }

    let mut queue = VecDeque::from([start]);
    let mut distances = vec![vec![None; grid[0].len()]; grid.len()];
    distances[start.0][start.1] = Some(0);
    while let Some((row, col, dist)) = queue.pop_front() {
        if row > 0 && distances[row - 1][col].is_none() && grid[row - 1][col] != '#' {
            distances[row - 1][col] = Some(dist + 1);
            queue.push_back((row - 1, col, dist + 1));
        }
        if row + 1 < grid.len() && distances[row + 1][col].is_none() && grid[row + 1][col] != '#' {
            distances[row + 1][col] = Some(dist + 1);
            queue.push_back((row + 1, col, dist + 1));
        }
        if col > 0 && distances[row][col - 1].is_none() && grid[row][col - 1] != '#' {
            distances[row][col - 1] = Some(dist + 1);
            queue.push_back((row, col - 1, dist + 1));
        }
        if col + 1 < grid[0].len() && distances[row][col + 1].is_none() && grid[row][col + 1] != '#'
        {
            distances[row][col + 1] = Some(dist + 1);
            queue.push_back((row, col + 1, dist + 1));
        }
    }

    distances
}

pub fn part1(input: &str) -> u64 {
    distance_matrix(input)
        .iter()
        .flatten()
        .flatten()
        .filter(|c| **c <= 64 && *c % 2 == 0)
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let distances = distance_matrix(input);

    macro_rules! count {
        ($pred:expr) => {
            distances.iter().flatten().flatten().filter($pred).count() as u64
        };
    }

    let even_far = count!(|d| **d % 2 == 0 && **d > 65);
    let odd_far = count!(|d| **d % 2 == 1 && **d > 65);
    let even = count!(|d| **d % 2 == 0);
    let odd = count!(|d| **d % 2 == 1);

    40925290000 * even + 40925694601 * odd + 202300 * even_far - 202301 * odd_far
}
