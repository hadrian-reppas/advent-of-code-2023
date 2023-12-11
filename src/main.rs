#![allow(dead_code)]

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("{}", day11::part1(&input));
    println!("{}", day11::part2(&input));
}
