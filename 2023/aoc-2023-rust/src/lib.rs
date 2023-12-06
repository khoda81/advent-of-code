use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub mod day_1;
pub mod day_2;
pub mod day_3;

pub const INPUT_FILES_PATH: &str = r"../";

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Part {
    #[default]
    One = 1,
    Two = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

type PuzzleOutput = u32;
type PuzzleResult = anyhow::Result<PuzzleOutput>;

impl Day {
    fn number(&self) -> u8 {
        *self as u8
    }

    pub fn input_path(&self) -> PathBuf {
        PathBuf::from_iter([INPUT_FILES_PATH, &self.number().to_string(), "input.txt"])
    }

    pub fn get_solution<I: BufRead>(&self) -> fn(I, Part) -> PuzzleResult {
        match self {
            Day::One => day_1::main,
            Day::Two => day_2::main,
            Day::Three => day_3::main,
            _ => todo!(),
        }
    }

    pub fn solve(&self, part: Part) -> PuzzleResult {
        let path = self.input_path();
        let input_file = File::open(path)?;
        let buf_reader = BufReader::new(input_file);
        let solution = self.get_solution();

        solution(buf_reader, part)
    }
}
