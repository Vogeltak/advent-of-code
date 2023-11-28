use std::cmp::max;

use itertools::Itertools;

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let grid = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec())
        .collect_vec();

    let mut p1 = 0;

    for (i_row, row) in grid.iter().enumerate() {
        for (i_col, tree) in row.iter().enumerate() {
            if grid[i_row][0..i_col].iter().all(|u| u < tree)
                || grid[i_row][i_col + 1..].iter().all(|u| u < tree)
                || grid[0..i_row].iter().all(|u| u[i_col] < *tree)
                || grid[i_row + 1..].iter().all(|u| u[i_col] < *tree)
            {
                p1 += 1;
            }
        }
    }

    let p2 = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .fold(0, |score, (i, j)| {
            let tree = grid[i][j];
            let mut candidate = 1;

            for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let (mut i, mut j, mut dir_score) = (i, j, 0);
                while let Some(&next) = grid
                    .get((i as isize + di) as usize)
                    .and_then(|x| x.get((j as isize + dj) as usize))
                {
                    dir_score += 1;
                    if tree <= next {
                        break;
                    }
                    i = (i as isize + di) as usize;
                    j = (j as isize + dj) as usize;
                }
                candidate *= dir_score;
            }

            max(score, candidate)
        });

    (p1, p2)
}
