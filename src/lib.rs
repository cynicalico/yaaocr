use std::error::Error;
use std::path::PathBuf;

mod solutions {
    pub mod y2015 {
        pub mod day01;
    }
}

mod util {
    pub mod bits;
    pub mod integer;
    pub mod parse;
}

#[derive(Debug)]
pub struct ParseError(String);

impl Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse puzzle input: {}", self.0)
    }
}

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub input_path: PathBuf,
    pub wrapper: fn(&str) -> Result<(Option<String>, Option<String>), Box<dyn Error>>,
}

pub fn filtered_solutions(year: Option<u32>, day: Option<u32>) -> Vec<Solution> {
    std::iter::empty()
        .chain(y2015())
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| day.is_none_or(|y| y == s.day))
        .collect()
}

macro_rules! make_solutions {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                use crate::util::parse::ParseOps;

                let year: u32 = stringify!($year).unsigned();
                let day: u32 = stringify!($day).unsigned();

                let input_path = std::path::Path::new("input")
                    .join(format!("{}", year))
                    .join(format!("{:02}", day))
                    .with_extension("txt");

                let wrapper = |filepath: &str| {
                    use solutions::$year::$day::*;

                    let input = std::fs::read_to_string(filepath)?;
                    let parsed = parse(&input)?;

                    Ok((part1(&parsed).map(|v| v.to_string()), part2(&parsed).map(|v| v.to_string())))
                };

                Solution { year: year, day: day, input_path, wrapper }
            },)*]
        }
    }
}

make_solutions!(y2015 day01);
