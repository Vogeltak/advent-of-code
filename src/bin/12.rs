use std::collections::VecDeque;

use itertools::Itertools;

fn bfs(grid: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    while let Some(((x, y), len)) = q.pop_front() {
        if (x, y) == end {
            return Some(len);
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&height) = grid.get(nx).and_then(|row| row.get(ny)) else { continue };
            if height <= grid[x][y] + 1 && !visited[nx][ny] {
                visited[nx][ny] = true;
                q.push_back(((nx, ny), len + 1));
            }
        }
    }

    None
}

#[aoc::main(12)]
fn main(input: &str) -> (usize, usize) {
     let mut grid = input
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect_vec())
        .collect_vec();

    let (sx, sy) = (0..grid.len()).cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'S')
        .unwrap();
    let (ex, ey) = (0..grid.len()).cartesian_product(0..grid[0].len())
        .find(|&(x, y)| grid[x][y] == b'E')
        .unwrap();

    grid[sx][sy] = b'a';
    grid[ex][ey] = b'z';

    let p1 = bfs(&grid, (sx, sy), (ex, ey)).unwrap();

    let p2 = (0..grid.len()).cartesian_product(0..grid[0].len())
        .filter(|&(x, y)| grid[x][y] == b'a')
        .filter_map(|start| bfs(&grid, start, (ex, ey)))
        .min()
        .unwrap();

    (p1, p2)
}