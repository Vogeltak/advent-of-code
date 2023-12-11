use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
struct Field<'a> {
    tiles: Vec<&'a [u8]>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn to_diff(&self) -> (isize, isize) {
        use Direction::*;
        match &self {
            N => (-1, 0),
            E => (0, 1),
            S => (1, 0),
            W => (0, -1),
        }
    }
}

impl PartialEq<Direction> for &Direction {
    fn eq(&self, other: &Direction) -> bool {
        *self == other
    }
}

impl<'a> Field<'a> {
    fn find_start(&'a self) -> Option<(usize, usize)> {
        (0..self.tiles.len())
            .cartesian_product(0..self.tiles[0].len())
            .find(|&(r, c)| self.tiles[r][c] == b'S')
    }

    /// Finds connected tiles based on the tile's pipe.
    ///
    /// Assumes that both ends of a pipe must connect with respective pipes
    /// on the other tiles, which holds true for the main loop.
    ///
    /// Can return tiles outside of the field. It is safe for the main loop,
    /// because of the provided property that it's a closed circuit.
    fn get_connections(&'a self, tile: (usize, usize)) -> HashSet<(usize, usize)> {
        self.match_directions(&tile)
            .iter()
            .map(|d| apply_dir_to_tile(tile.clone(), d.clone()))
            .collect()
    }

    fn match_directions(&'a self, tile: &'a (usize, usize)) -> Vec<Direction> {
        use Direction::*;
        [N, E, S, W]
            .into_iter()
            .filter(|d| match self.tiles[tile.0][tile.1] {
                b'|' => d == N || d == S,
                b'-' => d == W || d == E,
                b'L' => d == N || d == E,
                b'J' => d == N || d == W,
                b'7' => d == S || d == W,
                b'F' => d == S || d == E,
                _ => false,
            })
            .collect()
    }

    fn get_loop(&'a self) -> HashSet<(usize, usize)> {
        use Direction::*;
        let mut seen = HashSet::new();
        let start = self.find_start().unwrap();
        seen.insert(start.clone());

        // Get connections for start tile
        let mut conns = [N, E, S, W]
            .into_iter()
            .map(|d| apply_dir_to_tile(start.clone(), d))
            .filter(|candidate| self.get_connections(candidate.clone()).contains(&start))
            .collect::<VecDeque<_>>();

        while let Some(conn) = conns.pop_front() {
            seen.insert(conn.clone());
            self.get_connections(conn)
                .difference(&seen)
                .for_each(|t| conns.push_back(*t));
        }

        seen
    }
}

fn apply_dir_to_tile(tile: (usize, usize), dir: Direction) -> (usize, usize) {
    let diff = dir.to_diff();
    (
        (tile.0 as isize + diff.0).max(0) as usize,
        (tile.1 as isize + diff.1).max(0) as usize,
    )
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let field = Field {
        tiles: input.lines().map(|l| l.as_bytes()).collect_vec(),
    };

    let pipe = field.get_loop();
    let p1 = pipe.len() / 2;
    // Switch whether we're inside when we scan past a pipe that connects north
    let p2 = (0..field.tiles.len())
        .cartesian_product(0..field.tiles[0].len())
        .fold((0, false), |mut state, tile| {
            if !pipe.contains(&tile) {
                state.0 += state.1 as usize;
            } else if field.match_directions(&tile).contains(&Direction::N) {
                state.1 = !state.1;
            }
            state
        })
        .0;

    (p1, p2)
}
