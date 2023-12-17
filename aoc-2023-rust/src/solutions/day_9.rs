use crate::{Part, PuzzleResult};
use itertools::Itertools;
use std::io::BufRead;

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut total = 0;

    for line in input.lines() {
        let seq = parse_line(&line?)?;

        total += match part {
            Part::One => extrapolate(seq),
            Part::Two => extrapolate(seq.into_iter().rev()),
        };
    }

    // Part 1: 1938800261
    // Part 2: 1112
    Ok(total.to_string())
}

pub fn extrapolate<I>(seq: I) -> i32
where
    I: IntoIterator<Item = i32>,
    I::IntoIter: DoubleEndedIterator + Clone,
{
    let seq = seq.into_iter();

    let Some(last) = seq.clone().next_back() else {
        println!("Extrapolating empty seq!");
        return 0;
    };

    if seq.clone().all_equal() {
        return last;
    }

    let diffs = seq.tuple_windows().map(|(a, b)| b - a).collect_vec();
    extrapolate(diffs) + last
}

fn parse_line(line: &str) -> anyhow::Result<Vec<i32>> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().map_err(Into::into))
        .collect()
}
