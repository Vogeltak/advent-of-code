use itertools::Itertools;

fn shoelace(instructions: impl Iterator<Item = (u8, isize)>) -> isize {
    let (mut a, mut r, mut c) = (0, 0, 0);

    for (d, n) in instructions {
        let (r_prev, c_prev) = (r, c);
        match d {
            b'U' => r += n,
            b'R' => c += n,
            b'D' => r -= n,
            b'L' => c -= n,
            _ => unreachable!(),
        };
        a += r_prev * c - c_prev * r + n;
    }

    a / 2 + 1
}

#[aoc::main(18)]
fn main(input: &str) -> (isize, isize) {
    let p1 = input.lines().map(|l| {
        let t: (&str, &str, &str) = l.split_whitespace().collect_tuple().unwrap();
        (t.0.as_bytes()[0], t.1.parse().unwrap())
    });

    let p2 = input.lines().map(|l| {
        let hex = l.split_whitespace().last().unwrap();
        // leaves us with just the 6 hexadecimal digits
        let hex = &hex[2..hex.len() - 1];
        let d = match &hex[hex.len() - 1..] {
            "0" => b'R',
            "1" => b'D',
            "2" => b'L',
            "3" => b'U',
            _ => unreachable!(),
        };
        let n = isize::from_str_radix(&hex[0..hex.len() - 1], 16).unwrap();
        (d, n)
    });

    (shoelace(p1), shoelace(p2))
}
