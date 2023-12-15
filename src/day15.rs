enum Operation {
    Remove,
    Insert(u64),
}

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
            (&s[..s.len() - 1], Operation::Remove)
        };
        Step { label, operation }
    }
}

#[derive(Clone)]
struct Pair<'a> {
    label: &'a str,
    focal_length: u64,
}

#[derive(Clone)]
struct LenseBox<'a>(Vec<Pair<'a>>);

impl<'a> LenseBox<'a> {
    fn insert(&mut self, label: &'a str, focal_length: u64) {
        if let Some(index) = self.0.iter().position(|p| p.label == label) {
            self.0[index].focal_length = focal_length;
        } else {
            self.0.push(Pair {
                label,
                focal_length,
            });
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(index) = self.0.iter().position(|p| p.label == label) {
            self.0.remove(index);
        }
    }

    fn focusing_power(&self, index: u64) -> u64 {
        self.0
            .iter()
            .zip(1..)
            .map(|(pair, slot)| index * slot * pair.focal_length)
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
    let mut boxes = vec![LenseBox(Vec::new()); 256];

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
