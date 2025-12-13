/*
 *
 */

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => unreachable!(),
    })
}

pub fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor < 0 {
            return i + 1;
        }
    }
    unreachable!()
}
