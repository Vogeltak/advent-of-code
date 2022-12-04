use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

struct Range {
    start: usize,
    end: usize
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .unwrap();

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?
        })
    }
}

impl Range {
    fn fully_contains(&self, r: &Range) -> bool {
        self.start <= r.start && self.end >= r.end
    }

    fn overlaps(&self, r: &Range) -> bool {
        self.start <= r.end && self.end >= r.start
    }
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let input = input.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a,b)| (Range::from_str(a).unwrap(), Range::from_str(b).unwrap()))
        .collect_vec();
    
    let p1 = input.iter()
        .filter(|(a,b)| a.fully_contains(b) || b.fully_contains(a))
        .count();

    let p2 = input.iter()
        .filter(|(a,b)| a.overlaps(b))
        .count();

    (p1, p2)
}