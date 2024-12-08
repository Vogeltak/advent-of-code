use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();

    // Build HashMap of frequencies and the locations of antennas
    let mut antennas_by_freq: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != b'.' {
                antennas_by_freq.entry(grid[r][c]).or_default().push((r, c));
            }
        }
    }

    // Unique locations that contain an antinode
    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

    // Iter through all pairs and add valid antinode locations to a hashset
    for antennas in antennas_by_freq.values() {
        for (&a, &b) in antennas.iter().tuple_combinations() {
            for (a, b) in [(a, b), (b, a)] {
                let (dr, dc) = (b.0 - a.0, b.1 - a.1);
                let mut r = b.0 + dr;
                let mut c = b.1 + dc;

                if (0..grid.len()).contains(&r) && (0..grid[0].len()).contains(&c) {
                    p1.insert((r, c));
                }

                // Include antenna itself
                p2.insert((b.0, b.1));

                while (0..grid.len()).contains(&r) && (0..grid[0].len()).contains(&c) {
                    p2.insert((r, c));
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    (p1.len(), p2.len())
}
