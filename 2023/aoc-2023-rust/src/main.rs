use std::iter;

use aoc_2023::{Day, Part};
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    Day::iter()
        .skip(5)
        .take(1)
        .flat_map(|day| {
            println!("{day:-^30}");
            iter::repeat(day).zip(Part::iter())
        })
        .try_for_each(|(day, part)| {
            match day.solve(part) {
                Ok(result) => println!("{part}: {result}"),
                Err(err) => eprintln!("Error: {err}"),
            };

            Ok(())
        })
}
