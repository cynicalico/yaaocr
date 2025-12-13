use std::error::Error;

pub fn parse<'a>(input: &'a str) -> Result<&'a str, Box<dyn Error>> {
    Ok(input)
}

pub fn part1(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => unreachable!(),
    }))
}

pub fn part2(input: &str) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor < 0 {
            return Some(i + 1);
        }
    }
    unreachable!()
}
