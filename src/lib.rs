#![feature(iter_array_chunks)]

use crate::util::parse::ParseOps;
use std::path::{Path, PathBuf};

pub mod aoc_proxy;
pub mod runner;

pub fn check_year_day(year: u32, day: u32) -> bool {
    (year >= 2015 && year < 2025 && day >= 1 && day <= 25)
        || (year == 2025 && day >= 1 && day <= 12)
}

pub fn input_path(year: u32, day: u32) -> PathBuf {
    Path::new("input")
        .join(format!("{year}"))
        .join(format!("day{day:02}"))
        .with_extension("txt")
}

pub fn expected_path(year: u32, day: u32) -> PathBuf {
    Path::new("puzzle")
        .join(format!("{year}"))
        .join("expected")
        .join(format!("day{day:02}"))
        .with_extension("txt")
}

pub fn puzzle_path(year: u32, day: u32) -> PathBuf {
    Path::new("puzzle")
        .join(format!("{year}"))
        .join(format!("day{day:02}"))
        .with_extension("md")
}

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub wrapper: fn(&str) -> (String, String),
}

pub fn filtered_solutions(year: Option<u32>, day: Option<u32>) -> Vec<Solution> {
    std::iter::empty()
        .chain(y2015())
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|y| y == s.day))
        .collect()
}

mod util {
    pub mod bits;
    pub mod integer;
    pub mod parse;
}

mod solutions {
    pub mod y2015 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
    }
}

macro_rules! make_solutions {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).unsigned(),
                    day: stringify!($day).unsigned(),
                    wrapper: |input: &str| {
                        use solutions::$year::$day::*;

                        let parsed_input = parse(&input);
                        let part1 = part1(&parsed_input).to_string();
                        let part2 = part2(&parsed_input).to_string();

                        (part1, part2)
                    }
                }
            ,)*]
        }
    }
}

make_solutions!(y2015
    day01, day02, day03, day04, day05);
