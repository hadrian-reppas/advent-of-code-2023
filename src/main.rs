#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("{}", day7::part1(&input));
    println!("{}", day7::part2(&input));
}
