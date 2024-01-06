use anyhow::Context;
use aoc_2023::{Day, Part, Year};
use config::{Config, File};
use std::{fs, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-cli", about = "Advent of Code CLI")]
struct Args {
    #[structopt(long, default_value = "../", parse(from_os_str))]
    inputs_dir: PathBuf,

    #[structopt(long, default_value = "../secrets.toml", parse(from_os_str))]
    secrets_file: PathBuf,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get-input")]
    GetInput {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,
    },

    #[structopt(name = "solve")]
    Solve {
        #[structopt(short, long)]
        year: Year,

        #[structopt(short, long)]
        day: Day,

        #[structopt(short, long)]
        part: Part,
    },
}

async fn get_input(
    inputs_dir: PathBuf,
    year: Year,
    day: Day,
    secrets: config::Config,
) -> anyhow::Result<()> {
    let cookie = secrets
        .get::<&str>("aoc.token")
        .context("cookie not found in config")?;

    let client = reqwest::Client::builder().build()?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", cookie.parse()?);

    let request_url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        year as u8, day as u8
    );

    let request = client
        .request(reqwest::Method::GET, &request_url)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    // Save the input to the specified output path
    let output_file = get_puzzle_input_path(inputs_dir, year, day);

    let output_dir = output_file
        .parent()
        .expect("the input file is not stored at root");

    tokio::fs::create_dir_all(&output_dir).await?;
    tokio::fs::write(&output_file, body).await?;

    println!("Input saved to: {}", output_file.display());

    Ok(())
}

fn get_puzzle_input_path(inputs_dir: PathBuf, year: Year, day: Day) -> PathBuf {
    inputs_dir
        .join((year as u32).to_string())
        .join((day as u8).to_string())
        .join("input.txt")
}

fn solve(inputs_dir: PathBuf, year: Year, day: Day, part: Part) -> Result<(), anyhow::Error> {
    let path = get_puzzle_input_path(inputs_dir, year, day);

    let input_file = fs::File::open(path.clone()).with_context(|| {
        let invalid_path_msg = "Failed: path had invalid unicode";
        let path = path.to_str().unwrap_or(invalid_path_msg);

        format!("failed to open file \"{path}\"")
    })?;

    let input = BufReader::new(input_file);

    println!("Solving Advent of Code {year}/{day} - Part {part:?}");
    if year == Year::Year2023 {
        match day.solve(input, part) {
            Ok(result) => println!("{part}: {result}"),
            Err(err) => eprintln!("Error solving {part}: {err}"),
        }
    } else {
        panic!("only solutions of year 2023 are supported for now!");
    };

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args();

    // Load configuration from aoc_config.toml
    let config_file_str = args
        .secrets_file
        .to_str()
        .context("invalid UTF-8 in config path.")?;

    match args.cmd {
        Command::GetInput { year, day } => {
            get_input(
                args.inputs_dir,
                year,
                day,
                Config::builder()
                    .add_source(File::with_name(config_file_str))
                    .build()?,
            )
            .await?
        }

        Command::Solve { year, day, part } => solve(args.inputs_dir, year, day, part)?,
    }

    Ok(())
}
