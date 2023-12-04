use std::cmp::max;

struct Round {
    red: u64,
    green: u64,
    blue: u64,
}

impl Round {
    fn parse(s: &str) -> Self {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for pair in s.split(", ") {
            let (n, color) = pair.split_once(' ').unwrap();
            match color {
                "red" => round.red += n.parse::<u64>().unwrap(),
                "green" => round.green += n.parse::<u64>().unwrap(),
                "blue" => round.blue += n.parse::<u64>().unwrap(),
                _ => unreachable!(),
            }
        }

        round
    }

    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (prefix, suffix) = s.split_once(": ").unwrap();
        let (_, id) = prefix.split_once(' ').unwrap();
        Game {
            id: id.parse().unwrap(),
            rounds: suffix.split("; ").map(Round::parse).collect(),
        }
    }

    fn is_possible(&self) -> bool {
        self.rounds.iter().all(Round::is_possible)
    }

    fn power(&self) -> u64 {
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for round in &self.rounds {
            max_red = max(round.red, max_red);
            max_green = max(round.green, max_green);
            max_blue = max(round.blue, max_blue);
        }
        max_red * max_green * max_blue
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(Game::parse)
        .filter(Game::is_possible)
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().map(Game::parse).map(|g| g.power()).sum()
}
