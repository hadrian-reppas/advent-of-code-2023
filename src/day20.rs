use std::collections::{HashMap, VecDeque};
use std::iter::repeat_with;

#[derive(Clone, Debug)]
struct Setup<'a> {
    broadcaster: Vec<&'a str>,
    flip_flops: HashMap<&'a str, FlipFlop<'a>>,
    conjunctions: HashMap<&'a str, Conjunction<'a>>,
}

impl<'a> Setup<'a> {
    fn parse(input: &'a str) -> Self {
        let mut broadcaster = Vec::new();
        let mut flip_flops = HashMap::new();
        let mut conjunctions = HashMap::new();
        for line in input.lines() {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            if line.starts_with("%") {
                flip_flops.insert(
                    &lhs[1..],
                    FlipFlop {
                        is_on: false,
                        modules: rhs.split(", ").collect(),
                    },
                );
            } else if line.starts_with("&") {
                conjunctions.insert(
                    &lhs[1..],
                    Conjunction {
                        recent: HashMap::new(),
                        modules: rhs.split(", ").collect(),
                    },
                );
            } else {
                broadcaster.extend(rhs.split(", "));
            }
        }

        for (name, flip_flop) in &flip_flops {
            for module in &flip_flop.modules {
                if let Some(conj) = conjunctions.get_mut(module) {
                    conj.recent.insert(name, Pulse::Low);
                }
            }
        }
        for (name, conjunction) in conjunctions.clone() {
            for module in &conjunction.modules {
                if let Some(conj) = conjunctions.get_mut(module) {
                    conj.recent.insert(name, Pulse::Low);
                }
            }
        }

        Setup {
            broadcaster,
            flip_flops,
            conjunctions,
        }
    }

    fn button(&mut self) -> (u64, u64, bool) {
        let (mut low_pulses, mut high_pulses, mut rx_low, mut rx_high) = (1, 0, 0, 0);
        let mut queue: VecDeque<_> = self.broadcaster.iter().map(Signal::broadcaster).collect();
        while let Some(signal) = queue.pop_front() {
            if signal.pulse.is_high() {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if let Some(flip_flop) = self.flip_flops.get_mut(signal.to) {
                flip_flop.handle(signal, &mut queue);
            } else if let Some(conjunction) = self.conjunctions.get_mut(signal.to) {
                conjunction.handle(signal, &mut queue);
            } else if signal.to == "rx" {
                if signal.pulse.is_high() {
                    rx_high += 1;
                } else {
                    rx_low += 1;
                }
            }
        }
        (
            low_pulses,
            high_pulses,
            dbg!(rx_low) == 1 && dbg!(rx_high) == 0,
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

impl<'a> Signal<'a> {
    fn broadcaster(to: &&'a str) -> Self {
        Signal {
            from: "broadcaster",
            to: *to,
            pulse: Pulse::Low,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn is_high(self) -> bool {
        matches!(self, Pulse::High)
    }
}

#[derive(Clone, Debug)]
struct FlipFlop<'a> {
    is_on: bool,
    modules: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn handle(&mut self, signal: Signal<'a>, queue: &mut VecDeque<Signal<'a>>) {
        if signal.pulse.is_high() {
            return;
        }
        let pulse = if self.is_on { Pulse::Low } else { Pulse::High };
        self.is_on = !self.is_on;
        for module in &self.modules {
            queue.push_back(Signal {
                from: signal.to,
                to: module,
                pulse,
            });
        }
    }
}

#[derive(Clone, Debug)]
struct Conjunction<'a> {
    recent: HashMap<&'a str, Pulse>,
    modules: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    fn handle(&mut self, signal: Signal<'a>, queue: &mut VecDeque<Signal<'a>>) {
        self.recent.insert(signal.from, signal.pulse);
        let pulse = if self.recent.values().copied().all(Pulse::is_high) {
            Pulse::Low
        } else {
            Pulse::High
        };
        for module in &self.modules {
            queue.push_back(Signal {
                from: signal.to,
                to: module,
                pulse,
            });
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut setup = Setup::parse(input);
    let (low_pulses, high_pulses) = repeat_with(|| setup.button())
        .take(1000)
        .fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1));
    dbg!(low_pulses, high_pulses);
    low_pulses * high_pulses
}

pub fn part2(input: &str) -> u64 {
    let mut setup = Setup::parse(input);
    let times = repeat_with(|| setup.button())
        .take_while(|(_, _, done)| !done)
        .count() as u64;
    times + 1
}
