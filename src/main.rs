#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();
    println!("{}", day6::part1(&input));
    println!("{}", day6::part2(&input));
}
