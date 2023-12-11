use std::str::FromStr;

fn parse(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|l| {
        l.split(' ')
            .map(i64::from_str)
            .map(Result::unwrap)
            .collect()
    })
}

fn deltas(mut nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack = Vec::new();
    while nums.iter().any(|n| *n != 0) {
        let deltas = nums.iter().skip(1).zip(&nums).map(|(a, b)| a - b).collect();
        stack.push(nums);
        nums = deltas;
    }
    stack
}

pub fn part1(input: &str) -> u64 {
    parse(input)
        .map(|nums| deltas(nums).iter().map(|d| d.last().unwrap()).sum::<i64>())
        .sum::<i64>() as u64
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|nums| {
            deltas(nums)
                .iter()
                .rev()
                .map(|d| d[0])
                .fold(0, |acc, i| i - acc)
        })
        .sum::<i64>() as u64
}
