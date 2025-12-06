use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let worksheet = input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();

    let (operators, numbers) = worksheet.split_last().unwrap();

    let p1 = (0..numbers[0].len())
        .map(|c| {
            let op = operators[c];
            (0..numbers.len())
                .map(|r| numbers[r][c].parse::<usize>().unwrap())
                .reduce(|acc, n| match op {
                    "+" => acc + n,
                    "*" => acc * n,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum();

    (p1, 0)
}
