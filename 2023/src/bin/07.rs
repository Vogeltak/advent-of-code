use std::{cmp::Ordering, collections::HashMap};

use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn new(s: char, p2: bool) -> Result<Self> {
        match s {
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(match p2 {
                false => Self::Jack,
                true => Self::Joker,
            }),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(anyhow::anyhow!("failed to parse card")),
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.get_type().cmp(&other.get_type());
        match ordering {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_counts = HashMap::new();
        for c in self.cards.clone() {
            card_counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
        let joker_count = card_counts.get(&Card::Joker).cloned().unwrap_or(0);

        match card_counts.keys().len() {
            1 => HandType::Yahtzee,
            2 => {
                if card_counts.values().any(|n| *n == 4) {
                    return match joker_count > 0 {
                        true => HandType::Yahtzee,
                        false => HandType::FoaK,
                    };
                }
                match joker_count {
                    2 | 3 => HandType::Yahtzee,
                    _ => HandType::FullHouse,
                }
            }
            3 => {
                if card_counts.values().any(|n| *n == 3) {
                    return match joker_count {
                        1 | 3 => HandType::FoaK,
                        _ => HandType::ToaK,
                    };
                }
                match joker_count {
                    2 => HandType::FoaK,
                    1 => HandType::FullHouse,
                    _ => HandType::TwoPair,
                }
            }
            4 => match joker_count {
                1 | 2 => HandType::ToaK,
                _ => HandType::OnePair,
            },
            _ => match joker_count {
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ToaK,
    FullHouse,
    FoaK,
    Yahtzee,
}

impl Hand {
    fn new(s: &str, p2: bool) -> Result<Self> {
        let (cards, bid) = s
            .split_once(' ')
            .ok_or(anyhow::anyhow!("failed to split line"))?;
        let cards = cards
            .chars()
            .map(|c| Card::new(c, p2))
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let bid = bid.parse()?;

        Ok(Hand { cards, bid })
    }
}

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let (p1, p2) = [false, true]
        .iter()
        .map(|p2| {
            let mut hands = input
                .lines()
                .map(|l| Hand::new(l, *p2))
                .filter_map(Result::ok)
                .collect::<Vec<_>>();

            hands.sort_unstable();

            hands
                .iter()
                .enumerate()
                .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid)
        })
        .collect_tuple()
        .unwrap();

    (p1, p2)
}
