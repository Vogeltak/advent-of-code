use std::convert::TryFrom;

use hashbrown::HashSet;
use itertools::Itertools;

struct Move {
    direction: (i32, i32),
    steps: usize,
}

impl TryFrom<&str> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, steps) = value.split_once(' ').unwrap();

        let dir = match dir {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => return Err(color_eyre::eyre::eyre!("Not a valid direction: {dir:?}")),
        };

        Ok(Self {
            direction: dir,
            steps: steps.parse()?,
        })
    }
}

fn simulate_rope(moves: &[Move], tail: usize) -> usize {
    let mut rope = vec![(0i32, 0i32); tail + 1];
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    for m in moves {
        for _ in 0..m.steps {
            rope[0] = (rope[0].0 + m.direction.0, rope[0].1 + m.direction.1);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i-1].0 - rope[i].0, rope[i-1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[tail]);
        }
    }

    visited.len()
}

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let input = input
        .lines()
        .map(|l| Move::try_from(l).unwrap())
        .collect_vec();

    (simulate_rope(&input, 1), simulate_rope(&input, 9))
}