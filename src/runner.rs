use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::{Solution, expected_path, filtered_solutions, input_path};

pub fn run(
    year: Option<u32>,
    day: Option<u32>,
    input_path_override: Option<PathBuf>,
    verify: bool,
    totals: bool,
) -> Result<(), Box<dyn Error>> {
    let mut total_elapsed = Duration::ZERO;

    for Solution { year, day, wrapper } in filtered_solutions(year, day) {
        let input_path = input_path_override.clone().unwrap_or(input_path(year, day));

        if !input_path.exists() {
            eprintln!("{year} Day {day:02}");
            eprintln!("  Missing input!");
            eprintln!("  Place input file at '{}'", input_path.display());
        } else {
            match fs::read_to_string(&input_path) {
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

fn try_read_expected(year: u32, day: u32, verify: bool) -> (Option<String>, Option<String>) {
    let expected_path = expected_path(year, day);
    if verify && expected_path.exists() {
        match fs::read_to_string(&expected_path) {
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
