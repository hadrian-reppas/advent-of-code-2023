mod day1;

fn main() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    println!("{}", day1::part1(&input));
    println!("{}", day1::part2(&input));
}
