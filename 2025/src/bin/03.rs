use itertools::Itertools;

fn max_battery(battery: &[u8]) -> usize {
    let mut joltage = String::new();

    // Find the maximum joltage battery while not considering the final one in the bank yet
    let i = (0..battery.len() - 1)
        .max_by_key(|&x| (battery[x], usize::MAX - x))
        .unwrap();

    let max = battery[i];
    joltage.push(max as char);

    // Find the second battery, starting from the index after the first one
    let j = (i + 1..battery.len()).max_by_key(|&x| battery[x]).unwrap();
    joltage.push(battery[j] as char);

    joltage.parse().unwrap()
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let banks = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();

    let p1 = banks.iter().map(|b| max_battery(b)).sum();

    (p1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_with_second_highest_number() {
        let battery = "811111111111119".as_bytes();
        assert_eq!(max_battery(battery), 89);
    }
}
