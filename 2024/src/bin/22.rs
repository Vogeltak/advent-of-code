use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Secret(i64);

impl Secret {
    fn evolve(&mut self) {
        // Step 1
        let m64 = self.0 * 64;
        self.mix(m64);
        self.prune();

        // Step 2
        let d32 = self.0 / 32;
        self.mix(d32);
        self.prune();

        // Step 3
        let m2048 = self.0 * 2048;
        self.mix(m2048);
        self.prune();
    }

    fn mix(&mut self, other: i64) {
        self.0 ^= other;
    }

    fn prune(&mut self) {
        self.0 = self.0.rem_euclid(16777216);
    }
}

#[aoc::main(22)]
fn main(input: &str) -> (i64, i64) {
    let mut secrets = input
        .lines()
        .map(|l| l.parse().unwrap())
        .map(Secret)
        .collect_vec();

    let mut changeset_prices = HashMap::new();
    let mut seen = HashSet::new();

    for s in secrets.iter_mut() {
        let mut prices = [0; 2000];
        // Simulate generation of 2000 new secrets
        for i in 0..2000 {
            s.evolve();
            prices[i] = s.0 % 10;
        }

        // Sliding window of 5 to calculate the 4 last price changes
        seen.clear();
        for (a, b, c, d, e) in prices.iter().tuple_windows() {
            let changeset = (b - a, c - b, d - c, e - d);
            // Add the current price for this changeset if it's the first
            // time we saw it for this buyer's price evolution.
            if seen.insert(changeset) {
                *changeset_prices.entry(changeset).or_insert(0) += e;
            }
        }
    }

    let p1 = secrets.iter().map(|s| s.0).sum();
    let p2 = changeset_prices.values().max().unwrap();

    (p1, *p2)
}
