use itertools::Itertools;

fn next_in_seq(nums: &[i64]) -> i64 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs = nums
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    return nums.last().unwrap() + next_in_seq(&diffs);
}

#[aoc::main(09)]
fn main(input: &str) -> (i64, i64) {
    let mut histories = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let p1 = histories.iter().map(|h| next_in_seq(h.as_slice())).sum();
    let p2 = histories
        .iter_mut()
        .map(|h| {
            h.reverse();
            next_in_seq(h.as_slice())
        })
        .sum();

    (p1, p2)
}
