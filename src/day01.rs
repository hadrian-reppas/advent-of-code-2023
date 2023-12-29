use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    fn get_calibration_value(line: &str) -> u64 {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap();
        let zero = '0' as u64;
        10 * (first as u64 - zero) + (last as u64 - zero)
    }

    input.lines().map(get_calibration_value).sum()
}

pub fn part2(input: &str) -> u64 {
    fn last_ascii_match<'a>(s: &'a str, re: &Regex) -> &'a str {
        for start in (0..s.len()).rev() {
            if let Some(m) = re.find(&s[start..]) {
                return m.as_str();
            }
        }

        unreachable!()
    }

    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .lines()
        .map(|line| {
            let first = re.find(line).unwrap().as_str();
            let last = last_ascii_match(line, &re);
            10 * map[first] + map[last]
        })
        .sum()
}
