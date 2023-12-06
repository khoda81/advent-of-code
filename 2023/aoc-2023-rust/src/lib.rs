use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use strum_macros::EnumIter;

pub mod day_1;
pub mod day_2;
pub mod day_3;

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

type PuzzleOutput = u32;
type PuzzleResult = anyhow::Result<PuzzleOutput>;

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
        self.solver_fn()(buf_reader, part)
    }

    pub fn solver_fn<I: BufRead>(&self) -> fn(I, Part) -> PuzzleResult {
        match self {
            Day::One => day_1::main,
            Day::Two => day_2::main,
            Day::Three => day_3::main,
            _ => todo!(),
        }
    }
}
