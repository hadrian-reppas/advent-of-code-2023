use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Block {
    x: (u64, u64),
    y: (u64, u64),
    z: (u64, u64),
}

impl Block {
    fn parse(s: &str) -> Self {
        let (lhs, rhs) = s.split_once('~').unwrap();
        let (mut lhs, mut rhs) = (lhs.split(','), rhs.split(','));
        macro_rules! range {
            () => {
                (
                    lhs.next().unwrap().parse::<u64>().unwrap(),
                    rhs.next().unwrap().parse::<u64>().unwrap(),
                )
            };
        }
        Block {
            x: range!(),
            y: range!(),
            z: range!(),
        }
    }

    fn down(self) -> Option<Self> {
        if self.z.0 <= 1 {
            None
        } else {
            Some(Block {
                x: self.x,
                y: self.y,
                z: (self.z.0 - 1, self.z.1 - 1),
            })
        }
    }

    fn intersects(self, other: Self) -> bool {
        self.x.0 <= other.x.1
            && other.x.0 <= self.x.1
            && self.y.0 <= other.y.1
            && other.y.0 <= self.y.1
            && self.z.0 <= other.z.1
            && other.z.0 <= self.z.1
    }

    fn supports(self, other: Self) -> bool {
        self.x.0 <= other.x.1
            && other.x.0 <= self.x.1
            && self.y.0 <= other.y.1
            && other.y.0 <= self.y.1
            && self.z.1 + 1 == other.z.0
    }
}

fn collect_blocks(input: &str) -> Vec<Block> {
    let mut blocks: Vec<_> = input.lines().map(Block::parse).collect();
    blocks.sort_unstable_by_key(|b| b.z.0);

    for i in 0..blocks.len() {
        let (left, right) = blocks.split_at_mut(i);
        let (block, right) = right.split_first_mut().unwrap();
        let mut new = *block;

        while let Some(down) = new.down() {
            if left.iter().chain(right.iter()).any(|b| b.intersects(down)) {
                break;
            }
            new = down;
        }
        *block = new;
    }

    blocks
}

fn get_supported_by(blocks: &[Block]) -> HashMap<Block, Vec<Block>> {
    let mut supported_by: HashMap<_, _> = blocks.iter().map(|b| (*b, Vec::new())).collect();
    for block in blocks {
        for other in blocks {
            if other.supports(*block) {
                supported_by.get_mut(block).unwrap().push(*other);
            }
        }
    }
    supported_by
}

fn chain_reaction(alive: &mut HashSet<Block>, supported_by: &HashMap<Block, Vec<Block>>) {
    let mut done = false;
    while !done {
        done = true;
        for (block, support) in supported_by {
            if alive.contains(block)
                && !support.is_empty()
                && support.iter().all(|b| !alive.contains(b))
            {
                alive.remove(block);
                done = false;
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let blocks = collect_blocks(input);
    let supported_by = get_supported_by(&blocks);

    let mut important = HashSet::new();
    for blocks in supported_by.values() {
        if blocks.len() == 1 {
            important.insert(blocks[0]);
        }
    }

    (supported_by.len() - important.len()) as u64
}

pub fn part2(input: &str) -> u64 {
    let blocks = collect_blocks(input);
    let supported_by = get_supported_by(&blocks);

    let mut total = 0;
    for block in &blocks {
        let mut alive: HashSet<_> = blocks.iter().copied().collect();
        alive.remove(block);
        chain_reaction(&mut alive, &supported_by);
        total += (blocks.len() - alive.len() - 1) as u64;
    }
    total
}
