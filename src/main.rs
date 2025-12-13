use clap::{Parser, Subcommand};
use std::error::Error;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use yaaocr::{ParseError, Solution, filtered_solutions};

/// Yet Another Advent of Code Runner
#[derive(Debug, Parser)]
#[clap(name = "yaaocr", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        /// Year to run
        #[arg(short, long, required = false)]
        year: Option<u32>,

        /// Day to run
        #[arg(short, long, required = false)]
        day: Option<u32>,

        /// Input file to use instead of the default
        #[arg(short, long, required = false, requires = "year", requires = "day")]
        input_path_override: Option<PathBuf>,

        /// Verify solutions
        #[arg(short, long, required = false)]
        verify: bool,

        /// Display totals
        #[arg(short, long, required = false)]
        totals: bool,
    },
}

fn main() {
    let args = App::parse();

    if let Err(err) = match args.command {
        Command::Run {
            year,
            day,
            input_path_override,
            verify,
            totals,
        } => run(year, day, input_path_override, verify, totals),
    } {
        eprintln!("Error: {err}");
    }
}

fn answer_verification(actual: Option<String>, expected: Option<String>) -> String {
    if let Some(actual) = actual
        && let Some(expected) = expected
    {
        if actual == expected {
            " ✓".to_owned()
        } else {
            format!(" ✗ ({})", expected)
        }
    } else {
        String::new()
    }
}

fn run(
    year: Option<u32>,
    day: Option<u32>,
    input_path_override: Option<PathBuf>,
    verify: bool,
    totals: bool,
) -> Result<(), Box<dyn Error>> {
    let mut duration = Duration::ZERO;
    for Solution {
        year,
        day,
        input_path,
        wrapper,
    } in filtered_solutions(year, day)
    {
        let input_path = input_path_override.as_ref().unwrap_or(&input_path);

        let expected_path = format!("input/{year}/expected/{day:02}.txt");
        let (part1_expected, part2_expected) =
            if verify && Path::new(expected_path.as_str()).exists() {
                let contents = std::fs::read(expected_path)?;
                let mut lines = contents.lines();
                (
                    lines.next().map(|r| r.ok()).flatten(),
                    lines.next().map(|r| r.ok()).flatten(),
                )
            } else {
                (None, None)
            };

        println!("{year} Day {day:02}");

        let instant = Instant::now();
        match wrapper(input_path.to_str().unwrap()) {
            Ok((part1, part2)) => {
                let elapsed = instant.elapsed();
                duration += elapsed;

                println!(
                    "  Part 1: {}{}",
                    part1.clone().unwrap_or("unsolved".to_owned()),
                    answer_verification(part1, part1_expected)
                );
                println!(
                    "  Part 2: {}{}",
                    part2.clone().unwrap_or("unsolved".to_owned()),
                    answer_verification(part2, part2_expected)
                );
                println!("Elapsed: {:.03}s", elapsed.as_nanos() as f64 / 1e9);
            }
            Err(err) => {
                if let Ok(parse_err) = err.downcast::<ParseError>() {
                    println!("  {parse_err}")
                } else {
                    println!("  Missing input!");
                    println!("  Place input file in {}", input_path.display());
                }
            }
        }

        println!();
    }

    if totals {
        println!(
            "Total elapsed time: {:.03}s",
            duration.as_nanos() as f64 / 1e9
        );
        println!();
    }

    Ok(())
}
