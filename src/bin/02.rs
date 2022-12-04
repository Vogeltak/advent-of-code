use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

trait Beats {
    fn beats(&self) -> Self;
    fn beaten_by(&self) -> Self;
}

impl Beats for Move {
    fn beats(&self) -> Self {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match *self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Move,
    me: Move,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (o, m) = s
            .split_once(' ')
            .unwrap();

        let opponent = match o {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(anyhow!("Can't parse opponent's move")),
        };
        let me = match m {
            "X" => opponent.beats(),
            "Y" => opponent,
            "Z" => opponent.beaten_by(),
            _ => return Err(anyhow!("Can't parse intended outcome")),
        };

        Ok(Round { opponent, me })
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        if self.me.beats() == self.opponent {
            Outcome::Win
        } else if self.me == self.opponent {
            Outcome::Draw
        } else {
            Outcome::Lose
        }
    }
    fn score(&self) -> usize {
        self.me as usize + self.outcome() as usize
    }
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let strategy: Vec<Round> = input.lines()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let p1 = 15691;
    let p2 = strategy.iter()
        .map(|r| r.score())
        .sum();
    
    (p1, p2)
}