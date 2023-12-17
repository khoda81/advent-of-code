use std::{collections::HashMap, io::BufRead, ops::AddAssign};

use crate::{Part, PuzzleResult};

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut hands = input
        .lines()
        .map(|line| {
            let line = line?;
            let line = line.trim();

            parse_line(line)
        })
        .collect::<Result<Vec<_>, _>>()?;

    if let Part::Two = part {
        hands
            .iter_mut()
            .flat_map(|(hand, _)| hand.0.iter_mut())
            .for_each(|card| {
                if let Card::Jack = card {
                    *card = Card::Joker
                }
            });
    }

    let mut hands: Vec<_> = hands
        .into_iter()
        .map(|(hand, bid)| (hand, hand.hand_type(), bid))
        .collect();

    hands.sort_by_key(|&(hand, kind, _)| (kind, hand.0));

    let total: u32 = (1..).zip(hands).map(|(rank, (_, _, bid))| rank * bid).sum();

    Ok(total.into())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Card {
    Joker = 1,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hand([Card; 5]);

impl Hand {
    pub fn counts(&self) -> HashMap<Card, u8> {
        self.0.into_iter().fold(HashMap::new(), |mut map, card| {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
            map
        })
    }

    fn hand_type(&self) -> HandType {
        let mut counts = self.counts();
        if let Some(joker_counts) = counts.remove(&Card::Joker) {
            let &highest_card = counts
                .iter()
                .max_by_key(|&(_card, count)| count)
                .map(|(card, _)| card)
                .unwrap_or(&Card::Joker);

            counts
                .entry(highest_card)
                .or_default()
                .add_assign(joker_counts);
        }

        let count_counts = counts.into_values().fold([0; 5], |mut counts, val| {
            counts[val as usize - 1] += 1;
            counts
        });

        HandType::from_counts(count_counts)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn from_counts(counts: [u8; 5]) -> Self {
        match counts {
            [_, 0, 0, 0, 0] => HandType::HighCard,
            [_, 1, 0, 0, 0] => HandType::OnePair,
            [_, _, 0, 0, 0] => HandType::TwoPair,
            [_, 0, _, 0, 0] => HandType::ThreeOfAKind,
            [_, _, _, 0, 0] => HandType::FullHouse,
            [_, _, _, _, 0] => HandType::FourOfAKind,
            [_, _, _, _, _] => HandType::FiveOfAKind,
        }
    }
}

fn parse_line(line: &str) -> anyhow::Result<(Hand, u32)> {
    let (hand_str, bid_str) = line.split_once(' ').unwrap();

    let bid = bid_str.parse()?;
    let mut hand = [Card::Two; 5];
    hand_str
        .chars()
        .map(|card_char| match card_char {
            '1' | 'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            _ => Card::Two,
        })
        .enumerate()
        .for_each(|(i, card)| hand[i] = card);

    Ok((Hand(hand), bid))
}
