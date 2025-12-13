use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

use crate::{check_year_day, input_path, puzzle_path};
use clap::ValueEnum;
use dotenv::dotenv;
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

#[derive(Clone, Debug, ValueEnum)]
pub enum DownloadTarget {
    Input,
    Puzzle,
    Both,
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
