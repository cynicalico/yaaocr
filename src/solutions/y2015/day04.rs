/* Day 4: The Ideal Stocking Stuffer
 * https://adventofcode.com/2015/day/4
 */

use md5::{Digest, Md5};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

const NOT_FOUND: usize = usize::MAX;

fn compare_exchange_min(slot: &AtomicUsize, v: usize) {
    let mut cur = slot.load(Ordering::Relaxed);
    while cur == NOT_FOUND || cur > v {
        match slot.compare_exchange_weak(cur, v, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(new_cur) => cur = new_cur,
        }
    }
}

pub fn parse(input: &str) -> (usize, usize) {
    let input = input.trim().to_owned();

    let p1_ans = Arc::new(AtomicUsize::new(NOT_FOUND));
    let p2_ans = Arc::new(AtomicUsize::new(NOT_FOUND));

    let thread_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let mut threads = Vec::with_capacity(thread_count);
    for tid in 0..thread_count {
        let input = input.clone();
        let p1_ans = Arc::clone(&p1_ans);
        let p2_ans = Arc::clone(&p2_ans);

        threads.push(std::thread::spawn(move || {
            let mut i = tid + 1;
            loop {
                let curr_p2_ans = p2_ans.load(Ordering::Relaxed);
                if curr_p2_ans != NOT_FOUND && i > curr_p2_ans {
                    break;
                }

                let digest = Md5::digest(format!("{input}{i}"));
                if digest[0] == 0 && digest[1] == 0 && digest[2] >> 4 == 0 {
                    compare_exchange_min(&p1_ans, i);
                }
                if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
                    compare_exchange_min(&p2_ans, i);
                }

                i += thread_count;
            }
        }));
    }

    for t in threads {
        t.join().ok();
    }

    (
        p1_ans.load(Ordering::Relaxed),
        p2_ans.load(Ordering::Relaxed),
    )
}

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}
