use itertools::Itertools;

fn value(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 26 + 1,
        _ => unreachable!(),
    }
}

fn same_char(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|&c| b.contains(c)).collect_vec()
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| same_char(a, b))
        .map(|c| value(c[0]))
        .sum();

    let p2 = input
        .lines()
        .tuples()
        .map(|(a, b, c)| same_char(a, same_char(b, c).iter().collect::<String>().as_str()))
        .map(|c| value(c[0]))
        .sum();

    (p1, p2)
}
