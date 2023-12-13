use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn parse(c: char) -> Self {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!(),
        }
    }

    fn maybe_damaged(&self) -> bool {
        matches!(self, Condition::Damaged | Condition::Unknown)
    }

    fn maybe_operational(&self) -> bool {
        matches!(self, Condition::Operational | Condition::Unknown)
    }
}

fn parse(line: &str, copies: usize) -> (Vec<Condition>, Vec<usize>) {
    let (springs, runs) = line.split_once(' ').unwrap();
    let springs = vec![springs; copies].join("?");
    let runs = vec![runs; copies].join(",");
    (
        springs.chars().map(Condition::parse).collect(),
        runs.split(',')
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect(),
    )
}

fn count_arrangements(springs: &[Condition], runs: &[usize]) -> u64 {
    if runs.is_empty() {
        return springs.iter().all(Condition::maybe_operational).into();
    }

    let (left_runs, right_runs) = runs.split_at(runs.len() / 2);
    let (&run, right_runs) = right_runs.split_first().unwrap();

    if run == springs.len() {
        return springs.iter().all(Condition::maybe_damaged).into();
    }

    let mut left_margin = left_runs.iter().sum::<usize>() + left_runs.len();
    let mut right_margin = right_runs.iter().sum::<usize>() + right_runs.len();

    let mut count = 0;

    if left_margin == 0 {
        left_margin = 1;
        if springs[..run].iter().all(Condition::maybe_damaged) && springs[run].maybe_operational() {
            count += count_arrangements(&springs[run + 1..], right_runs);
        }
    }
    if right_margin == 0 {
        right_margin = 1;
        if springs[springs.len() - run..]
            .iter()
            .all(Condition::maybe_damaged)
            && springs[springs.len() - run - 1].maybe_operational()
        {
            count += count_arrangements(&springs[..springs.len() - run - 1], left_runs);
        }
    }

    for start in left_margin..springs.len() + 1 - right_margin - run {
        if springs[start - 1].maybe_operational()
            && springs[start + run].maybe_operational()
            && springs[start..start + run]
                .iter()
                .all(Condition::maybe_damaged)
        {
            count += count_arrangements(&springs[..start - 1], left_runs)
                * count_arrangements(&springs[start + run + 1..], right_runs);
        }
    }

    count
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| parse(line, 1))
        .map(|(springs, runs)| count_arrangements(&springs, &runs))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| parse(line, 5))
        .map(|(springs, runs)| count_arrangements(&springs, &runs))
        .sum()
}
