use itertools::Itertools;

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let elves = input.split("\n\n")
        .map(|s| s.lines().map(|c| c.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .collect_vec();
    
    let p1 = elves[0];
    let p2 = elves[0..3].iter().sum();

    (p1, p2)
}