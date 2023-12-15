use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn hash(x: &str) -> usize {
    x.as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

#[aoc::main(15)]
fn main(input: &str) -> (usize, usize) {
    let steps = input.split(',').collect_vec();

    let p1 = steps.iter().map(|s| hash(s)).sum();

    let mut hashmap: HashMap<usize, VecDeque<(&str, usize)>> = HashMap::new();

    for step in steps {
        match step.contains('=') {
            true => {
                let (label, lens) = step.split_once('=').unwrap();
                let target_box = hash(label);
                let q = hashmap.entry(target_box).or_insert(VecDeque::new());
                match q.iter_mut().find(|(l, _)| *l == label) {
                    Some(x) => x.1 = lens.parse().unwrap(),
                    None => q.push_back((label, lens.parse().unwrap())),
                }
            }
            false => {
                let label = &step[0..step.len() - 1];
                let target_box = hash(label);
                match hashmap.get_mut(&target_box) {
                    Some(q) => match q.iter().find_position(|(l, _)| *l == label) {
                        Some((i, _)) => _ = q.remove(i),
                        None => {}
                    },
                    None => {}
                }
            }
        }
    }

    let p2 = hashmap
        .iter()
        .flat_map(|(b, q)| {
            q.iter()
                .enumerate()
                .map(|(slot, (_, lens))| (*b + 1) * (slot + 1) * *lens)
        })
        .sum();

    (p1, p2)
}
