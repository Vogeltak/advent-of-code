use std::collections::HashSet;

use itertools::Itertools;

fn find_guard(map: &[Vec<u8>]) -> (usize, usize) {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == b'^' {
                return (r, c);
            }
        }
    }

    (0, 0)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn dc(&self) -> isize {
        use Direction::*;
        match self {
            Right => 1,
            Left => -1,
            _ => 0,
        }
    }

    fn dr(&self) -> isize {
        use Direction::*;
        match self {
            Down => 1,
            Up => -1,
            _ => 0,
        }
    }

    fn go_from(&self, (r, c): (usize, usize)) -> Option<(usize, usize)> {
        match (
            r.checked_add_signed(self.dr()),
            c.checked_add_signed(self.dc()),
        ) {
            (Some(r), Some(c)) => Some((r, c)),
            _ => None,
        }
    }
}

fn predict_path(
    map: &[Vec<u8>],
    start_r: usize,
    start_c: usize,
    allow_revisit: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();

    // Simulate the guard's patrol.
    // We're always starting in the Up direction (as verified by static input).
    let mut d = Direction::Up;

    let mut start = (start_r, start_c);

    loop {
        // Found a loop
        if visited.contains(&(start, d)) {
            return None;
        }

        // Keep track of the unique positions the guard has visited
        visited.insert((start, d));

        // Determine the next position as per the direction of the guard
        let next = d.go_from(start);

        let Some(next) = next else {
            // Next position is outside top or left boundaries,
            // meaning that we're done.
            break;
        };

        match map.get(next.0).and_then(|row| row.get(next.1)) {
            // The guard is directly in front of something
            Some(b'#') => d = d.turn(),
            // The guard takes a step forward
            Some(_) => start = next,
            // Next position is outside the map,
            // meaning that we're done
            None => break,
        }
    }

    // Shortcut since we don't use the output Vec anyway when
    // looking for loops.
    if !allow_revisit {
        return Some(vec![]);
    }

    Some(
        visited
            .into_iter()
            .map(|(pos, _)| pos)
            .unique()
            .collect_vec(),
    )
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let mut map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    let start = find_guard(&map);

    let p1 = predict_path(&map, start.0, start.1, true).unwrap();
    let p2 = p1
        .iter()
        .filter(|&&(r, c)| {
            if (r, c) == start {
                return false;
            }
            map[r][c] = b'#';
            let found_loop = predict_path(&map, start.0, start.1, false).is_none();
            map[r][c] = b'.';
            found_loop
        })
        .count();

    (p1.len(), p2)
}
