use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let worksheet = input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();

    let (operators, numbers) = worksheet.split_last().unwrap();

    let p1 = (0..numbers[0].len())
        .map(|c| {
            let op = operators[c];
            (0..numbers.len())
                .map(|r| numbers[r][c].parse::<usize>().unwrap())
                .reduce(|acc, n| match op {
                    "+" => acc + n,
                    "*" => acc * n,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum();

    let mut ceph_worksheet = input.lines().collect_vec();
    let _operators = ceph_worksheet.pop();

    let mut ceph_numbers = vec![vec![]];
    for c in 0..ceph_worksheet[0].len() {
        let n = (0..ceph_worksheet.len())
            .map(|r| ceph_worksheet[r].as_bytes()[c] as char)
            .filter(|ch| ch.is_ascii_digit())
            .collect::<String>();

        if n.is_empty() {
            ceph_numbers.push(vec![]);
        } else {
            ceph_numbers
                .last_mut()
                .unwrap()
                .push(n.parse::<usize>().unwrap());
        }
    }

    let p2 = (0..ceph_numbers.len())
        .map(|c| {
            let op = operators[c];
            (0..ceph_numbers[c].len())
                .map(|r| ceph_numbers[c][r])
                .reduce(|acc, n| match op {
                    "+" => acc + n,
                    "*" => acc * n,
                    _ => unreachable!(),
                })
                .unwrap()
        })
        .sum();

    (p1, p2)
}
