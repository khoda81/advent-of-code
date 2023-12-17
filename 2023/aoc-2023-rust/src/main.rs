use aoc_2023::{Day, Part};
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    Day::iter().skip(7).take(1).try_for_each(|day| {
        println!("{day:-^30}");

        // let part = Part::One;
        // match day.solve(part) {
        //     Ok(result) => println!("{part}: {result}"),
        //     Err(err) => eprintln!("Error: {err}"),
        // };

        let part = Part::Two;
        match day.solve(part) {
            Ok(result) => println!("{part}: {result}"),
            Err(err) => eprintln!("Error: {err}"),
        };

        Ok(())
    })
}
