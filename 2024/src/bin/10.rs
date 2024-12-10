use cached::proc_macro::cached;
use cached::UnboundCache;
use itertools::Itertools;

#[cached(
    ty = "UnboundCache<(usize, usize), Vec<(usize, usize)>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (r, c) }"#
)]
fn score(map: &[&[u8]], r: usize, c: usize) -> Vec<(usize, usize)> {
    let height = map[r][c];

    if height == b'9' {
        return [(r, c)].to_vec();
    }

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dr, dc)| (r + dr as usize, c + dc as usize))
        .filter_map(|(rr, cc)| {
            map.get(rr)
                .and_then(|row| row.get(cc))
                .map(|h| ((rr, cc), h))
        })
        .filter(|(_, &h)| h == height + 1)
        .flat_map(|((rr, cc), _)| score(map, rr, cc))
        .collect()
}

#[cached(
    ty = "UnboundCache<(usize, usize), usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (r, c) }"#
)]
fn rating(map: &[&[u8]], r: usize, c: usize) -> usize {
    let height = map[r][c];

    if height == b'9' {
        return 1;
    }

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dr, dc)| (r + dr as usize, c + dc as usize))
        .filter_map(|(rr, cc)| {
            map.get(rr)
                .and_then(|row| row.get(cc))
                .map(|h| ((rr, cc), h))
        })
        .filter(|(_, &h)| h == height + 1)
        .map(|((rr, cc), _)| rating(map, rr, cc))
        .sum()
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let map = input.lines().map(|l| l.as_bytes()).collect_vec();

    let trailheads = (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|(r, c)| map[*r][*c] == b'0');

    let p1 = trailheads
        .clone()
        .map(|(r, c)| score(&map, r, c))
        .map(|tops| tops.iter().unique().count())
        .sum();

    let p2 = trailheads.map(|(r, c)| rating(&map, r, c)).sum();

    (p1, p2)
}
