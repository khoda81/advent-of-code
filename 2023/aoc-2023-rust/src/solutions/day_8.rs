use std::{collections::HashMap, io::BufRead};

use anyhow::Context;
use thiserror::Error;

use crate::{Day, Part, PuzzleResult, UnimplementedSolution};

use self::parser::{edges, Node};

mod parser {
    use nom::{branch, character, combinator, multi, sequence, IResult, Parser as _};

    use character::streaming as cs;
    use sequence as s;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Direction {
        Left,
        Right,
    }

    pub struct Node<'a> {
        pub name: &'a str,
        pub right: &'a str,
        pub left: &'a str,
    }

    // type Inp<'a> = &'a [u8];

    type Inp<'a> = &'a str;

    pub fn parse_first_line(input: Inp) -> IResult<Inp, Vec<Direction>> {
        let parse_direction = branch::alt((
            combinator::value(Direction::Left, cs::char('L')),
            combinator::value(Direction::Right, cs::char('R')),
        ));

        s::terminated(
            multi::many0(s::preceded(cs::space0, parse_direction)),
            s::preceded(cs::space0, cs::line_ending),
        )
        .parse(input)
    }

    pub fn edges(input: Inp) -> IResult<Inp, Option<Node>> {
        let identifier = cs::alphanumeric0;

        let pair = s::separated_pair(
            s::terminated(identifier, cs::space0),
            s::terminated(cs::char(','), cs::space0),
            identifier,
        );

        let tuple = s::delimited(
            s::terminated(cs::char('('), cs::space0),
            s::terminated(pair, cs::space0),
            cs::char(')'),
        );

        let assignment = s::separated_pair(
            s::terminated(identifier, cs::space0),
            s::terminated(cs::char('='), cs::space0),
            tuple,
        );

        s::terminated(
            s::preceded(cs::space0, combinator::opt(assignment)),
            s::preceded(cs::space0, cs::line_ending),
        )
        .parse(input)
        .map(|(rem, res)| {
            (
                rem,
                res.map(|(node, (left, right))| Node {
                    name: node,
                    right,
                    left,
                }),
            )
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
pub enum UnexpectedEndOfInput {
    #[error("expected a line of L and R s at the beginning")]
    ExpectedDirections,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edges {
    left: String,
    right: String,
}

pub fn main<I: BufRead>(input: I, part: Part) -> PuzzleResult {
    let mut lines = input.lines();
    let mut first_line = lines
        .next()
        .ok_or(UnexpectedEndOfInput::ExpectedDirections)?
        .context("failed to read first line")?;

    first_line.push('\n');

    let (_, directions) = parser::parse_first_line(&first_line)
        .map_err(|err| err.to_owned())
        .context("failed parse first line")?;

    let mut graph = HashMap::new();

    for line in lines {
        let mut line = line?;

        line.push('\n');

        if let (_, Some(Node { name, right, left })) = edges(&line)
            .map_err(|err| err.to_owned())
            .context("failed to parse assignment line")?
        {
            graph.insert(
                name.to_string(),
                Edges {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }
    }

    match part {
        Part::One => {
            let mut current = "AAA";
            let mut count = 0;

            while current != "ZZZ" {
                current = directions.iter().fold(current, |current, instruction| {
                    let edges = graph
                        .get(current)
                        .unwrap_or_else(|| panic!("Node {current} does not exist"));

                    match instruction {
                        parser::Direction::Left => edges.left.as_str(),
                        parser::Direction::Right => edges.right.as_str(),
                    }
                });

                count += 1;
            }

            Ok(count * directions.len() as u32)
        }
        Part::Two => Err(UnimplementedSolution(Day::Eight, part).into()),
    }
}
