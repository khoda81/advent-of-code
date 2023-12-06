use aoc_2023::{Day, Part};

fn main() -> anyhow::Result<()> {
    let day = Day::Two;
    let part = Part::Two;

    let result = day.solve(part)?;
    println!("{result}");

    Ok(())
}
