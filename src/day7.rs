use std::any::{Any, TypeId};
use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardPart1 {
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum CardPart2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

macro_rules! card_impl {
    ($card:ident, $j:ident) => {
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
                    'J' => $card::$j,
                    'Q' => $card::Queen,
                    'K' => $card::King,
                    'A' => $card::Ace,
                    _ => unreachable!(),
                }
            }
        }

        impl From<$card> for usize {
            fn from(card: $card) -> Self {
                card as _
            }
        }
    };
}

card_impl!(CardPart1, Jack);
card_impl!(CardPart2, Joker);

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

        if TypeId::of::<C>() == TypeId::of::<CardPart2>() {
            counts[1..].sort();
            counts[12] += counts[0];
        } else {
            counts.sort();
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
        let (self_type, other_type) = (HandType::of(self.cards), HandType::of(other.cards));
        if self_type == other_type {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(self_type.cmp(&other_type))
        }
    }
}

impl<C> Ord for Hand<C>
where
    C: Ord + Into<usize> + Copy + Any,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand<CardPart1>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut hands: Vec<Hand<CardPart2>> = input.lines().map(Hand::parse).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
        .sum()
}
