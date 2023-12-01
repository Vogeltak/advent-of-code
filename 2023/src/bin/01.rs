const DIGITS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>())
        .map(|digits| format!("{}{}", digits.first().unwrap(), digits.last().unwrap()))
        .map(|x| x.parse::<usize>().unwrap())
        .sum();

    let p2 = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .map(|l| {
            (0..l.len())
                .filter_map(|i| match l[i] {
                    b'0'..=b'9' => Some((l[i] - b'0') as usize),
                    _ => DIGITS
                        .iter()
                        .enumerate()
                        .find_map(|(di, d)| l[i..].starts_with(d).then_some(di + 1)),
                })
                .collect::<Vec<usize>>()
        })
        .map(|digits| format!("{}{}", digits.first().unwrap(), digits.last().unwrap()))
        .map(|x| x.parse::<usize>().unwrap())
        .sum();

    (p1, p2)
}
