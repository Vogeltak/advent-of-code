use itertools::Itertools;

fn max_bank(bank: &[u8], how_many: usize) -> usize {
    let mut joltage = String::new();
    let mut sp = 0;

    for i in 0..how_many {
        sp = (sp..bank.len() - how_many + i + 1)
            .max_by_key(|&x| (bank[x], usize::MAX - x))
            .unwrap();

        joltage.push(bank[sp] as char);

        sp += 1;
    }

    joltage.parse().unwrap()
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let banks = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();

    let p1 = banks.iter().map(|b| max_bank(b, 2)).sum();
    let p2 = banks.iter().map(|b| max_bank(b, 12)).sum();

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_with_second_highest_number() {
        let battery = "811111111111119".as_bytes();
        assert_eq!(max_bank(battery, 2), 89);
    }

    #[test]
    fn same_but_with_12() {
        let battery = "811111111111119".as_bytes();
        assert_eq!(max_bank(battery, 12), 811111111119);
    }
}
