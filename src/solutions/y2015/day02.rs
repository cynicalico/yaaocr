/* Day 2: I Was Told There Would Be No Math
 * https://adventofcode.com/2015/day/2
 */

use crate::util::parse::ParseOps;
use std::cmp::min;

pub fn parse(input: &str) -> Vec<[u32; 3]> {
    input.iter_unsigned().array_chunks().collect()
}

pub fn part1(input: &[[u32; 3]]) -> u32 {
    input
        .iter()
        .map(|[l, w, h]| 2 * l * w + 2 * w * h + 2 * h * l + min(l * w, min(w * h, h * l)))
        .sum()
}

pub fn part2(input: &[[u32; 3]]) -> u32 {
    input
        .iter()
        .map(|[l, w, h]| 2 * min(l + w, min(w + h, h + l)) + l * w * h)
        .sum()
}
