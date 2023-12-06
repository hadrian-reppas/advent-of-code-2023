#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    println!("{}", day4::part1(&input));
    println!("{}", day4::part2(&input));
}
