#![allow(dead_code)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();
    println!("{}", day14::part1(&input));
    println!("{}", day14::part2(&input));
}
