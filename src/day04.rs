fn parse(s: &str) -> Option<u128> {
    s.parse().ok()
}

fn get_overlap(card: &str) -> u32 {
    let (winning, numbers) = card.split_once(" | ").unwrap();
    let (mut winning_mask, mut number_mask) = (0_u128, 0_u128);
    for win in winning.split(' ').filter_map(parse) {
        winning_mask |= 1 << win;
    }
    for number in numbers.split(' ').filter_map(parse) {
        number_mask |= 1 << number;
    }

    (winning_mask & number_mask).count_ones()
}

pub fn part1(input: &str) -> u64 {
    let mut points = 0;

    for line in input.lines() {
        let overlap = get_overlap(&line[10..]);
        points += if overlap == 0 { 0 } else { 1 << (overlap - 1) };
    }

    points
}

pub fn part2(input: &str) -> u64 {
    let mut copies = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let overlap = get_overlap(&line[10..]) as usize;
        for j in 0..overlap {
            copies[i + j + 1] += copies[i];
        }
    }

    copies.into_iter().sum()
}
