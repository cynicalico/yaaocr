/* Day 6: Probably a Fire Hazard
 * https://adventofcode.com/2015/day/6
 */

use crate::util::parse::ParseOps;
use itertools::Itertools;

pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn parse(input: &str) -> Vec<(Instruction, [usize; 4])> {
    input
        .lines()
        .map(|line| {
            (
                if line.starts_with("turn on") {
                    Instruction::TurnOn
                } else if line.starts_with("turn off") {
                    Instruction::TurnOff
                } else {
                    Instruction::Toggle
                },
                line.iter_unsigned().collect_array().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[(Instruction, [usize; 4])]) -> u32 {
    let mut lights = [false; 1000 * 1000];
    for (instruction, [x0, y0, x1, y1]) in input {
        for y in *y0..=*y1 {
            for x in *x0..=*x1 {
                match instruction {
                    Instruction::TurnOn => lights[y * 1000 + x] = true,
                    Instruction::TurnOff => lights[y * 1000 + x] = false,
                    Instruction::Toggle => lights[y * 1000 + x] = !lights[y * 1000 + x],
                }
            }
        }
    }
    lights.iter().map(|&light| if light { 1 } else { 0 }).sum()
}

pub fn part2(input: &[(Instruction, [usize; 4])]) -> u32 {
    let mut lights = [0u32; 1000 * 1000];
    for (instruction, [x0, y0, x1, y1]) in input {
        for y in *y0..=*y1 {
            for x in *x0..=*x1 {
                match instruction {
                    Instruction::TurnOn => lights[y * 1000 + x] += 1,
                    Instruction::TurnOff => {
                        if lights[y * 1000 + x] > 0 {
                            lights[y * 1000 + x] -= 1;
                        }
                    }
                    Instruction::Toggle => lights[y * 1000 + x] += 2,
                }
            }
        }
    }
    lights.iter().sum()
}
