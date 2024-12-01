use std::collections::HashMap;

use itertools::Itertools;

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let (mut l1, mut l2): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    l1.sort_unstable();
    l2.sort_unstable();

    let p1 = l1
        .iter()
        .zip(l2.iter())
        .map(|(i1, i2)| i1.abs_diff(*i2))
        .sum();

    let mut freq = HashMap::new();

    for i in l2.iter() {
        *freq.entry(i).or_insert(0) += 1;
    }

    let p2 = l1
        .iter()
        .map(|i| {
            let i_freq = freq.get(i).unwrap_or(&0);
            i * i_freq
        })
        .sum();

    (p1, p2)
}
