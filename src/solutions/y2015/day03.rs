/* Day 3: Perfectly Spherical Houses in a Vacuum
 * https://adventofcode.com/2015/day/3
 */

use std::collections::HashSet;

pub fn parse(input: &str) -> &str {
    input
}

fn do_move(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '^' => (pos.0, pos.1 + 1),
        'v' => (pos.0, pos.1 - 1),
        '<' => (pos.0 - 1, pos.1),
        '>' => (pos.0 + 1, pos.1),
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();

    let mut santa_pos = (0, 0);
    visited.insert(santa_pos);

    for c in input.chars() {
        santa_pos = do_move(santa_pos, c);
        visited.insert(santa_pos);
    }

    visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();

    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    visited.insert(santa_pos);

    let mut which = false;
    for c in input.chars() {
        if !which {
            santa_pos = do_move(santa_pos, c);
            visited.insert(santa_pos);
        } else {
            robo_santa_pos = do_move(robo_santa_pos, c);
            visited.insert(robo_santa_pos);
        }
        which = !which;
    }

    visited.len()
}
