/* Day 10: Elves Look, Elves Say
 * https://adventofcode.com/2015/day/10
 */

use itertools::Itertools;

fn say(seq: &str) -> String {
    let mut s = String::new();
    s.reserve((seq.len() as f64 * 2.0) as usize);

    let mut n = 0;
    for (c1, c2) in std::iter::once(seq.chars().next().unwrap())
        .chain(seq.chars())
        .chain(std::iter::once('\0'))
        .tuple_windows()
    {
        if c1 == c2 {
            n += 1
        } else {
            s.push_str(&format!("{}{}", n, c1));
            n = 1;
        }
    }

    s
}

pub fn parse(input: &str) -> (usize, usize) {
    let mut seq = input.trim().to_string();

    for _ in 0..40 {
        seq = say(&seq);
    }
    let p1_ans = seq.len();

    for _ in 0..10 {
        seq = say(&seq);
    }

    (p1_ans, seq.len())
}

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}
