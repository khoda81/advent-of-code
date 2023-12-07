use std::{
    fmt::Display,
    io::{self, BufRead},
    ops::{ControlFlow as CF, RangeInclusive},
};
use thiserror::Error;

use crate::{Part, PuzzleResult};

type Range<T> = RangeInclusive<T>;

pub fn main<I: BufRead>(mut input: I, part: Part) -> PuzzleResult {
    let numbers = read_numbers(&mut input)?;

    let mut ranges = match part {
        Part::One => numbers.into_iter().map(|num| num..=num).collect(),
        Part::Two => numbers
            .chunks_exact(2)
            .map(<[_; 2]>::try_from)
            .map(|arr| arr.map(|[a, b]| a..=a + b - 1))
            .collect::<Result<_, _>>()?,
    };

    while let CF::Continue(mapping_info) = read_mapping(&mut input)
        // Ok(x) -> Ok(Continue(x))
        .map(CF::Continue)
        // Err(ReachedEndOfInput) -> Ok(Break(ReachedEndOfInput))
        .or_else(|err| err.downcast::<ReachedEndOfInput>().map(CF::Break))
        // Err(_) => return
        ?
    {
        let (_name1, _name2, mut mappings) = mapping_info;
        mappings.sort_by_key(|map| map.start1);

        ranges = apply_mappings(ranges, &mappings)
            .filter(|range| !range.is_empty())
            .collect();

        // TODO remove
        // ranges.sort_by_key(|seed| *seed.start());
    }

    let result = ranges
        .into_iter()
        .map(|range| *range.start())
        .min()
        .expect("no seeds remaining");

    Ok(result)
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Mapping {
    start1: u32,
    start2: u32,
    size: u32,
}

impl Mapping {
    fn map_num(&self, inp: u32) -> Option<u32> {
        (self.input_range())
            .contains(&inp)
            .then(|| self.map_num_unchecked(inp))
    }

    fn map_num_unchecked(&self, inp: u32) -> u32 {
        ((inp as u64 + self.start2 as u64) - self.start1 as u64)
            .try_into()
            .unwrap()
    }

    fn map_range(&self, range: Range<u32>) -> (Range<u32>, [Range<u32>; 2]) {
        let input_range = self.input_range();

        let &is = input_range.start();
        let &ie = input_range.end();

        let &rs = range.start();
        let &re = range.end();

        #[allow(clippy::reversed_empty_ranges)]
        let empty_range = 1..=0;

        let piece_1 = is
            .checked_sub(1)
            .map(|nis| rs..=re.min(nis))
            .unwrap_or(empty_range.clone());

        let piece_2 = ie
            .checked_add(1)
            .map(|nie| rs.max(nie)..=re)
            .unwrap_or(empty_range.clone());

        let mut mapped = empty_range;
        let overlap_start = rs.max(is);
        let overlap_end = re.min(ie);
        if !(overlap_start..=overlap_end).is_empty() {
            let mapped_start = self.map_num(overlap_start).expect("start was not in range");
            let mapped_end = self.map_num(overlap_end).expect("end was not in range");
            mapped = mapped_start..=mapped_end;
        }

        (mapped, [piece_1, piece_2])
    }

    fn input_range(&self) -> Range<u32> {
        self.start1..=self.start1 + (self.size - 1)
    }
}

impl Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = format!(
            "{:?}->{}",
            self.input_range(),
            self.start2 as i64 - self.start1 as i64
        );
        buf.fmt(f)
    }
}

fn apply_mappings(
    numbers: Vec<Range<u32>>,
    mappings: &[Mapping],
) -> impl Iterator<Item = Range<u32>> + '_ {
    numbers.into_iter().flat_map(move |mut range| {
        // println!("working on {range:?}");
        let mut results = vec![];

        for mapping in mappings {
            if range.is_empty() {
                return results;
            }

            let (mapped, [piece1, piece2]) = mapping.map_range(range.clone());
            // println!(
            //     "mapped {mapped:?} {}",
            //     if mapped.is_empty() { "(empty)" } else { "" }
            // );
            // println!(
            //     "piece1 {piece1:?} {}",
            //     if piece1.is_empty() { "(empty)" } else { "" }
            // );
            // println!(
            //     "piece2 {piece2:?} {}",
            //     if piece2.is_empty() { "(empty)" } else { "" }
            // );
            range = piece2;
            results.extend([mapped, piece1])
        }

        results.push(range);

        results
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("there is nothing to read")]
struct ReachedEndOfInput;

fn read_chunk<I: BufRead>(input: &mut I) -> impl Iterator<Item = Result<String, io::Error>> + '_ {
    input.lines().take_while(|line| match line {
        Ok(line) => !line.is_empty(),
        Err(_) => true,
    })
}

fn read_numbers<I: BufRead>(input: &mut I) -> anyhow::Result<Vec<u32>> {
    let mut chunk = read_chunk(input);
    let first = chunk.next().ok_or(ReachedEndOfInput)??;

    // Eat the rest of the chunk
    let extra_count = chunk.count();
    if extra_count > 0 {
        eprintln!("WARN: There was {extra_count} extra lines in the first chunk!");
    }

    let (_, seeds) = first.trim().split_once("seeds: ").unwrap();

    let collect = seeds
        .split_ascii_whitespace()
        .map(|num| num.parse())
        .collect::<Result<_, _>>()?;

    Ok(collect)
}

fn read_mapping<I: BufRead>(input: &mut I) -> anyhow::Result<(String, String, Vec<Mapping>)> {
    let mut chunk = read_chunk(input);
    let first = chunk.next().ok_or(ReachedEndOfInput)??;

    let (names, _) = first.split_once(" map:").unwrap();
    let (name1, name2) = names.split_once("-to-").unwrap();

    let mappings = chunk
        .map(|line| -> anyhow::Result<Mapping> {
            let line = line?;
            let mut numbers = line.split_whitespace().map(|num| num.parse());
            Ok(Mapping {
                start2: numbers.next().unwrap()?,
                start1: numbers.next().unwrap()?,
                size: numbers.next().unwrap()?,
            })
        })
        .collect::<anyhow::Result<_>>()?;

    Ok((name1.to_string(), name2.to_string(), mappings))
}
