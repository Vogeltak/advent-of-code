use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    // (dx, dy)
    d: (isize, isize),
}

impl Beam {
    #[inline]
    fn step(&mut self) -> Result<()> {
        self.x = self
            .x
            .checked_add_signed(self.d.0)
            .ok_or(anyhow!("out of bounds (smaller than 0)"))?;
        self.y = self
            .y
            .checked_add_signed(self.d.1)
            .ok_or(anyhow!("out of bounds (smaller than 0)"))?;

        match self.x >= 110 || self.y >= 110 {
            true => Err(anyhow!("out of bounds (larger than 110)")),
            false => Ok(()),
        }
    }
}

fn solve(g: &Vec<&[u8]>, start: Beam) -> usize {
    let mut q = VecDeque::new();
    q.push_back(start);

    let mut seen = HashSet::new();

    while let Some(mut b) = q.pop_front() {
        seen.insert(b.clone());

        let mut new: Option<Beam> = None;

        let step_res = match (g[b.y][b.x], b.d) {
            (b'.', _) => b.step(),
            (b'/', (dx, dy)) => {
                b.d = (-dy, -dx);
                b.step()
            }
            (b'\\', (dx, dy)) => {
                b.d = (dy, dx);
                b.step()
            }
            (b'|', (0, _)) => b.step(),
            (b'|', (dx, 0)) => {
                let mut new_b = b.clone();
                new_b.d = (0, -dx);
                new = Some(new_b);
                b.d = (0, dx);
                b.step()
            }
            (b'-', (_, 0)) => b.step(),
            (b'-', (0, dy)) => {
                let mut new_b = b.clone();
                new_b.d = (-dy, 0);
                new = Some(new_b);
                b.d = (dy, 0);
                b.step()
            }
            _ => Ok(()),
        };

        if step_res.is_ok() && !seen.contains(&b) {
            q.push_back(b);
        }

        if let Some(mut b) = new {
            if b.step().is_ok() && !seen.contains(&b) {
                q.push_back(b);
            }
        }
    }

    seen.into_iter().map(|b| (b.x, b.y)).unique().count()
}

#[aoc::main(16)]
fn main(input: &str) -> (usize, usize) {
    let layout = input.lines().map(|l| l.as_bytes()).collect_vec();

    let p1_start = Beam {
        x: 0,
        y: 0,
        d: (1, 0),
    };
    let p1 = solve(&layout, p1_start);

    let p2 = (0..layout.len())
        .flat_map(|r| {
            // (0, i), (i, 0), (109, i), (i, 109)
            [
                Beam {
                    x: 0,
                    y: r,
                    d: (1, 0),
                },
                Beam {
                    x: layout.len() - 1,
                    y: r,
                    d: (-1, 0),
                },
            ]
        })
        .chain((0..layout.len()).flat_map(|c| {
            [
                Beam {
                    x: c,
                    y: 0,
                    d: (0, 1),
                },
                Beam {
                    x: c,
                    y: layout.len() - 1,
                    d: (0, -1),
                },
            ]
        }))
        .map(|start| solve(&layout, start))
        .max()
        .unwrap();

    (p1, p2)
}
