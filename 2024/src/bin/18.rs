use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Explicitly make this a min-heap
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &[[bool; 71]; 71]) -> Option<usize> {
    let start = (0, 0);
    let goal = (70, 70);

    let mut dist = (0..71)
        .cartesian_product(0..71)
        .map(|(x, y)| ((x, y), usize::MAX))
        .collect::<HashMap<_, _>>();

    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(State {
        pos: start,
        steps: 0,
    });

    while let Some(State { pos, steps }) = heap.pop() {
        if pos == goal {
            return Some(steps);
        }

        if steps > dist[&pos] {
            continue;
        }

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dx, dy)| {
                (
                    (pos.0 as isize + dx) as usize,
                    (pos.1 as isize + dy) as usize,
                )
            })
            .filter(|&(adj_x, adj_y)| {
                grid.get(adj_x)
                    .and_then(|col| col.get(adj_y))
                    .is_some_and(|has_byte| !has_byte)
            })
            .for_each(|(adj_x, adj_y)| {
                let next = State {
                    pos: (adj_x, adj_y),
                    steps: steps + 1,
                };

                if next.steps < dist[&next.pos] {
                    heap.push(next.clone());
                    dist.insert(next.pos, next.steps);
                }
            });
    }

    None
}

#[aoc::main(18)]
fn main(input: &str) -> (usize, String) {
    let falling_bytes = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect_vec();

    let mut mem_space = [[false; 71]; 71];

    // Simulate the first kibibyte falling
    for &(x, y) in &falling_bytes[..1024] {
        mem_space[x][y] = true;
    }

    // Quick and dirty Dijkstra
    let p1 = dijkstra(&mem_space).unwrap();

    // Simulate the bytes falling one at a time to establish which one blocks
    // the path to the goal.
    let p2 = falling_bytes[1024..falling_bytes.len()]
        .iter()
        .find(|(x, y)| {
            mem_space[*x][*y] = true;
            dijkstra(&mem_space).is_none()
        })
        .map(|(x, y)| format!("{x},{y}"))
        .unwrap();

    (p1, p2)
}
