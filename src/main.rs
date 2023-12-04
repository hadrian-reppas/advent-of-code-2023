#![allow(dead_code)]

mod day1;
mod day2;

fn main() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    println!("{}", day2::part1(&input));
    println!("{}", day2::part2(&input));
}
