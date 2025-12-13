use clap::{Parser, Subcommand};
use std::error::Error;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use yaaocr::{Solution, filtered_solutions};

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

fn try_read_expected(year: u32, day: u32, verify: bool) -> (Option<String>, Option<String>) {
    let expected_path = PathBuf::from(format!("input/{year}/expected/{day:02}.txt"));
    if verify && expected_path.exists() {
        match std::fs::read_to_string(&expected_path) {
            Ok(content) => {
                let mut lines = content.lines();
                (
                    lines.next().map(|s| s.to_owned()),
                    lines.next().map(|s| s.to_owned()),
                )
            }
            Err(err) => {
                eprintln!("  Failed to read '{}': {err}", expected_path.display());
                (None, None)
            }
        }
    } else {
        (None, None)
    }
}

fn verification_str(actual: String, expected: Option<String>) -> String {
    if let Some(expected) = expected {
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
    let mut total_elapsed = Duration::ZERO;

    for Solution { year, day, wrapper } in filtered_solutions(year, day) {
        let input_path = input_path_override
            .clone()
            .unwrap_or(PathBuf::from(format!("input/{year}/{day:02}.txt")));

        if !input_path.exists() {
            eprintln!("{year} Day {day:02}");
            eprintln!("  Missing input!");
            eprintln!("  Place input file at '{}'", input_path.display());
        } else {
            match std::fs::read_to_string(&input_path) {
                Ok(input) => {
                    println!("{year} Day {day:02}");

                    let instant = Instant::now();
                    let (part1, part2) = wrapper(&input);
                    let elapsed = instant.elapsed();
                    total_elapsed += elapsed;

                    let (part1_expected, part2_expected) = try_read_expected(year, day, verify);

                    println!(
                        "  Part 1: {}{}",
                        part1.clone(),
                        verification_str(part1, part1_expected)
                    );
                    println!(
                        "  Part 2: {}{}",
                        part2.clone(),
                        verification_str(part2, part2_expected)
                    );
                    println!("Elapsed: {:.03}s", elapsed.as_nanos() as f64 / 1e9);
                }
                Err(err) => {
                    eprintln!("{year} Day {day:02}");
                    eprintln!("  Failed to read '{}': {err}", input_path.display());
                }
            }
        }

        println!();
    }

    if totals {
        println!(
            "Total elapsed time: {:.03}s",
            total_elapsed.as_nanos() as f64 / 1e9
        );
        println!();
    }

    Ok(())
}
