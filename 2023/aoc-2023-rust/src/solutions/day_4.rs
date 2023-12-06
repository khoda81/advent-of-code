use std::{collections::HashSet, io::BufRead};
use thiserror::Error;

use crate::{Part, PuzzleResult};

pub fn main<I: BufRead>(buf_reader: I, part: Part) -> PuzzleResult {
    let cards: Vec<_> = buf_reader
        .lines()
        .map(|line| {
            let line = line?;
            let line = line.trim();

            parse_line(line)
        })
        .collect::<Result<_, _>>()?;

    let total = match part {
        Part::One => cards.iter().map(|card| card.evaluate()).sum(),
        Part::Two => {
            let mut wins = vec![1; cards.len()];
            for (idx, card) in cards.into_iter().enumerate().rev() {
                let num_matches = card.count_matches();
                let last_idx = (idx + num_matches + 1).min(wins.len());
                wins[idx] = wins[idx..last_idx].iter().sum();
            }

            wins.iter().sum()
        }
    };

    Ok(total)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Card {
    set1: HashSet<u32>,
    set2: Vec<u32>,
}

impl Card {
    fn evaluate(&self) -> u32 {
        let num_matches = self.count_matches();

        if num_matches > 0 {
            2_u32.pow(num_matches as u32 - 1)
        } else {
            0
        }
    }

    fn count_matches(&self) -> usize {
        self.set2
            .iter()
            .filter(|num| self.set1.contains(num))
            .count()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
enum ParseCardError {
    #[error("the line did not contain the string \": \"\nLine: {0}")]
    NoColon(String),

    #[error("the part after the colon did not contain \" | \"\nLine: {0}")]
    NoBar(String),
}

fn parse_line(line: &str) -> anyhow::Result<Card> {
    let (_card_number, sets) = line
        .split_once(": ")
        .ok_or_else(|| ParseCardError::NoColon(line.to_string()))?;

    let (set1_str, set2_str) = sets
        .split_once(" | ")
        .ok_or_else(|| ParseCardError::NoBar(line.to_string()))?;

    Ok(Card {
        set1: set1_str
            .split_ascii_whitespace()
            .map(|str| str.parse())
            .collect::<Result<_, _>>()?,
        set2: set2_str
            .split_ascii_whitespace()
            .map(|str| str.parse())
            .collect::<Result<_, _>>()?,
    })
}
