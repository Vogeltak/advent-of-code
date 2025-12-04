use itertools::Itertools;

fn get(grid: &[&[u8]], r: usize, c: usize) -> u8 {
    *grid.get(r).and_then(|row| row.get(c)).unwrap_or(&b'.')
}

fn count_adjacent_paper(grid: &[&[u8]], p: (usize, usize)) -> usize {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(r, c)| (r, c) != (0, 0))
        .map(
            |(dr, dc)| match (p.0.checked_add_signed(dr), p.1.checked_add_signed(dc)) {
                (Some(r), Some(c)) => get(grid, r, c),
                _ => b'.',
            },
        )
        .filter(|&cell| cell == b'@')
        .count()
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();

    let p1 = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] == b'@')
        .map(|p| count_adjacent_paper(&grid, p))
        .filter(|&n| n < 4)
        .count();

    (p1, 0)
}
