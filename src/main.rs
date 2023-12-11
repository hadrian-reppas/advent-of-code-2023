#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("{}", day8::part1(&input));
    println!("{}", day8::part2(&input));
}
