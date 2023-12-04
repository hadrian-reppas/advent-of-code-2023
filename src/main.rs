#![allow(dead_code)]

mod day1;
mod day2;
mod day3;

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    println!("{}", day3::part1(&input));
    println!("{}", day3::part2(&input));
}
