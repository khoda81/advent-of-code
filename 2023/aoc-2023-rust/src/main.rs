use std::iter;

use aoc_2023::{Day, Part};
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    Day::iter()
        .skip(6)
        .take(1)
        .flat_map(|day| {
            println!("{day:-^20}");
            iter::repeat(day).zip(Part::iter())
        })
        .try_for_each(|(day, part)| day.solve(part).map(|result| println!("{part}: {result}")))
}
