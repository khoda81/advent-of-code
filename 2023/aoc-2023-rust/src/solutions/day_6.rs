use std::io::BufRead;

use crate::{Part, PuzzleResult};

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut lines = input.lines();

    let times = parse_line(lines.next().unwrap()?, part)?;
    let distances = parse_line(lines.next().unwrap()?, part)?;

    let total: u32 = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            (0..=time)
                .filter(|hold| hold * (time - hold) > distance)
                .count() as u32
        })
        .product();

    Ok(total.to_string())
}

fn parse_line(line: String, part: Part) -> Result<Vec<u64>, anyhow::Error> {
    let (_, numbers) = line.split_once(':').unwrap();
    let numbers = numbers.split_ascii_whitespace();

    let numbers = if let Part::Two = part {
        let num = numbers.collect::<String>();
        vec![num]
    } else {
        numbers.map(|num| num.to_string()).collect()
    };

    let numbers: Vec<u64> = numbers
        .into_iter()
        .map(|num| num.parse())
        .collect::<Result<_, _>>()?;

    Ok(numbers)
}
