use crate::{Day, Part, PuzzleResult, UnimplementedSolution};
use std::io::BufRead;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("could not find 'S' in the input")]
    StartNotFound,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
#[error("pipes are disconnected")]
pub struct DisconnectedPipes;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
#[error("pipe goes out the window")]
pub struct PipeGoesOutTheWindow;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::E => (0, 1),
            Direction::S => (1, 0),
            Direction::W => (0, -1),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

fn has_direction(pipe: u8, direction: Direction) -> bool {
    match direction {
        Direction::N => matches!(pipe, b'|' | b'J' | b'L'),
        Direction::E => matches!(pipe, b'-' | b'F' | b'L'),
        Direction::S => matches!(pipe, b'|' | b'F' | b'7'),
        Direction::W => matches!(pipe, b'-' | b'J' | b'7'),
    }
}

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let input: Vec<_> = input
        .lines()
        .map(|line| line.map(String::into_bytes))
        .collect::<Result<_, _>>()?;

    let (mut nr, mut nc) = input
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == b'S').map(|col| (row, col)))
        .ok_or(ParseError::StartNotFound)?;

    let mut direction;
    let mut next_pipe;

    (direction, next_pipe, (nr, nc)) = Direction::iter()
        .map(|direction| {
            let (dr, dc) = direction.offset();

            let nr = nr.wrapping_add(dr as isize as usize);
            let nc = nc.wrapping_add(dc as isize as usize);

            (direction, (nr, nc))
        })
        .filter_map(|(direction, (r, c))| {
            let neighbor_pipe = input.get(r).and_then(|line| line.get(c))?;

            Some((direction, *neighbor_pipe, (r, c)))
        })
        .find(|(direction, pipe, _)| has_direction(*pipe, direction.opposite()))
        .ok_or(DisconnectedPipes)?;

    let mut count = 1;

    while next_pipe != b'S' {
        direction = Direction::iter()
            // Can't go back the way we came
            .filter(|&new_direction| new_direction != direction.opposite())
            // Find the direction that this pipe has outgoing edge
            .find(|&new_direction| has_direction(next_pipe, new_direction))
            .ok_or(DisconnectedPipes)?;

        let (dr, dc) = direction.offset();

        nr = nr.wrapping_add(dr as isize as usize);
        nc = nc.wrapping_add(dc as isize as usize);

        next_pipe = *input
            .get(nr)
            .and_then(|line| line.get(nc))
            .ok_or(PipeGoesOutTheWindow)?;

        count += 1;
    }

    match part {
        // Part 1: 6947
        Part::One => return Ok((count / 2).to_string()),
        Part::Two => {}
    };

    Err(UnimplementedSolution(Day::Ten, part).into())
}
