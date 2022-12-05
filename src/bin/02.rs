use std::str::FromStr;
use std::convert::TryFrom;

use color_eyre::Report;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl TryFrom<&str> for Outcome {
    type Error = Report;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}

impl Outcome {
    fn matching_move(self, other: Move) -> Move {
        match self {
            Outcome::Win => other.winning_move(),
            Outcome::Draw => other.drawing_move(),
            Outcome::Lose => other.losing_move(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for Move {
    type Error = Report;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn beats(&self) -> Self {
        match *self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats() == self)
            .expect("at least one move beats me")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats() == m)
            .expect("I beat at least one move")
    }

    fn drawing_move(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Move,
    me: Move,
}

impl FromStr for Round {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (o, m) = s
            .split_once(' ')
            .unwrap();

        let opponent = Move::try_from(o)?;
        let outcome = Outcome::try_from(m)?;
        let me = outcome.matching_move(opponent);

        Ok(Self { opponent, me })
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
    color_eyre::install().unwrap();

    let strategy: Vec<Round> = input.lines()
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let p1 = 15691;
    let p2 = strategy.iter()
        .map(|r| r.score())
        .sum();
    
    (p1, p2)
}
