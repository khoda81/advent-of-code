use std::{
    fmt::Display,
    io::{self, BufRead},
    ops::{ControlFlow as CF, RangeInclusive},
};
use thiserror::Error;

use crate::{Part, PuzzleResult};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Error)]
#[error("there was an un even number of seeds (num seeds: {0})")]
pub struct UnEvenSeedCount(usize);

type Range<T> = RangeInclusive<T>;

pub fn main<I: BufRead>(mut input: I, part: Part) -> PuzzleResult {
    let numbers = read_numbers(&mut input)?;

    let mut ranges = match part {
        Part::One => numbers.into_iter().map(|num| num..=num).collect(),
        Part::Two => numbers
            .chunks_exact(2)
            .map(<[_; 2]>::try_from)
            .map(|arr| arr.map(|[a, b]| a..=a + b - 1))
            .collect::<Result<_, _>>()
            .map_err(|_| UnEvenSeedCount(numbers.len()))?,
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

        ranges = apply_mappings(ranges, &mappings).collect();
    }

    let result = ranges
        .into_iter()
        .map(|range| *range.start())
        .min()
        .expect("no seeds remaining");

    Ok(result)
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Shift {
    start1: u32,
    start2: u32,
    size: u32,
}

impl Shift {
    pub fn map_num(&self, inp: u32) -> Option<u32> {
        (self.input_range())
            .contains(&inp)
            .then(|| self.map_num_unchecked(inp))
    }

    pub fn map_num_unchecked(&self, inp: u32) -> u32 {
        inp.wrapping_add(self.start2).wrapping_sub(self.start1)
    }

    pub fn map_range(&self, range: Range<u32>) -> (Range<u32>, [Range<u32>; 2]) {
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
            let mapped_start = self.map_num_unchecked(overlap_start);
            let mapped_end = self.map_num_unchecked(overlap_end);
            mapped = mapped_start..=mapped_end;
        }

        (mapped, [piece_1, piece_2])
    }

    pub fn input_range(&self) -> Range<u32> {
        self.start1..=self.start1 + (self.size - 1)
    }
}

impl Display for Shift {
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
    mappings: &[Shift],
) -> impl Iterator<Item = Range<u32>> + '_ {
    numbers
        .into_iter()
        .flat_map(move |range| {
            let mut remained = range;
            let mut results = vec![];

            for mapping in mappings {
                let (mapped, passed);

                (mapped, [passed, remained]) = mapping.map_range(remained.clone());
                results.extend([passed, mapped])
            }

            results.push(remained);

            results
        })
        .filter(|range| !range.is_empty())
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

fn read_mapping<I: BufRead>(input: &mut I) -> anyhow::Result<(String, String, Vec<Shift>)> {
    let mut chunk = read_chunk(input);
    let first = chunk.next().ok_or(ReachedEndOfInput)??;

    let (names, _) = first.split_once(" map:").unwrap();
    let (name1, name2) = names.split_once("-to-").unwrap();

    let mappings = chunk
        .map(|line| -> anyhow::Result<Shift> {
            let line = line?;
            let mut numbers = line.split_whitespace().map(|num| num.parse());
            Ok(Shift {
                start2: numbers.next().unwrap()?,
                start1: numbers.next().unwrap()?,
                size: numbers.next().unwrap()?,
            })
        })
        .collect::<anyhow::Result<_>>()?;

    Ok((name1.to_string(), name2.to_string(), mappings))
}
