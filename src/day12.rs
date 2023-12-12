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

    fn maybe_damaged(self) -> bool {
        matches!(self, Condition::Damaged | Condition::Unknown)
    }

    fn maybe_operational(self) -> bool {
        matches!(self, Condition::Operational | Condition::Unknown)
    }
}

fn parse(line: &str) -> (Vec<Condition>, Vec<usize>) {
    let (springs, runs) = line.split_once(' ').unwrap();
    let springs = springs.chars().map(Condition::parse).collect();
    let runs = runs
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect();
    (springs, runs)
}

fn count_arrangements(springs: &mut [Condition], runs: &[usize]) -> u64 {
    // dbg!(&springs, runs);
    match springs.get(0) {
        None => runs.is_empty().into(),
        Some(Condition::Operational) => count_arrangements(&mut springs[1..], runs),
        Some(Condition::Damaged) => {
            if let Some(run) = runs.get(0) {
                if *run < springs.len() {
                    if springs
                        .iter()
                        .take(*run)
                        .copied()
                        .all(Condition::maybe_damaged)
                        && springs[*run].maybe_operational()
                    {
                        count_arrangements(&mut springs[*run + 1..], &runs[1..])
                    } else {
                        0
                    }
                } else if *run == springs.len() {
                    u64::from(
                        runs.len() == 1 && springs.iter().copied().all(Condition::maybe_damaged),
                    )
                } else {
                    0
                }
            } else {
                0
            }
        }
        Some(Condition::Unknown) => {
            let operational_count = count_arrangements(&mut springs[1..], runs);
            springs[0] = Condition::Damaged;
            let damaged_count = count_arrangements(springs, runs);
            springs[0] = Condition::Unknown;
            damaged_count + operational_count
        }
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse)
        .map(|(mut springs, runs)| count_arrangements(&mut springs, &runs))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
