/* Day 5: Doesn't He Have Intern-Elves For This?
 * https://adventofcode.com/2015/day/5
 */

use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn is_nice_p1(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut no_bad = true;
    for (a, b) in input.chars().chain(['\0']).tuple_windows() {
        if a == 'a' || a == 'e' || a == 'i' || a == 'o' || a == 'u' {
            vowel_count += 1;
        }
        if a == b {
            has_double = true;
        }
        if (a == 'a' && b == 'b')
            || (a == 'c' && b == 'd')
            || (a == 'p' && b == 'q')
            || (a == 'x' && b == 'y')
        {
            no_bad = false;
        }
    }
    vowel_count >= 3 && has_double && no_bad
}

pub fn part1(input: &[&str]) -> usize {
    input.iter().filter(|&&s| is_nice_p1(s)).count()
}

fn is_nice_p2(input: &str) -> bool {
    let mut pairs = HashMap::<(char, char), Vec<usize>>::new();
    let mut has_repeat_w_middle = false;
    for (i, (a, b, c)) in input.chars().chain(['\0']).tuple_windows().enumerate() {
        pairs.entry((a, b)).or_default().push(i);
        if a == c {
            has_repeat_w_middle = true;
        }
    }
    has_repeat_w_middle
        && pairs.values().any(|indices| {
            indices
                .iter()
                .permutations(2)
                .any(|p| p[0].abs_diff(*p[1]) > 1)
        })
}

pub fn part2(input: &[&str]) -> usize {
    input.iter().filter(|&&s| is_nice_p2(s)).count()
}
