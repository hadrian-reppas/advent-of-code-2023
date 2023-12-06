use std::cmp::min;
use std::str::FromStr;

struct Segment {
    destination: u64,
    source: u64,
    length: u64,
}

impl Segment {
    fn parse(s: &str) -> Self {
        let mut ints = s.split(' ').flat_map(u64::from_str);
        Segment {
            destination: ints.next().unwrap(),
            source: ints.next().unwrap(),
            length: ints.next().unwrap(),
        }
    }
}

struct Map(Vec<Segment>);

impl Map {
    fn parse(s: &str) -> Self {
        Map(s.lines().skip(1).map(Segment::parse).collect())
    }

    fn map(&self, x: u64) -> u64 {
        for segment in &self.0 {
            if x >= segment.source && x < segment.source + segment.length {
                return x - segment.source + segment.destination;
            }
        }
        x
    }
}

pub fn part1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seed_line = sections.next().unwrap();
    let mut seeds: Vec<_> = seed_line
        .split(' ')
        .skip(1)
        .flat_map(u64::from_str)
        .collect();

    for map in sections.map(Map::parse) {
        for seed in &mut seeds {
            *seed = map.map(*seed);
        }
    }

    seeds.into_iter().min().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seed_line = sections.next().unwrap();
    let seeds: Vec<_> = seed_line
        .split(' ')
        .skip(1)
        .flat_map(u64::from_str)
        .collect();
    let maps: Vec<_> = sections.map(Map::parse).collect();

    let mut min_location = u64::MAX;
    for pair in seeds.chunks(2) {
        for mut seed in pair[0]..(pair[0] + pair[1]) {
            for map in &maps {
                seed = map.map(seed);
            }
            min_location = min(seed, min_location);
        }
    }

    min_location
}
