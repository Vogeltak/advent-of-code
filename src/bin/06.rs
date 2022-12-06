use std::collections::HashSet;

fn find_marker(input: &str, size: usize) -> usize {
    input.as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, window)| HashSet::<&u8>::from_iter(window.iter()).len() == size)
        .unwrap().0 + size
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    (find_marker(input, 4), find_marker(input, 14))
}