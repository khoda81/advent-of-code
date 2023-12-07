use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use strum_macros::EnumIter;
use thiserror::Error;

pub mod solutions;

pub const INPUT_FILES_PATH: &str = r"..";

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, EnumIter)]
pub enum Part {
    #[default]
    One = 1,
    Two = 2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = format!("Part {}", *self as u8);
        buf.fmt(f)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum Day {
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
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
    TwentyFour = 24,
    TwentyFive = 25,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = format!("Day {}", self.number());
        buf.fmt(f)
    }
}

pub type PuzzleOutput = u32;
pub type PuzzleResult = anyhow::Result<PuzzleOutput>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
#[error("the solution for {0} is not implemented yet")]
pub struct UnimplementedSolution(Day);

impl Day {
    pub fn number(self) -> u8 {
        self as u8
    }

    pub fn input_path(&self) -> PathBuf {
        PathBuf::from_iter([INPUT_FILES_PATH, &self.number().to_string(), "input.txt"])
    }

    pub fn solve(&self, part: Part) -> PuzzleResult {
        let path = self.input_path();
        let input_file = File::open(path.clone()).with_context(|| {
            let invalid_path_msg = "Failed: path had invalid unicode";
            let path = path.to_str().unwrap_or(invalid_path_msg);
            let cwd = match std::env::current_dir() {
                Ok(cwd) => cwd.to_str().unwrap_or(invalid_path_msg).to_string(),
                Err(err) => format!("Failed: {err}"),
            };

            format!("while opening file \"{path}\" (cwd: \"{cwd}\")")
        })?;

        let buf_reader = BufReader::new(input_file);
        self.solve_with_input(buf_reader, part)
    }

    fn solve_with_input<I: BufRead>(&self, buf_reader: I, part: Part) -> PuzzleResult {
        match self {
            Day::One => solutions::day_1::main(buf_reader, part),
            Day::Two => solutions::day_2::main(buf_reader, part),
            Day::Three => solutions::day_3::main(buf_reader, part),
            Day::Four => solutions::day_4::main(buf_reader, part),
            Day::Seven => solutions::day_7::main(buf_reader, part),
            _ => Err(UnimplementedSolution(*self).into()),
        }
    }
}
