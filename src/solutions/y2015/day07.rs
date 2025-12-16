/* Day 7: Some Assembly Required
 * https://adventofcode.com/2015/day/7
 */

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Gate<'a> {
    And(&'a str, &'a str),
    AndConst(u16, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
    Not(&'a str),
    Jumper(&'a str),
}

#[derive(Debug, Copy, Clone)]
pub enum Wire<'a> {
    Signal(u16),
    Gate(Gate<'a>),
}

pub fn parse(input: &str) -> HashMap<&str, Wire<'_>> {
    Regex::new(r"([a-z\d]*?) ?(AND|OR|LSHIFT|RSHIFT|NOT|) ?([a-z\d]+) -> ([a-z]+)")
        .unwrap()
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [lhs, op, rhs, wire])| {
            (
                wire,
                match op {
                    "AND" => match lhs.parse() {
                        Ok(v) => Wire::Gate(Gate::AndConst(v, rhs)),
                        Err(_) => Wire::Gate(Gate::And(lhs, rhs)),
                    },
                    "OR" => Wire::Gate(Gate::Or(lhs, rhs)),
                    "LSHIFT" => Wire::Gate(Gate::LShift(lhs, rhs.parse().unwrap())),
                    "RSHIFT" => Wire::Gate(Gate::RShift(lhs, rhs.parse().unwrap())),
                    "NOT" => Wire::Gate(Gate::Not(rhs)),
                    _ => match rhs.parse() {
                        Ok(v) => Wire::Signal(v),
                        Err(_) => Wire::Gate(Gate::Jumper(rhs)),
                    },
                },
            )
        })
        .collect()
}

fn solve(input: &mut HashMap<&str, Wire<'_>>, wire: &str) -> u16 {
    let v = match input[wire] {
        Wire::Signal(v) => v,
        Wire::Gate(Gate::And(lhs, rhs)) => solve(input, lhs) & solve(input, rhs),
        Wire::Gate(Gate::AndConst(v, rhs)) => v & solve(input, rhs),
        Wire::Gate(Gate::Or(lhs, rhs)) => solve(input, lhs) | solve(input, rhs),
        Wire::Gate(Gate::LShift(lhs, rhs)) => solve(input, lhs) << rhs,
        Wire::Gate(Gate::RShift(lhs, rhs)) => solve(input, lhs) >> rhs,
        Wire::Gate(Gate::Not(rhs)) => !solve(input, rhs),
        Wire::Gate(Gate::Jumper(rhs)) => solve(input, rhs),
    };
    *input.get_mut(wire).unwrap() = Wire::Signal(v);
    v
}

pub fn part1(input: &HashMap<&str, Wire>) -> u16 {
    let mut wires = input.clone();
    solve(&mut wires, "a")
}

pub fn part2(input: &HashMap<&str, Wire>) -> u16 {
    let mut wires = input.clone();
    let prev_a = solve(&mut wires, "a");

    wires = input.clone();
    *wires.get_mut("b").unwrap() = Wire::Signal(prev_a);
    solve(&mut wires, "a")
}
