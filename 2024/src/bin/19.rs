use std::collections::HashMap;

use itertools::Itertools;

fn towel_permutations_for_pattern<'a>(
    pattern: &'a [u8],
    towels: &[&[u8]],
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    // Recursive base case: we've processed the whole pattern, which
    // means that we succeeded in finding a permutation.
    if pattern.is_empty() {
        return 1;
    }

    // Return memoized sub-problems.
    if let Some(n) = cache.get(pattern) {
        return *n;
    }

    // Find all towels that overlap the start of our pattern and recurse on
    // the sub-pattern.
    let nr = towels
        .iter()
        .filter(|t| pattern.starts_with(t))
        .map(|t| towel_permutations_for_pattern(&pattern[t.len()..], towels, cache))
        .sum();
    cache.insert(pattern, nr);
    nr
}

#[aoc::main(19)]
fn main(input: &str) -> (usize, usize) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(|t| t.as_bytes()).collect_vec();
    let patterns = patterns.lines().map(|l| l.as_bytes()).collect_vec();

    let mut cache = HashMap::new();

    let results = patterns
        .iter()
        .map(|p| towel_permutations_for_pattern(p, &towels, &mut cache))
        .filter(|&perms| perms > 0)
        .collect_vec();

    let p1 = results.len();
    let p2 = results.iter().sum();

    (p1, p2)
}
