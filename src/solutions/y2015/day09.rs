/* Day 9: All in a Single Night
 * https://adventofcode.com/2015/day/9
 */

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::util::parse::ParseOps;
use itertools::Itertools;

type DistMat = Vec<Vec<u64>>;
type IdCache<'a> = HashMap<&'a str, usize>;

pub fn parse(input: &str) -> DistMat {
    let lines: Vec<[&str; 5]> = input
        .lines()
        .map(|l| l.split_whitespace().collect_array().unwrap())
        .collect();

    let mut ids = IdCache::default();
    let mut next_id = 0usize;
    for line in &lines {
        ids.entry(line[0]).or_insert_with(|| {
            next_id += 1;
            next_id - 1
        });
        ids.entry(line[2]).or_insert_with(|| {
            next_id += 1;
            next_id - 1
        });
    }

    let mut dm = vec![vec![0u64; ids.len()]; ids.len()];
    for line in &lines {
        let from = ids.get(line[0]).unwrap();
        let to = ids.get(line[2]).unwrap();
        let dist: u64 = line[4].unsigned();
        dm[*from][*to] = dist;
        dm[*to][*from] = dist;
    }

    dm
}

fn opti_hamilton(ord: Ordering, dm: &DistMat, from: usize, visited: &mut [bool]) -> u64 {
    let sentinel = if ord == Ordering::Less {
        u64::MAX
    } else {
        u64::MIN
    };
    let mut opti_len = sentinel;

    visited[from] = true;
    for to in (0..visited.len()).filter(|&i| i != from) {
        if visited[to] {
            continue;
        }

        let d = dm[from][to] + opti_hamilton(ord, dm, to, visited);
        if d.cmp(&opti_len) == ord {
            opti_len = d;
        }
    }
    visited[from] = false;

    if opti_len == sentinel { 0 } else { opti_len }
}

pub fn part1(input: &DistMat) -> u64 {
    let mut visited = vec![false; input.len()];
    (0..input.len())
        .map(|start| opti_hamilton(Ordering::Less, input, start, &mut visited))
        .min()
        .unwrap()
}

pub fn part2(input: &DistMat) -> u64 {
    let mut visited = vec![false; input.len()];
    (0..input.len())
        .map(|start| opti_hamilton(Ordering::Greater, input, start, &mut visited))
        .max()
        .unwrap()
}
