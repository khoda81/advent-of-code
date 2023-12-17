use self::parser::{edges, Node};
use crate::{Part, PuzzleResult};
use anyhow::Context;
use num::Integer;
use std::{collections::HashMap, io::BufRead};
use thiserror::Error;

mod parser {
    use nom::{branch, character, combinator, multi, sequence, IResult, Parser as _};

    use character::streaming as cs;
    use sequence as s;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Instruction {
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

    pub fn parse_first_line(input: Inp) -> IResult<Inp, Vec<Instruction>> {
        let parse_direction = branch::alt((
            combinator::value(Instruction::Left, cs::char('L')),
            combinator::value(Instruction::Right, cs::char('R')),
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
    ExpectedInstructions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edges {
    left: String,
    right: String,
}

pub fn main<I: BufRead>(mut input: I, part: Part) -> PuzzleResult {
    let instructions = parse_instructions(&mut input)?;
    let graph = parse_graph(&mut input)?;

    match part {
        Part::One => part_1(instructions, graph),
        Part::Two => part_2(instructions, graph),
    }
}

fn part_1(instructions: Vec<parser::Instruction>, graph: HashMap<String, Edges>) -> PuzzleResult {
    let mut current = "AAA";
    let mut count = 0;

    while current != "ZZZ" {
        for instruction in instructions.iter() {
            let Some(edges) = graph.get(current) else {
                panic!("Node {current:?} does not exist in the graph")
            };

            match instruction {
                parser::Instruction::Left => current = edges.left.as_str(),
                parser::Instruction::Right => current = edges.right.as_str(),
            }
        }

        count += instructions.len();
    }

    // 17873
    Ok(count as u64)
}

fn part_2(instructions: Vec<parser::Instruction>, graph: HashMap<String, Edges>) -> PuzzleResult {
    let mut meta_graph: HashMap<&str, &str> = HashMap::new();
    let mut lcm = 1;

    for node in graph.keys().filter(|node| node.ends_with('A')) {
        let mut current = node.as_str();
        let mut cycle_length = 0;

        while !current.ends_with('Z') {
            current = *meta_graph.entry(current).or_insert_with_key(|&node| {
                let mut current = node;

                // Perform the instructions
                for instruction in instructions.iter() {
                    let Some(edges) = graph.get(current) else {
                        panic!("Node {current:?} does not exist in the graph")
                    };

                    match instruction {
                        parser::Instruction::Left => current = edges.left.as_str(),
                        parser::Instruction::Right => current = edges.right.as_str(),
                    }
                }

                current
            });

            cycle_length += 1;
        }

        lcm = lcm.lcm(&cycle_length);
    }

    let result = lcm * instructions.len();
    // 15746133679061
    Ok(result as u64)
}

fn parse_graph<I: BufRead>(input: I) -> anyhow::Result<HashMap<String, Edges>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
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
    Ok(graph)
}

fn parse_instructions<I: BufRead>(input: I) -> anyhow::Result<Vec<parser::Instruction>> {
    let mut lines = input.lines();
    let mut first_line = lines
        .next()
        .ok_or(UnexpectedEndOfInput::ExpectedInstructions)?
        .context("failed to read first line")?;
    first_line.push('\n');

    let (_, instructions) = parser::parse_first_line(&first_line)
        .map_err(|err| err.to_owned())
        .context("failed parse first line")?;

    Ok(instructions)
}
