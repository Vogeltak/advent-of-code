#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let mut fresh: Vec<(usize, usize)> = fresh
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let available: Vec<usize> = available.lines().map(|l| l.parse().unwrap()).collect();

    let p1 = available
        .iter()
        .filter(|&i| fresh.iter().any(|(a, b)| a <= i && i <= b))
        .count();

    // Sort by start of the range
    fresh.sort_by_key(|r| r.0);

    let mut combined = vec![fresh[0]];
    for &(a, b) in fresh.iter().skip(1) {
        // We can always start from the last range in the combined vector
        let cur = combined.last_mut().unwrap();

        // Base case: new range starts after current last range
        if a > cur.1 {
            combined.push((a, b));
            continue;
        }

        // Otherwise, we're overlappping and we can expand the current last range
        // but only if it's larger than the current end of the range.
        cur.1 = b.max(cur.1);
    }

    let p2 = combined.iter().map(|(a, b)| b - a + 1).sum();

    (p1, p2)
}
