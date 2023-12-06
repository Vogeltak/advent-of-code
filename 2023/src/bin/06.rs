fn count_wins(time: u64, dist: u64) -> usize {
    (1..time).filter(|i| i * (time - i) > dist).count()
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let (times, dists): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|l| l.split_whitespace().filter_map(|n| n.parse::<u32>().ok()))
        .partition(|n| *n < 100);

    let p1 = times
        .iter()
        .zip(dists.iter())
        .map(|(t, d)| count_wins(*t as u64, *d as u64))
        .product();

    let (time, dist) = [times, dists]
        .map(|v| {
            v.iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .concat()
                .parse::<u64>()
                .unwrap()
        })
        .into();

    let p2 = count_wins(time, dist);

    (p1, p2)
}
