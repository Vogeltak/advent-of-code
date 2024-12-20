use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    psec: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Explicitly make this a min-heap
        other
            .psec
            .cmp(&self.psec)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Distances for all nodes and the distance to the goal.
type Output = (HashMap<(usize, usize), usize>, usize);

fn dijkstra(grid: &[&[u8]], start: (usize, usize), goal: (usize, usize)) -> Output {
    let mut psec_to_goal = usize::MAX;
    let mut dist = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(x, y)| ((x, y), usize::MAX))
        .collect::<HashMap<_, _>>();

    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(State {
        pos: start,
        psec: 0,
    });

    while let Some(State { pos, psec }) = heap.pop() {
        if pos == goal {
            psec_to_goal = psec;
            continue;
            // return (dist, psec);
        }

        if psec > dist[&pos] {
            continue;
        }

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dr, dc)| {
                (
                    (pos.0 as isize + dr) as usize,
                    (pos.1 as isize + dc) as usize,
                )
            })
            .filter(|&(adj_r, adj_c)| {
                grid.get(adj_r)
                    .and_then(|col| col.get(adj_c))
                    .is_some_and(|tile| tile != &b'#')
            })
            .for_each(|(adj_r, adj_c)| {
                let next = State {
                    pos: (adj_r, adj_c),
                    psec: psec + 1,
                };

                if next.psec < dist[&next.pos] {
                    heap.push(next.clone());
                    dist.insert(next.pos, next.psec);
                }
            });
    }

    (dist, psec_to_goal)
}

fn count_cheats(
    dists: &HashMap<(usize, usize), usize>,
    cheat_time: usize,
    psec_saved: usize,
) -> usize {
    let mut cheats = 0;

    for ((&(r1, c1), &s1), (&(r2, c2), &s2)) in dists.iter().tuple_combinations() {
        let d = r1.abs_diff(r2) + c1.abs_diff(c2);
        if d <= cheat_time
            && s1.abs_diff(s2) >= d + psec_saved
            && s1 != usize::MAX
            && s2 != usize::MAX
        {
            cheats += 1;
        }
    }

    cheats
}

#[aoc::main(20)]
fn main(input: &str) -> (usize, usize) {
    let map = input.lines().map(|l| l.as_bytes()).collect_vec();
    let (mut start, mut end) = ((0, 0), (0, 0));
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            match map[r][c] {
                b'S' => start = (r, c),
                b'E' => end = (r, c),
                _ => {}
            }
        }
    }

    let (dists, baseline) = dijkstra(&map, start, end);

    println!("shortest path without cheating takes {baseline} picoseconds");

    let p1 = count_cheats(&dists, 2, 100);
    let p2 = count_cheats(&dists, 20, 100);

    (p1, p2)
}
