use std::{fmt::Display, io::BufRead};

use strum_macros::{Display, EnumIter, EnumString};
use thiserror::Error;

pub mod solutions;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, EnumIter, EnumString)]
pub enum Part {
    #[default]
    #[strum(ascii_case_insensitive)]
    One = 1,

    #[strum(ascii_case_insensitive)]
    Two = 2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("Part {}", *self as u8).fmt(f)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, EnumString)]
pub enum Day {
    #[strum(serialize = "1")]
    Day1 = 1,
    #[strum(serialize = "2")]
    Day2 = 2,
    #[strum(serialize = "3")]
    Day3 = 3,
    #[strum(serialize = "4")]
    Day4 = 4,
    #[strum(serialize = "5")]
    Day5 = 5,
    #[strum(serialize = "6")]
    Day6 = 6,
    #[strum(serialize = "7")]
    Day7 = 7,
    #[strum(serialize = "8")]
    Day8 = 8,
    #[strum(serialize = "9")]
    Day9 = 9,
    #[strum(serialize = "10")]
    Day10 = 10,
    #[strum(serialize = "11")]
    Day11 = 11,
    #[strum(serialize = "12")]
    Day12 = 12,
    #[strum(serialize = "13")]
    Day13 = 13,
    #[strum(serialize = "14")]
    Day14 = 14,
    #[strum(serialize = "15")]
    Day15 = 15,
    #[strum(serialize = "16")]
    Day16 = 16,
    #[strum(serialize = "17")]
    Day17 = 17,
    #[strum(serialize = "18")]
    Day18 = 18,
    #[strum(serialize = "19")]
    Day19 = 19,
    #[strum(serialize = "20")]
    Day20 = 20,
    #[strum(serialize = "21")]
    Day21 = 21,
    #[strum(serialize = "22")]
    Day22 = 22,
    #[strum(serialize = "23")]
    Day23 = 23,
    #[strum(serialize = "24")]
    Day24 = 24,
    #[strum(serialize = "25")]
    Day25 = 25,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("Day {}", *self as u8).fmt(f)
    }
}

pub type PuzzleResult = anyhow::Result<String>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
#[error("the solution for {}-{} is not implemented yet", self.0 as u8, self.1 as u8)]
pub struct UnimplementedSolution(Day, Part);

impl Day {
    pub fn solve<I: BufRead>(&self, input: I, part: Part) -> PuzzleResult {
        match self {
            Day::Day1 => solutions::day_1::main(input, part),
            Day::Day2 => solutions::day_2::main(input, part),
            Day::Day3 => solutions::day_3::main(input, part),
            Day::Day4 => solutions::day_4::main(input, part),
            Day::Day5 => solutions::day_5::main(input, part),
            Day::Day6 => solutions::day_6::main(input, part),
            Day::Day7 => solutions::day_7::main(input, part),
            Day::Day8 => solutions::day_8::main(input, part),
            Day::Day9 => solutions::day_9::main(input, part),
            Day::Day10 => solutions::day_10::main(input, part),
            Day::Day11 => solutions::day_11::main(input, part),

            _ => Err(UnimplementedSolution(*self, part).into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumString, Display)]
pub enum Year {
    #[strum(serialize = "2015")]
    Year2015 = 2015,
    #[strum(serialize = "2016")]
    Year2016 = 2016,
    #[strum(serialize = "2017")]
    Year2017 = 2017,
    #[strum(serialize = "2018")]
    Year2018 = 2018,
    #[strum(serialize = "2019")]
    Year2019 = 2019,
    #[strum(serialize = "2020")]
    Year2020 = 2020,
    #[strum(serialize = "2021")]
    Year2021 = 2021,
    #[strum(serialize = "2022")]
    Year2022 = 2022,
    #[strum(serialize = "2023")]
    Year2023 = 2023,
}
