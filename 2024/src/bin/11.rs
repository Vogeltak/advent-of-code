use std::collections::HashMap;

fn blink(current_stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut stones = HashMap::with_capacity(current_stones.len());

    for (&stone, &count) in current_stones {
        match stone {
            0 => *stones.entry(1).or_default() += count,
            s if s.to_string().len() % 2 == 0 => {
                // Apologies for the wordplay
                let strone = stone.to_string();
                // 2024.len() = 4
                //   ^------- idx 2
                let (left, right) = strone.split_at(strone.len() / 2);
                *stones.entry(left.parse().unwrap()).or_default() += count;
                *stones.entry(right.parse().unwrap()).or_default() += count;
            }
            _ => *stones.entry(stone * 2024).or_default() += count,
        }
    }

    stones
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut stones: HashMap<u64, usize> = input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    for _ in 0..25 {
        stones = blink(&stones);
    }

    let p1 = stones.values().sum();

    for _ in 0..50 {
        stones = blink(&stones);
    }

    let p2 = stones.values().sum();

    (p1, p2)
}
