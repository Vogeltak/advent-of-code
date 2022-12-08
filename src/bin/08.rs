use itertools::Itertools;

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec())
        .collect_vec();

    let mut p1 = 0;

    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, tree) in row.iter().enumerate() {
            if grid[i_row][0..i_col].iter().all(|u| u < tree) ||
                grid[i_row][i_col+1..].iter().all(|u| u < tree) ||
                grid[0..i_row].iter().all(|u| u[i_col] < *tree) ||
                grid[i_row+1..].iter().all(|u| u[i_col] < *tree) {
                p1 += 1;
            }
        }
    }

    (p1, 0)
}