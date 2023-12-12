use regex::Regex;
use std::io::BufRead;
use thiserror::Error;

use crate::{Part, PuzzleOutput, PuzzleResult};

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut total = 0;
    for line in input.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        let [first, last] = match part {
            Part::One => extract_digits_part_1(trimmed_line)?,
            Part::Two => extract_digits_part_2(trimmed_line)?,
        };

        let num = first * 10 + last;
        total += num as u32;
    }

    Ok(PuzzleOutput::try_from(total)?)
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Error)]
#[error("given str slice contains no digits")]
struct NoDigitsInStr;

fn extract_digits_part_1(line: &str) -> anyhow::Result<[u8; 2]> {
    let mut digits = line.chars().filter(|chr| chr.is_numeric());

    let first = digits.next().ok_or(NoDigitsInStr)?;
    let last = digits.last().unwrap_or(first);
    let [first, last] = [first, last].map(|d| d.to_string());

    Ok([first.as_str(), last.as_str()].map(digit_to_int))
}

fn extract_digits_part_2(line: &str) -> anyhow::Result<[u8; 2]> {
    let forward_re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let backward_re = Regex::new(r"([1-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();

    let first = forward_re.find(line).ok_or(NoDigitsInStr)?.as_str();

    let reversed_line: String = line.chars().rev().collect();
    let last = backward_re
        .find(&reversed_line)
        .map(|m| m.as_str().chars().rev().collect::<String>());

    let last = last.as_deref().unwrap_or(first);

    Ok([first, last].map(digit_to_int))
}

fn digit_to_int(digit: &str) -> u8 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        d => d.parse().unwrap(),
    }
}
