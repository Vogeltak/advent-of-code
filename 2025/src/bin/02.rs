use itertools::Itertools;

struct Range {
    start: usize,
    end: usize,
    next: usize,
}

impl From<(&str, &str)> for Range {
    fn from(value: (&str, &str)) -> Self {
        let start = value.0.parse().unwrap();
        let end = value.1.parse().unwrap();

        Self {
            start,
            end,
            next: start,
        }
    }
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.next;
        self.next += 1;

        if candidate > self.end {
            None
        } else {
            Some(candidate)
        }
    }
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let ranges = input
        .split(',')
        .map(|r| Range::from(r.split_once('-').unwrap()))
        .collect_vec();

    let p1 = ranges
        .into_iter()
        .flatten()
        .filter(|id| {
            let n = id.checked_ilog10().unwrap() + 1;

            // Early return because IDs with odd number of digits cannot be invalid
            if n % 2 == 1 {
                return false;
            }

            let mid = 10usize.pow(n / 2);
            let prefix = id.checked_div(mid).unwrap();
            let mask = prefix * mid;
            let suffix = id - mask;

            prefix == suffix
        })
        .sum();

    (p1, 0)
}
