use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

use crate::{check_year_day, filtered_solutions, input_path, puzzle_path};
use clap::ValueEnum;
use dotenv::dotenv;
use regex::Regex;
use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct SessionTokenError;

impl Error for SessionTokenError {}

impl fmt::Display for SessionTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to find environment variable: 'AOC_SESSION_TOKEN'"
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum DownloadTarget {
    Input,
    Puzzle,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum PuzzlePart {
    #[value(name = "1")]
    Part1,
    #[value(name = "2")]
    Part2,
}

pub fn download(
    year: u32,
    day: u32,
    what: DownloadTarget,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    if !check_year_day(year, day) {
        eprintln!("Invalid year/day combination: {year} {day}");
        return Ok(());
    }

    let client = http_client()?;
    match what {
        DownloadTarget::Input => download_input(&client, year, day, force),
        DownloadTarget::Puzzle => download_puzzle(&client, year, day, force),
        DownloadTarget::Both => {
            download_input(&client, year, day, force)?;
            download_puzzle(&client, year, day, force)
        }
    }
}

fn download_input(client: &Client, year: u32, day: u32, force: bool) -> Result<(), Box<dyn Error>> {
    let input_path = input_path(year, day);

    if force || !input_path.exists() {
        let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let resp = client.get(&input_url).send()?.error_for_status()?;
        let input = resp.text()?;

        fs::create_dir_all(input_path.parent().unwrap())?;
        fs::write(input_path, input)?;
    } else {
        println!(
            "{} exists, skipping download, use --force to force",
            input_path.display()
        );
    }

    Ok(())
}

fn download_puzzle(
    client: &Client,
    year: u32,
    day: u32,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let puzzle_path = puzzle_path(year, day);

    if force || !puzzle_path.exists() {
        let puzzle_url = format!("https://adventofcode.com/{year}/day/{day}");
        let resp = client.get(&puzzle_url).send()?.error_for_status()?;
        let text = resp.text()?;

        let html = Html::parse_document(&text);
        let selector = Selector::parse("article.day-desc")?;

        let file_content = html.select(&selector).fold(String::new(), |acc, article| {
            if let Ok(md) = htmd::convert(&article.html()) {
                if acc.is_empty() {
                    acc + &md
                } else {
                    acc + "\n\n" + &md
                }
            } else {
                acc
            }
        });

        fs::create_dir_all(puzzle_path.parent().unwrap())?;
        fs::write(puzzle_path, file_content)?;
    } else {
        println!(
            "{} exists, skipping download, use --force to force",
            puzzle_path.display()
        );
    }

    Ok(())
}

pub fn submit(year: u32, day: u32, part: PuzzlePart) -> Result<(), Box<dyn Error>> {
    let answer = {
        let solutions = filtered_solutions(Some(year), Some(day));
        if solutions.is_empty() {
            Err(format!("No solution found for {year} Day {day:02}"))
        } else if solutions.len() > 1 {
            Err(format!("Multiple solutions found for {year} Day {day:02}"))
        } else {
            let input_path = input_path(year, day);
            if !input_path.exists() {
                Err(format!(
                    "Missing input! Download using `yaaocr download {} {} input`",
                    year, day
                ))
            } else {
                match fs::read_to_string(&input_path) {
                    Ok(input) => {
                        let (part1, part2) = (solutions[0].wrapper)(&input);
                        match part {
                            PuzzlePart::Part1 => Ok(part1),
                            PuzzlePart::Part2 => Ok(part2),
                        }
                    }
                    Err(err) => Err(format!("Failed to read input: {err}")),
                }
            }
        }
    }?;

    let mut params = HashMap::new();
    match part {
        PuzzlePart::Part1 => params.insert("level", "1"),
        PuzzlePart::Part2 => params.insert("level", "2"),
    };
    params.insert("answer", &answer);

    println!("Submitting answer: {}", answer);

    let client = http_client()?;
    let submit_url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let resp = client
        .post(&submit_url)
        .form(&params)
        .send()?
        .error_for_status()?;
    let text = resp.text()?;

    let html = Html::parse_document(&text);
    let selector = Selector::parse("article")?;

    if let Some(article) = html.select(&selector).next() {
        let md = htmd::convert(&article.html())?;
        if md.contains("That's the right answer") {
            print!("✓ That's the right answer!");
            if part == PuzzlePart::Part1 {
                print!(" Refreshing puzzle description.");
                download_puzzle(&client, year, day, true)?;
            }
            println!();
        } else if md.contains("That's not the right answer") {
            if md.contains("too low") {
                println!("✗ That's not the right answer, it's too low.");
            } else if md.contains("too high") {
                println!("✗ That's not the right answer, it's too high.");
            } else {
                println!("✗ That's not the right answer.")
            }
        } else if md.contains("You gave an answer too recently") {
            print!("⚠ You gave an answer too recently.");
            if let Some(m) = Regex::new(r"You have \d+s left to wait\.")?.captures(&md) {
                print!(". {}", m.get(0).unwrap().as_str());
            }
            println!();
        } else if md.contains("You don't seem to be solving the right level") {
            println!(
                "⚠ You don't seem to be solving the right level. Did you already complete it?"
            );
        } else {
            eprintln!("⚠ Unexpected response. Saving to bad_response.html");
            fs::write("bad_response.html", text)?;
        }
    } else {
        eprintln!("⚠ Failed to find 'article' in response. Saving to bad_response.html");
        fs::write("bad_response.html", text)?;
    }

    Ok(())
}

fn http_client() -> Result<Client, Box<dyn Error>> {
    dotenv().ok();

    let Ok(session_token) = env::var("AOC_SESSION_TOKEN") else {
        return Err(SessionTokenError.into());
    };

    let cookie = format!("session={session_token}");
    let url: Url = "https://adventofcode.com".parse().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    Client::builder()
        .user_agent("github.com/cynicalico/yaaocr cynicalico@pm.me")
        .cookie_provider(jar.into())
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.into())
}
