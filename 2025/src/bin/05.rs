#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let fresh: Vec<(usize, usize)> = fresh
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let available: Vec<usize> = available.lines().map(|l| l.parse().unwrap()).collect();

    let p1 = available
        .iter()
        .filter(|&i| fresh.iter().any(|(a, b)| a <= i && i <= b))
        .count();

    (p1, 0)
}
