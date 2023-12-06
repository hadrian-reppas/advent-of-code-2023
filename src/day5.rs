use std::ops::Range;
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

    fn map(&self, x: u64) -> Option<u64> {
        if x >= self.source && x < self.source + self.length {
            Some(x - self.source + self.destination)
        } else {
            None
        }
    }

    fn map_inclusive(&self, x: u64) -> Option<u64> {
        if x >= self.source && x <= self.source + self.length {
            Some(x - self.source + self.destination)
        } else {
            None
        }
    }

    fn map_range(&self, range: Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
        let (source_start, source_end) = (self.source, self.source + self.length);
        let (dest_start, dest_end) = (self.destination, self.destination + self.length);
        let (start, end) = (range.start, range.end);

        if end <= source_start {
            (vec![range], None)
        } else if start < source_start && end <= source_end {
            (
                vec![start..source_start],
                Some(dest_start..self.map_inclusive(end).unwrap()),
            )
        } else if start < source_start && end > source_end {
            (
                vec![start..source_start, source_end..end],
                Some(dest_start..dest_end),
            )
        } else if start >= source_start && end <= source_end {
            (
                Vec::new(),
                Some(self.map(start).unwrap()..self.map_inclusive(end).unwrap()),
            )
        } else if start >= source_start && start < source_end && end > source_end {
            (
                vec![source_end..end],
                Some(self.map(start).unwrap()..dest_end),
            )
        } else {
            (vec![range], None)
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
            if let Some(mapped) = segment.map(x) {
                return mapped;
            }
        }
        x
    }

    fn map_set(&self, set: &RangeSet) -> RangeSet {
        RangeSet(
            set.0
                .iter()
                .cloned()
                .flat_map(|r| self.map_range(r))
                .collect(),
        )
    }

    fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let (mut todo, mut out) = (vec![range], Vec::new());
        for segment in &self.0 {
            todo = todo
                .into_iter()
                .flat_map(|range| {
                    let (todo, mapped) = segment.map_range(range);
                    out.extend(mapped);
                    todo
                })
                .collect();
        }
        out.extend(todo);
        out
    }
}

struct RangeSet(Vec<Range<u64>>);

impl RangeSet {
    fn min(&self) -> u64 {
        self.0.iter().map(|r| r.start).min().unwrap()
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
    let mut sets = Vec::new();
    for pair in seeds.chunks(2) {
        sets.push(RangeSet(vec![pair[0]..pair[0] + pair[1]]));
    }

    for map in sections.map(Map::parse) {
        for set in &mut sets {
            *set = map.map_set(set);
        }
    }

    sets.iter().map(RangeSet::min).min().unwrap()
}
