/* Day 8: Matchsticks
 * https://adventofcode.com/2015/day/8
 */

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn memory_len(s: &str) -> usize {
    let mut len = 0;

    let mut check_escape = false;
    let mut skip = 0;
    for c in s.chars().skip(1).take(s.len() - 2) {
        if skip > 0 {
            skip -= 1;
        } else if check_escape {
            match c {
                '\\' | '"' => (),
                'x' => skip += 2,
                _ => unreachable!(),
            }
            len += 1;
            check_escape = false;
        } else if c == '\\' {
            check_escape = true;
        } else {
            len += 1;
        }
    }

    len
}

pub fn part1(input: &[&str]) -> usize {
    let total_code_len = input.iter().map(|&l| l.len()).sum::<usize>();
    let total_memory_len = input.iter().map(|&l| memory_len(l)).sum::<usize>();
    total_code_len - total_memory_len
}

fn encoded_len(s: &str) -> usize {
    s.chars()
        .skip(1)
        .take(s.len() - 2)
        .map(|c| match c {
            '\\' => 2,
            '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 6
}

pub fn part2(input: &[&str]) -> usize {
    let total_code_len = input.iter().map(|l| l.len()).sum::<usize>();
    let total_encoded_len = input.iter().map(|l| encoded_len(l)).sum::<usize>();
    total_encoded_len - total_code_len
}
