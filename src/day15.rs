use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
enum Operation {
    Remove,
    Insert(u64),
}

#[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

impl<'a> Step<'a> {
    fn parse(s: &'a str) -> Self {
        let (label, operation) = if s.contains('=') {
            let (label, focal_length) = s.split_once('=').unwrap();
            (label, Operation::Insert(focal_length.parse().unwrap()))
        } else {
            let (label, _) = s.split_once('-').unwrap();
            (label, Operation::Remove)
        };
        Step { label, operation }
    }
}

#[derive(Default, Clone)]
struct LenseBox<'a> {
    focal_lengths: HashMap<&'a str, u64>,
    labels: Vec<&'a str>,
}

impl<'a> LenseBox<'a> {
    fn insert(&mut self, label: &'a str, focal_length: u64) {
        match self.focal_lengths.entry(label) {
            Entry::Vacant(v) => {
                v.insert(focal_length);
                self.labels.push(label);
            }
            Entry::Occupied(mut o) => {
                o.insert(focal_length);
            }
        }
    }

    fn remove(&mut self, label: &str) {
        if self.focal_lengths.remove(label).is_some() {
            self.labels.retain(|l| l != &label);
        }
    }

    fn focusing_power(&self, index: u64) -> u64 {
        self.labels
            .iter()
            .zip(1..)
            .map(|(label, slot)| index * slot * self.focal_lengths[label])
            .sum()
    }
}

fn hash(s: &str) -> u64 {
    let mut x = 0_u8;
    for c in s.as_bytes() {
        x = x.wrapping_add(*c).wrapping_mul(17);
    }
    x as u64
}

pub fn part1(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

pub fn part2(input: &str) -> u64 {
    let mut boxes = vec![LenseBox::default(); 256];

    for Step { label, operation } in input.split(',').map(Step::parse) {
        let index = hash(label) as usize;
        match operation {
            Operation::Remove => boxes[index].remove(label),
            Operation::Insert(focal_length) => boxes[index].insert(label, focal_length),
        }
    }

    boxes
        .iter()
        .zip(1..)
        .map(|(b, index)| b.focusing_power(index))
        .sum()
}
