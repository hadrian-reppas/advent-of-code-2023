use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Clone, Copy, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(s: &str) -> Self {
        let mut parts = s[1..s.len() - 1].split(',');
        Self {
            x: parts.next().unwrap()[2..].parse().unwrap(),
            m: parts.next().unwrap()[2..].parse().unwrap(),
            a: parts.next().unwrap()[2..].parse().unwrap(),
            s: parts.next().unwrap()[2..].parse().unwrap(),
        }
    }

    fn total_rating(self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn get_id<'a>(name: &'a str, map: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(id) = map.get(name) {
        *id
    } else {
        let id = map.len();
        map.insert(name, id);
        id
    }
}

#[derive(Clone, Debug)]
struct Workflow {
    rules: Vec<Rule>,
    fallback: Destination,
}

impl Workflow {
    fn parse<'a>(s: &'a str, map: &mut HashMap<&'a str, usize>) -> (usize, Self) {
        let (name, rhs) = s[..s.len() - 1].split_once('{').unwrap();
        let id = get_id(name, map);
        let mut rules = rhs.split(',');
        let fallback = Destination::parse(rules.next_back().unwrap(), map);
        (
            id,
            Workflow {
                rules: rules.map(|r| Rule::parse(r, map)).collect(),
                fallback,
            },
        )
    }

    fn handle(&self, part: Part) -> Destination {
        for rule in &self.rules {
            if let Some(dest) = rule.handle(part) {
                return dest;
            }
        }
        self.fallback
    }

    fn x_values(&self) -> impl Iterator<Item = u64> + '_ {
        self.rules
            .iter()
            .filter(|r| r.field == Field::X)
            .map(move |r| r.value + u64::from(!r.less_than))
    }

    fn m_values(&self) -> impl Iterator<Item = u64> + '_ {
        self.rules
            .iter()
            .filter(|r| r.field == Field::M)
            .map(move |r| r.value + u64::from(!r.less_than))
    }

    fn a_values(&self) -> impl Iterator<Item = u64> + '_ {
        self.rules
            .iter()
            .filter(|r| r.field == Field::A)
            .map(move |r| r.value + u64::from(!r.less_than))
    }

    fn s_values(&self) -> impl Iterator<Item = u64> + '_ {
        self.rules
            .iter()
            .filter(|r| r.field == Field::S)
            .map(move |r| r.value + u64::from(!r.less_than))
    }
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    field: Field,
    value: u64,
    less_than: bool,
    destination: Destination,
}

impl Rule {
    fn parse<'a>(s: &'a str, map: &mut HashMap<&'a str, usize>) -> Self {
        let field = Field::parse(&s[..1]);
        let less_than = &s[1..2] == "<";
        let (value, destination) = s[2..].split_once(':').unwrap();
        Rule {
            field,
            value: value.parse().unwrap(),
            less_than,
            destination: Destination::parse(destination, map),
        }
    }

    fn handle(self, part: Part) -> Option<Destination> {
        let part_value = match self.field {
            Field::X => part.x,
            Field::M => part.m,
            Field::A => part.a,
            Field::S => part.s,
        };

        let lt_match = self.less_than && part_value < self.value;
        let gt_match = !self.less_than && part_value > self.value;
        (lt_match || gt_match).then_some(self.destination)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn parse(s: &str) -> Self {
        match s {
            "x" => Field::X,
            "m" => Field::M,
            "a" => Field::A,
            "s" => Field::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Accepted,
    Rejected,
    Other(usize),
}

impl Destination {
    fn parse<'a>(name: &'a str, map: &mut HashMap<&'a str, usize>) -> Self {
        match name {
            "A" => Destination::Accepted,
            "R" => Destination::Rejected,
            _ => Destination::Other(get_id(name, map)),
        }
    }
}

fn accept_part(part: Part, workflows: &[Workflow]) -> bool {
    let mut dest = Destination::Other(0);
    while let Destination::Other(id) = dest {
        dest = workflows[id].handle(part);
    }
    matches!(dest, Destination::Accepted)
}

pub fn part1(input: &str) -> u64 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::from([("in", 0)]);
    let workflows: BTreeMap<_, _> = workflows
        .lines()
        .map(|l| Workflow::parse(l, &mut map))
        .collect();
    let workflows: Vec<_> = workflows.into_values().collect();

    parts
        .lines()
        .map(Part::parse)
        .filter(|part| accept_part(*part, &workflows))
        .map(Part::total_rating)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::from([("in", 0)]);
    let workflows: BTreeMap<_, _> = workflows
        .lines()
        .map(|l| Workflow::parse(l, &mut map))
        .collect();
    let workflows: Vec<_> = workflows.into_values().collect();

    let x_values: BTreeSet<_> = workflows.iter().flat_map(Workflow::x_values).collect();
    let m_values: BTreeSet<_> = workflows.iter().flat_map(Workflow::m_values).collect();
    let a_values: BTreeSet<_> = workflows.iter().flat_map(Workflow::a_values).collect();
    let s_values: BTreeSet<_> = workflows.iter().flat_map(Workflow::s_values).collect();
    let x_values: Vec<_> = x_values.into_iter().chain(Some(4001)).collect();
    let m_values: Vec<_> = m_values.into_iter().chain(Some(4001)).collect();
    let a_values: Vec<_> = a_values.into_iter().chain(Some(4001)).collect();
    let s_values: Vec<_> = s_values.into_iter().chain(Some(4001)).collect();

    let mut count = 0;
    let mut x = 1;
    for &next_x in &x_values {
        let mut m = 1;
        for &next_m in &m_values {
            let mut a = 1;
            for &next_a in &a_values {
                let mut s = 1;
                for &next_s in &s_values {
                    let part = Part { x, m, a, s };
                    if accept_part(part, &workflows) {
                        count += (next_x - x) * (next_m - m) * (next_a - a) * (next_s - s);
                    }
                    s = next_s;
                }
                a = next_a;
            }
            m = next_m;
        }
        x = next_x;
    }

    count
}
