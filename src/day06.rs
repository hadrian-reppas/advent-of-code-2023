use std::str::FromStr;

fn count_beats(time: u64, distance: u64) -> u64 {
    let mut beats = 0;
    for speed in 1..time {
        if (time - speed) * speed > distance {
            beats += 1;
        }
    }
    beats
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split(' ').flat_map(u64::from_str);
    let distances = lines.next().unwrap().split(' ').flat_map(u64::from_str);
    let mut margin = 1;
    for (time, distance) in times.zip(distances) {
        margin *= count_beats(time, distance);
    }
    margin
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap();
    count_beats(time, distance)
}
