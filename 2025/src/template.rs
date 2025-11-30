use itertools::Itertools;

#[aoc::main(DAY)]
fn main(input: &str) -> (usize, usize) {
    let xs = input
        .lines()
        .map(|l| l.split(',').collect_vec())
        .collect_vec();
    (0, 0)
}
