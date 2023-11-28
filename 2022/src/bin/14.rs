use std::cmp::{max, min};

use color_eyre::eyre::Result;
use itertools::Itertools;

#[allow(dead_code)]
fn print_cave(cave: &[Vec<bool>], left: usize, right: usize, height: usize) {
    let left = left - 1;
    let right = right + 1;

    for y in 0..height {
        for row in cave.iter().take(right + 1).skip(left) {
            match row[y] {
                true => print!("█"),
                false => print!("."),
            }
        }
        println!("\n");
    }
}

fn simulate(mut cave: Vec<Vec<bool>>, floor: usize, breaking_point: usize) -> Result<usize> {
    for sand in 0.. {
        let (mut x, mut y) = (500, 0);

        while y + 1 < floor {
            let Some(&dx) = [0, -1, 1]
                .iter()
                .find(|&&dx| {
                    !cave[(x as isize + dx) as usize][y + 1]
                }) else { break; };

            x = (x as isize + dx) as usize;
            y += 1;
        }

        if y == breaking_point {
            return Ok(sand);
        }
        cave[x][y] = true;
    }

    unreachable!()
}

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    // █
    let mut cave = vec![vec![false; 1000]; 1000];
    let mut max_y = 0;
    let mut boundary_left = 1000;
    let mut boundary_right = 0;

    for l in input.lines() {
        let coords = l
            .split(" -> ")
            .map(|x| {
                let (a, b) = x.split_once(',').unwrap();
                (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
            });

        for ((x1, y1), (x2, y2)) in coords.tuple_windows() {
            max_y = max(max_y, max(y1, y2));
            boundary_left = min(boundary_left, min(x1, x2));
            boundary_right = max(boundary_right, max(x1, x2));
            
            let (mut x1, mut y1, x2, y2) = (x1 as isize, y1 as isize, x2 as isize, y2 as isize);
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            cave[x1 as usize][y1 as usize] = true;
            while (x1, y1) != (x2, y2) {
                x1 += dx;
                y1 += dy;
                cave[x1 as usize][y1 as usize] = true;
            }
        }
    }

    print_cave(&cave, boundary_left, boundary_right, max_y);

    let p1 = simulate(cave.clone(), max_y + 2, max_y + 1).unwrap();
    let p2 = simulate(cave, max_y + 2, 0).unwrap() + 1;

    (p1, p2)
}