#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn of(cards: [Card; 5]) -> Self {
        let mut counts = [0; 13];
        for card in cards {
            counts[card as usize] += 1;
        }
        counts.sort();
        match counts {
            [.., 5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn parse(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        Hand {
            cards: cards
                .chars()
                .map(Card::parse)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bid: bid.parse().unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (self_type, other_type) = (HandType::of(self.cards), HandType::of(other.cards));
        if self_type == other_type {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(self_type.cmp(&other_type))
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part1(input: &str) -> u64 {
    let mut hands: Vec<_> = input.lines().map(Hand::parse).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
