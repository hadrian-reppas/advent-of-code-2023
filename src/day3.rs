use regex::Regex;
use std::cmp::min;
use std::collections::{hash_map::Entry, HashMap};

pub fn part1(input: &str) -> u64 {
    fn contains_symbol(s: &str) -> bool {
        s.chars().any(|c| c != '.' && !c.is_ascii_digit())
    }

    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    let lines: Vec<_> = input.lines().collect();

    for (i, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            let start = m.start().saturating_sub(1);
            let end = min(line.len(), m.end() + 1);
            let symbol_above = i > 0 && contains_symbol(&lines[i - 1][start..end]);
            let symbol_below = i < lines.len() - 1 && contains_symbol(&lines[i + 1][start..end]);
            let symbol_here = contains_symbol(&line[start..end]);
            if symbol_above || symbol_below || symbol_here {
                sum += m.as_str().parse::<u64>().unwrap();
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut gears = HashMap::<_, Vec<_>>::new();

    for (i, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            let start = m.start().saturating_sub(1);
            let end = min(line.len(), m.end() + 1);

            macro_rules! gears {
                ($i:expr) => {
                    lines[$i][start..end]
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '*')
                        .map(|(j, _)| ($i - 1, start + j))
                };
            }

            let gears_above = (i > 0).then(|| gears!(i - 1)).into_iter().flatten();
            let gears_here = gears!(i);
            let gears_below = (i < lines.len() - 1)
                .then(|| gears!(i + 1))
                .into_iter()
                .flatten();

            let num: u64 = m.as_str().parse().unwrap();

            for gear in gears_above.chain(gears_here).chain(gears_below) {
                match gears.entry(gear) {
                    Entry::Occupied(mut o) => o.get_mut().push(num),
                    Entry::Vacant(v) => {
                        v.insert(vec![num]);
                    }
                }
            }
        }
    }

    gears
        .into_values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum()
}
