use itertools::Itertools;

#[derive(Debug)]
struct Dial(i16);

impl Dial {
    fn new() -> Self {
        Self(50)
    }

    fn turn(&mut self, dir: Direction, clicks: i16) -> AtZero {
        match dir {
            Direction::Left => self.turn_left(clicks),
            Direction::Right => self.turn_right(clicks),
        }
    }

    fn turn_left(&mut self, d: i16) -> AtZero {
        self.0 -= d;
        self.0 = self.0.rem_euclid(100);
        (self.0 == 0).into()
    }

    fn turn_right(&mut self, d: i16) -> AtZero {
        self.0 += d;
        self.0 = self.0.rem_euclid(100);
        (self.0 == 0).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AtZero {
    Yes,
    No,
}

impl From<bool> for AtZero {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    clicks: i16,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let (dir, del) = value.split_at(1);
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let clicks = del.parse().unwrap();

        Self { direction, clicks }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let rotations = input.lines().map(|l| Rotation::from(l)).collect_vec();

    let mut dial = Dial::new();
    let password = rotations
        .iter()
        .map(|r| dial.turn(r.direction, r.clicks))
        .filter(|z| z == &AtZero::Yes)
        .count();

    (password, 0)
}
