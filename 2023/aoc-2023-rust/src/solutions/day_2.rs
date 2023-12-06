use anyhow::Ok;
use regex::Regex;
use std::io::BufRead;

use crate::{Part, PuzzleOutput, PuzzleResult};

#[derive(Debug)]
struct Bag {
    green: u32,
    blue: u32,
    red: u32,
}

pub fn main<I: BufRead>(buf_reader: I, part: Part) -> PuzzleResult {
    let mut total = 0;
    for line in buf_reader.lines() {
        let line = line?;
        let line = line.trim();

        let (game_id, bags) = parse_line(line)?;
        if let Part::One = part {
            if bags.iter().all(is_possible) {
                total += game_id as u32;
            }
        } else {
            let max_blues = bags.iter().map(|bag| bag.blue).max().unwrap();
            let max_greens = bags.iter().map(|bag| bag.green).max().unwrap();
            let max_reds = bags.iter().map(|bag| bag.red).max().unwrap();

            total += max_blues * max_greens * max_reds;
        }
    }

    Ok(PuzzleOutput::try_from(total)?)
}

fn is_possible(bag: &Bag) -> bool {
    bag.red <= 12 && bag.green <= 13 && bag.blue <= 14
}

fn parse_line(line: &str) -> anyhow::Result<(usize, Vec<Bag>)> {
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    if let Some(captures) = re.captures(line) {
        let (_, [game_id, bags_str]) = captures.extract();
        let game_id: usize = game_id.parse()?;
        let bags = bags_str
            .trim()
            .split(';')
            .map(parse_bag)
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok((game_id, bags))
    } else {
        panic!("Invalid input format");
    }
}

fn parse_bag(bag_str: &str) -> anyhow::Result<Bag> {
    let mut bag = Bag {
        blue: 0,
        green: 0,
        red: 0,
    };

    bag_str.split(',').try_for_each(|balls| {
        let (count, color) = balls.trim().split_once(' ').unwrap();
        let count: u32 = count.parse()?;
        match color {
            "red" => bag.red += count,
            "blue" => bag.blue += count,
            "green" => bag.green += count,
            color => panic!("Invalid color {color}"),
        }

        Ok(())
    })?;

    Ok(bag)
}
