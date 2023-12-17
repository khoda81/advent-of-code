use aoc_2023::{Day, Part};
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    Day::iter().skip(8).take(1).try_for_each(|day| {
        println!("{day:-^30}");

        solve(day, Part::One);
        solve(day, Part::Two);

        Ok(())
    })
}

fn solve(day: Day, part: Part) {
    match day.solve(part) {
        Ok(result) => println!("{part}: {result}"),
        Err(err) => eprintln!("Error solving {part}: {err}"),
    }
}
