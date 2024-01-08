use anyhow::Context;
use aoc_2023::{Day, Part, Year};
use std::{
    fs,
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-cli", about = "Advent of Code CLI")]
struct Args {
    #[structopt(long, default_value = "../input", parse(from_os_str))]
    inputs_dir: PathBuf,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "input", about = "Get input for a specific date")]
    Input {
        #[structopt(name = "YEAR")]
        year: Year,

        #[structopt(name = "DAY")]
        day: Day,
    },

    #[structopt(name = "solve", about = "Solve a specific day and year and part")]
    Solve {
        #[structopt(name = "YEAR")]
        year: Year,

        #[structopt(name = "DAY")]
        day: Day,

        #[structopt(short, long)]
        no_part1: bool,

        #[structopt(short, long)]
        no_part2: bool,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Error)]
enum InvalidToken {
    #[error("consider replacing \"TOKEN\" in the .env file with your advent of code session key")]
    DefaultToken,
}

async fn download_input(
    year: Year,
    day: Day,
    session_key: String,
) -> Result<String, anyhow::Error> {
    let cookie = format!("session={session_key}");
    let client = reqwest::Client::builder().build()?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", cookie.parse()?);

    let request_url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        year as u32, day as u8
    );

    let request = client
        .request(reqwest::Method::GET, &request_url)
        .headers(headers);

    let body = request.send().await?.error_for_status()?.text().await?;

    Ok(body)
}

enum Input {
    Cached {
        file: Box<dyn BufRead>,
        path: PathBuf,
    },
    Downloaded(String),
}

impl Input {
    fn into_bufread(self) -> Box<dyn BufRead> {
        match self {
            Input::Cached { file, .. } => file,
            Input::Downloaded(string) => Box::new(Cursor::new(string)),
        }
    }
}

struct App {
    args: Args,
    config: config::Config,
}

impl App {
    async fn run(self) -> anyhow::Result<()> {
        match self.args.command {
            Command::Input { year, day } => {
                let input = self.get_input(year, day).await?;
                if let Input::Cached { path, .. } = &input {
                    println!("Using cached value at {}", path.display());
                }

                for line in input.into_bufread().lines() {
                    println!("{}", line?);
                }
            }

            Command::Solve {
                year,
                day,
                no_part1,
                no_part2,
            } => {
                if !no_part1 {
                    self.solve_part(year, day, Part::One).await?;
                }

                if !no_part2 {
                    self.solve_part(year, day, Part::Two).await?;
                }
            }
        };

        // TODO implement submitting

        Ok(())
    }

    async fn solve_part(&self, year: Year, day: Day, part: Part) -> anyhow::Result<()> {
        print!("Solving Advent of Code {year}/{day} - Part {part:?}: ");
        let answer = self.solve(year, day, part).await?;
        println!("{answer}");
        Ok(())
    }

    async fn get_input(&self, year: Year, day: Day) -> anyhow::Result<Input> {
        // TODO: make the cache user aware
        let session_key = self
            .config
            .get::<String>("aoc_session")
            .context("Consider setting the AOC_SESSION env var")?;

        if session_key == "TOKEN" {
            Err(InvalidToken::DefaultToken)?;
        }

        // Save the input to the specified output path
        let path: PathBuf = self.get_cache_path(year, day);

        if let Ok(file) = fs::File::open(&path).map(BufReader::new).map(Box::new) {
            return Ok(Input::Cached { file, path });
        }

        if path.is_dir() {
            let _ = tokio::fs::remove_dir_all(&path).await;
        }

        let body = download_input(year, day, session_key).await?;

        if let Some(output_dir) = path.parent() {
            tokio::fs::create_dir_all(&output_dir).await?;
        }

        tokio::fs::write(&path, &body).await?;

        Ok(Input::Downloaded(body))
    }

    async fn solve(&self, year: Year, day: Day, part: Part) -> anyhow::Result<String> {
        let input = self.get_input(year, day).await?;
        let input = input.into_bufread();

        if year == Year::Year2023 {
            day.solve(input, part)
        } else {
            panic!("only solutions of year 2023 are supported for now!");
        }
    }

    fn get_cache_path(&self, year: Year, day: Day) -> PathBuf {
        self.args
            .inputs_dir
            .join((year as u32).to_string())
            .join(format!("{}.txt", day as u8))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args();

    match dotenv::dotenv() {
        Ok(path) => path
            .to_str()
            .map(|path_str| println!("Loaded environment variables from {path_str}")),

        Err(err) => return Err(err.into()),
    };

    let config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()?;

    App { args, config }.run().await
}
