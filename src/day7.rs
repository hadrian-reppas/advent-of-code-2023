use std::any::{Any, TypeId};
use std::fmt::Debug;

macro_rules! card_impl {
    ($card:ident, $($jack:ident)?, $($joker:ident)?) => {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
        enum $card {
            $($joker,)?
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            $($jack,)?
            Queen,
            King,
            Ace,
        }

        impl From<char> for $card {
            fn from(c: char) -> Self {
                match c {
                    '2' => $card::Two,
                    '3' => $card::Three,
                    '4' => $card::Four,
                    '5' => $card::Five,
                    '6' => $card::Six,
                    '7' => $card::Seven,
                    '8' => $card::Eight,
                    '9' => $card::Nine,
                    'T' => $card::Ten,
                    'J' => $card::$($jack)? $($joker)?,
                    'Q' => $card::Queen,
                    'K' => $card::King,
                    'A' => $card::Ace,
                    _ => unreachable!(),
                }
            }
        }

        impl From<$card> for usize {
            fn from(card: $card) -> Self {
                card as usize
            }
        }
    };
}

card_impl!(Part1Card, Jack,);
card_impl!(Part2Card,, Joker);

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
    fn of<C>(cards: [C; 5]) -> Self
    where
        C: Into<usize> + Any,
    {
        let mut counts = [0; 13];
        for card in cards {
            counts[card.into()] += 1;
        }

        if TypeId::of::<C>() == TypeId::of::<Part2Card>() {
            counts[1..].sort_unstable();
            counts[12] += counts[0];
        } else {
            counts.sort_unstable();
        }

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
struct Hand<C> {
    cards: [C; 5],
    bid: u64,
}

impl<C> Hand<C>
where
    C: From<char> + Debug,
{
    fn parse(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        Hand {
            cards: cards
                .chars()
                .map(C::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bid: bid.parse().unwrap(),
        }
    }
}

impl<C> PartialOrd for Hand<C>
where
    C: Ord + Into<usize> + Copy + Any,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for Hand<C>
where
    C: Ord + Into<usize> + Copy + Any,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (self_type, other_type) = (HandType::of(self.cards), HandType::of(other.cards));
        if self_type == other_type {
            self.cards.cmp(&other.cards)
        } else {
            self_type.cmp(&other_type)
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand<Part1Card>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut hands: Vec<Hand<Part2Card>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}
