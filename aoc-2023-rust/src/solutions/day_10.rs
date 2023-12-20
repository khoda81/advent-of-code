use crate::{Part, PuzzleResult};
use itertools::Itertools;
use std::io::BufRead;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("could not find 'S' in the input")]
    StartNotFound,

    #[error("no pipe is connected to 'S'")]
    LonelyStart,

    #[error("only one loop was connected to 'S'")]
    OpenLoopNearStart,

    #[error("pipes are disconnected at ({}, {})", .coord.0, .coord.1)]
    OpenLoop { coord: (usize, usize) },

    #[error("pipe goes out the window")]
    PipeGoesOutTheWindow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    #[rustfmt::skip]
    fn of(self, coord: (usize, usize)) -> (usize, usize) {
        let (r, c) = coord;

        match self {
            Direction::Up => (r - 1, c    ),
            Direction::Right => (r    , c + 1),
            Direction::Down => (r + 1, c    ),
            Direction::Left => (r    , c - 1),
        }
    }

    fn is_in(self, pipe: u8) -> bool {
        match self {
            Direction::Up => matches!(pipe, b'|' | b'J' | b'L'),
            Direction::Right => matches!(pipe, b'-' | b'F' | b'L'),
            Direction::Down => matches!(pipe, b'|' | b'F' | b'7'),
            Direction::Left => matches!(pipe, b'-' | b'J' | b'7'),
        }
    }
}

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut grid: Vec<_> = input
        .lines()
        .map(|line| line.map(String::into_bytes))
        .collect::<Result<_, _>>()?;

    let s_coord = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == b'S').map(|col| (row, col)))
        .ok_or(Error::StartNotFound)?;

    let get_cell = |(r, c): (usize, usize)| grid.get(r).and_then(|line| line.get(c)).copied();

    let mut s_directions = Direction::iter().filter_map(|direction| {
        let coord = direction.of(s_coord);
        let pipe = get_cell(coord)?;

        direction
            .opposite()
            .is_in(pipe)
            .then_some((direction, pipe, coord))
    });

    let first_pipe = s_directions.next().ok_or(Error::LonelyStart)?;
    let second_pipe = s_directions.next().ok_or(Error::OpenLoopNearStart)?;

    let (mut direction, mut next_pipe, mut coord) = first_pipe;

    let mut is_loop = grid
        .iter()
        .map(|line| vec![false; line.len()])
        .collect_vec();

    is_loop[coord.0][coord.1] = true;

    let mut count = 1;
    while next_pipe != b'S' {
        count += 1;

        direction = Direction::iter()
            // Can't go back the way we came
            .filter(|&new_direction| new_direction != direction.opposite())
            // Find the direction that this pipe has outgoing edge
            .find(|&new_direction| new_direction.is_in(next_pipe))
            .ok_or(Error::OpenLoop { coord })?;

        coord = direction.of(coord);
        next_pipe = get_cell(coord).ok_or(Error::PipeGoesOutTheWindow)?;
        is_loop[coord.0][coord.1] = true;
    }

    if let Part::One = part {
        // Part 1: 6947
        return Ok((count / 2).to_string());
    }

    grid[s_coord.0][s_coord.1] = b"|-JF7L"
        .iter()
        .copied()
        .find(|&pipe| first_pipe.0.is_in(pipe) && second_pipe.0.is_in(pipe))
        .expect("at least one of the pipes should replace 'S'");

    let mut count = 0;
    for (r, row) in grid.iter_mut().enumerate() {
        let mut up_in = false;
        let mut down_in = false;
        for (c, cell) in row.iter_mut().enumerate() {
            if is_loop[r][c] {
                up_in ^= Direction::Up.is_in(*cell);
                down_in ^= Direction::Down.is_in(*cell);
            } else if up_in && down_in {
                count += 1;
            }
        }
    }

    // Part 2: 273
    Ok(count.to_string())
}
