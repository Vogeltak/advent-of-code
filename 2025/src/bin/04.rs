use itertools::Itertools;

fn get(grid: &[Vec<u8>], r: usize, c: usize) -> u8 {
    *grid.get(r).and_then(|row| row.get(c)).unwrap_or(&b'.')
}

fn count_adjacent_paper(grid: &[Vec<u8>], p: (usize, usize)) -> usize {
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

fn get_remove_ready_paper(grid: &[Vec<u8>]) -> Option<Vec<(usize, usize)>> {
    let candidates = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] == b'@')
        .filter(|&p| count_adjacent_paper(grid, p) < 4)
        .collect_vec();

    match candidates.len() {
        0 => None,
        _ => Some(candidates),
    }
}

fn remove_max_paper(mut grid: Vec<Vec<u8>>) -> usize {
    let mut total_removed = 0;

    while let Some(candidates) = get_remove_ready_paper(&grid) {
        total_removed += candidates.len();
        for (r, c) in candidates {
            grid[r][c] = b'.';
        }
    }

    total_removed
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();

    let p1 = get_remove_ready_paper(&grid).unwrap().len();

    let p2 = remove_max_paper(grid);

    (p1, p2)
}
