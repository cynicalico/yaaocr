#![feature(iter_array_chunks)]

use crate::util::parse::ParseOps;
use std::path::{Path, PathBuf};

pub mod aoc_proxy;
pub mod runner;

pub fn check_year_day(year: u32, day: u32) -> bool {
    ((2015..2025).contains(&year) && (1..=25).contains(&day))
        || ((2025..).contains(&year) && (1..=12).contains(&day))
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
        .filter(|s| day.is_none_or(|d| d == s.day))
        .collect()
}

mod util {
    pub mod bits;
    pub mod integer;
    pub mod parse;
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

/*[[[cog
import os
import re

SOLUTIONS_DIR = os.path.join("src", "solutions")

year_re = re.compile(r"^y(\d{4})$")
day_re  = re.compile(r"^day(\d{2})\.rs$")

def year_key(yname: str) -> int:
    m = year_re.match(yname)
    return int(m.group(1)) if m else 0

def chunked(xs, n):
    for i in range(0, len(xs), n):
        yield xs[i:i+n]

years = []
for entry in os.listdir(SOLUTIONS_DIR):
    if year_re.match(entry) and os.path.isdir(os.path.join(SOLUTIONS_DIR, entry)):
        years.append(entry)

cog.outl("mod solutions {")
for year in sorted(years, key=year_key):
    year_path = os.path.join(SOLUTIONS_DIR, year)
    days = []
    for fname in os.listdir(year_path):
        m = day_re.match(fname)
        if m and os.path.isfile(os.path.join(year_path, fname)):
            days.append(m.group(1))

    cog.outl(f"    pub mod {year} {{")
    for dd in sorted(set(days)):
        cog.outl(f"        pub mod day{dd};")
    cog.outl("    }")
cog.outl("}")

for year in sorted(years, key=year_key):
    year_path = os.path.join(SOLUTIONS_DIR, year)
    days = []
    for fname in os.listdir(year_path):
        m = day_re.match(fname)
        if m and os.path.isfile(os.path.join(year_path, fname)):
            days.append(f"day{m.group(1)}")

    days = sorted(set(days))

    cog.outl()
    cog.outl(f"make_solutions!({year}")
    for group in chunked(days, 5):
        cog.out("    " + ", ".join(group))
        cog.outl(");" if group == days[-len(group):] else ",")
]]]*/
mod solutions {
    pub mod y2015 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
        pub mod day09;
        pub mod day10;
    }
}

make_solutions!(y2015
    day01, day02, day03, day04, day05,
    day06, day07, day08, day09, day10);
/*[[[end]]]*/
