use itertools::Itertools;

fn get(grid: &[&[u8]], r: usize, c: usize) -> u8 {
    *grid.get(r).and_then(|row| row.get(c)).unwrap_or(&b'.')
}

fn count_xmas_on(grid: &[&[u8]], r: usize, c: usize) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter(|(dr, dc)| {
        (1..4).all(
            |i| match (r.checked_add_signed(dr * i), c.checked_add_signed(dc * i)) {
                (Some(ri), Some(ci)) => get(grid, ri, ci) == b"XMAS"[i as usize],
                _ => false,
            },
        )
    })
    .count()
}

fn count_x_mas_on(grid: &[&[u8]], r: usize, c: usize) -> bool {
    let x1 = [get(grid, r - 1, c - 1), get(grid, r + 1, c + 1)];
    let x2 = [get(grid, r + 1, c - 1), get(grid, r - 1, c + 1)];
    [x1, x2].iter().all(|x| x == b"MS" || x == b"SM")
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();

    let (mut p1, mut p2) = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            match grid[r][c] {
                b'X' => p1 += count_xmas_on(&grid, r, c),
                b'A' => p2 += count_x_mas_on(&grid, r, c) as usize,
                _ => {}
            }
        }
    }

    (p1, p2)
}
