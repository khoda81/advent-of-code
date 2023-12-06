use anyhow::Ok;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

use crate::{Part, PuzzleOutput, PuzzleResult};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Object {
    PartNumber(u32),
    Symbol(char),
}

enum GearStatus {
    NoNeighbors,
    OneNumber(u32),
    GearRatio(u32),
    Bust,
}

pub fn main<I: BufRead>(buf_reader: I, part: Part) -> PuzzleResult {
    let mut total = 0;
    let object_rows = buf_reader
        .lines()
        .map(|line| {
            let line = line?;
            let line = line.trim();

            parse_line(line)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut gears = HashMap::new();

    for (row_index, row) in object_rows.iter().enumerate() {
        for (column_index, object) in row {
            let Object::PartNumber(part_number) = *object else {
                continue;
            };

            let horizontal_size = part_number.ilog10() as usize + 1;
            let vertical_size = 1;

            let first_row = row_index.saturating_sub(1);
            let last_row = (row_index + vertical_size + 1).min(object_rows.len());
            let row_range = first_row..last_row;

            let first_column = column_index.saturating_sub(1);
            let last_column = column_index + horizontal_size + 1;
            let column_range = first_column..last_column;

            let collisions = object_rows
                .iter()
                .enumerate()
                .flat_map(|(row_idx, row)| {
                    row.iter()
                        .cloned()
                        .map(move |(col_idx, obj)| ((row_idx, col_idx), obj))
                })
                .filter(|((obj_row, obj_col), _)| {
                    row_range.contains(obj_row) && column_range.contains(obj_col)
                })
                .filter_map(|(pos, obj)| match obj {
                    Object::Symbol(chr) => Some((pos, chr)),
                    _ => None,
                });

            for (pos, obj_symbol) in collisions {
                if let Part::One = part {
                    total += part_number;
                    continue;
                }

                // Part two:

                if obj_symbol != '*' {
                    continue;
                }

                let status = gears.entry(pos).or_insert(GearStatus::NoNeighbors);

                *status = match *status {
                    GearStatus::NoNeighbors => GearStatus::OneNumber(part_number),
                    GearStatus::OneNumber(prev_number) => {
                        GearStatus::GearRatio(prev_number * part_number)
                    }
                    GearStatus::GearRatio(_) => GearStatus::Bust,
                    GearStatus::Bust => GearStatus::Bust,
                };
            }
        }
    }

    let ratios = gears.values().filter_map(|gear| match gear {
        GearStatus::GearRatio(ratio) => Some(ratio),
        _ => None,
    });

    for ratio in ratios {
        total += ratio;
    }

    Ok(PuzzleOutput::try_from(total)?)
}

fn parse_line(line: &str) -> anyhow::Result<Vec<(usize, Object)>> {
    let object_re = Regex::new(r"\d+|[^\.]")?;
    object_re
        .find_iter(line)
        .map(|object_match| {
            let object_str = object_match.as_str();
            let object = if let Result::Ok(num) = object_str.parse() {
                Object::PartNumber(num)
            } else {
                Object::Symbol(object_str.chars().next().unwrap())
            };

            Ok((object_match.start(), object))
        })
        .collect()
}
