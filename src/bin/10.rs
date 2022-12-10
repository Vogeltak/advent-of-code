use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

#[aoc::main(10)]
fn main(input: &str) -> (i32, i32) {
    let mut instructions = input
        .lines()
        .map(|l| l.split_once(' ').map(|(_, i)| i.parse::<i32>().unwrap()))
        .cycle();

    let p1 = instructions
        .fold_while((1, 1, 0), |(mut cycle, mut rx, mut sig), i| {
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sig += cycle * rx;
                println!("{} * {} = {}", cycle, rx, cycle*rx);
            }

            match i {
                Some(val) => {
                    cycle += 1;
                    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                        sig += cycle * rx;
                        println!("{} * {} = {}", cycle, rx, cycle*rx);
                    }
                    cycle += 1;
                    rx += val;
                },
                None => cycle += 1,
            }

            match cycle {
                0..=240 => Continue((cycle, rx, sig)),
                _ => Done((cycle, rx, sig)),
            }
        }).into_inner().2;

    (p1, 0)
}