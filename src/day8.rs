use std::collections::HashMap;

fn parse(input: &str) -> (&[u8], HashMap<&str, (&str, &str)>) {
    let (directions, tree) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    for line in tree.lines() {
        map.insert(&line[..3], (&line[7..10], &line[12..15]));
    }
    (directions.as_bytes(), map)
}

fn distance<'a>(
    mut node: &'a str,
    directions: &[u8],
    tree: &HashMap<&str, (&'a str, &'a str)>,
    end: impl Fn(&str) -> bool,
) -> u64 {
    let mut index = 0;

    while !end(node) {
        node = if directions[index % directions.len()] == b'L' {
            tree[node].0
        } else {
            tree[node].1
        };
        index += 1;
    }

    index as u64
}

fn lcm(a: u64, b: u64) -> u64 {
    fn gcd(a: u64, b: u64) -> u64 {
        if a == 0 {
            b
        } else if b == 0 {
            a
        } else if a > b {
            gcd(a % b, b)
        } else {
            gcd(a, b % a)
        }
    }

    a / gcd(a, b) * b
}

pub fn part1(input: &str) -> u64 {
    let (directions, tree) = parse(input);
    distance("AAA", directions, &tree, |s| s == "ZZZ")
}

pub fn part2(input: &str) -> u64 {
    let (directions, tree) = parse(input);
    let nodes = tree.keys().copied().filter(|n| n.ends_with('A'));
    nodes
        .map(|node| distance(node, directions, &tree, |n| n.ends_with('Z')))
        .fold(1, lcm)
}
