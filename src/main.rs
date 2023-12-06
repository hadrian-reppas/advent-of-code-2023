#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("{}", day5::part1(&input));
    println!("{}", day5::part2(&input));
}
