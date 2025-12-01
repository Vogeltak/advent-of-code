use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    r: usize,
    c: usize,
    dir: Option<Direction>,
    tile: Tile,
    score: Option<usize>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Path,
    Wall,
    Start,
    End,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<(isize, isize)> for Direction {
    fn from(value: (isize, isize)) -> Self {
        use Direction::*;
        match value {
            (-1, 0) => North,
            (1, 0) => South,
            (0, -1) => West,
            (0, 1) => East,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn cost_of_turning(&self, other: &Direction) -> usize {
        use Direction::*;
        match (self, other) {
            (North, North) => 1,
            (North, East | West) => 1001,
            (North, South) => 2001,
            (East, North | South) => 1001,
            (East, East) => 1,
            (East, West) => 2001,
            (South, North) => 2001,
            (South, East | West) => 1001,
            (South, South) => 1,
            (West, North | South) => 1001,
            (West, East) => 2001,
            (West, West) => 1,
        }
    }

    fn dr(&self) -> isize {
        use Direction::*;
        match self {
            North => -1,
            East => 0,
            South => 1,
            West => 0,
        }
    }

    fn dc(&self) -> isize {
        use Direction::*;
        match self {
            North => 0,
            East => 1,
            South => 0,
            West => -1,
        }
    }
}

fn dijkstra(map: &[Vec<Node>], mut unvisited: PriorityQueue<Node, Reverse<usize>>) -> usize {
    let mut visited: HashSet<Node> = HashSet::new();

    while let Some(current) = unvisited.pop() {
        visited.insert(current.0.clone());

        // We're finished if we've reached the End
        // if current.0.tile == Tile::End {
        //     break;
        // }

        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            // Find neighboring nodes that are within the map
            .filter_map(|(dr, dc)| {
                let n = map
                    .get((current.0.r as isize + dr) as usize)
                    .and_then(|row| row.get((current.0.c as isize + dc) as usize));

                n.map(|n| (n, Direction::from((dr, dc))))
            })
            // Apply direction to nodes
            .map(|(n, d)| {
                let mut n = n.clone();
                n.dir = Some(d);
                n
            })
            // Discard walls
            .filter(|n| n.tile != Tile::Wall)
            // Only keep nodes we haven't visited yet
            .filter(|n| !visited.iter().any(|m| m.r == n.r && m.c == n.c))
            .collect_vec();

        for n in neighbors {
            let mut n = n.clone();
            let score = current.0.score.as_ref().unwrap()
                + current
                    .0
                    .dir
                    .as_ref()
                    .unwrap()
                    .cost_of_turning(n.dir.as_ref().unwrap());
            n.score = Some(score);
            // println!(
            //     "at {:?} facing {:?} ({}) and moving to {:?} facing {:?} ({})",
            //     (current.0.r, current.0.c),
            //     current.0.dir.as_ref().unwrap(),
            //     current.0.score.as_ref().unwrap(),
            //     (n.r, n.c),
            //     n.dir.as_ref().unwrap(),
            //     n.score.as_ref().unwrap(),
            // );
            unvisited.push(n, Reverse(score));
        }
    }

    // Find optimal paths by going backwards
    let end = visited.iter().find(|n| n.tile == Tile::End).unwrap();
    let mut best_paths = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(end);

    while let Some(cur) = q.pop_front() {
        best_paths.insert((cur.r, cur.c));

        // Find any other neighbors that have equal score and push them to the queue
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dr, dc)| {
                (
                    (
                        (cur.r as isize - dr) as usize,
                        (cur.c as isize - dc) as usize,
                    ),
                    Direction::from((*dr, *dc)),
                )
            })
            .filter_map(|((rr, cc), dir)| {
                visited
                    .iter()
                    .find(|n| n.r == rr && n.c == cc)
                    .map(|n| (n, dir))
            })
            .filter(|(n, dir)| {
                let cost_of_move = n.dir.as_ref().unwrap().cost_of_turning(dir);
                n.score.as_ref().unwrap() + cost_of_move == *cur.score.as_ref().unwrap()
            })
            .for_each(|(n, _)| {
                q.push_back(n);
            });
    }

    println!(
        "found {} tiles on the best paths:\n{:?}",
        best_paths.len(),
        best_paths
    );

    end.score.unwrap()
}

#[aoc::main(16)]
fn main(input: &str) -> (usize, usize) {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    let mut unvisited = PriorityQueue::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, t)| {
                    let mut n = Node {
                        r,
                        c,
                        dir: None,
                        tile: match t {
                            '.' => Tile::Path,
                            '#' => Tile::Wall,
                            'S' => Tile::Start,
                            'E' => Tile::End,
                            _ => unreachable!(),
                        },
                        score: None,
                    };

                    if n.tile == Tile::Start {
                        n.score = Some(0);
                        n.dir = Some(Direction::East);
                        unvisited.push(n.clone(), Reverse(n.score.unwrap()));
                    }

                    n
                })
                .collect_vec()
        })
        .collect_vec();

    let p1 = dijkstra(&map, unvisited);

    (p1, 0)
}
