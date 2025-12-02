use itertools::Itertools;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl From<(&str, &str)> for Range {
    fn from(value: (&str, &str)) -> Self {
        let start = value.0.parse().unwrap();
        let end = value.1.parse().unwrap();

        Self { start, end }
    }
}

impl IntoIterator for Range {
    type Item = usize;

    type IntoIter = RangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        RangeIterator {
            curr: self.start,
            end: self.end,
        }
    }
}

struct RangeIterator {
    curr: usize,
    end: usize,
}

impl Iterator for RangeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.curr;
        self.curr += 1;

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
        .iter()
        .cloned()
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

    let p2 = ranges
        .into_iter()
        .flatten()
        .filter(|id| {
            let n = id.checked_ilog10().unwrap() + 1;

            (1..n / 2 + 1)
                .filter(|d| n % d == 0)
                .map(|d| has_repeat_seq_of_len(*id, d))
                .any(|res| res)
        })
        .sum();

    (p1, p2)
}

fn has_repeat_seq_of_len(id: usize, n: u32) -> bool {
    let mut candidate = id;
    let mut parts = vec![];
    let divisor = 10usize.pow(n);
    while candidate != 0 {
        let prefix = candidate.checked_div(divisor).unwrap();
        let mask = prefix * divisor;
        let part = candidate - mask;
        parts.push(part);
        candidate = prefix;
    }

    parts.iter().all_equal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_seq() {
        assert!(has_repeat_seq_of_len(446446, 3));
        assert!(!has_repeat_seq_of_len(1698522, 2));
        assert!(has_repeat_seq_of_len(824824824, 3));
        assert!(has_repeat_seq_of_len(222222, 1));
        assert!(has_repeat_seq_of_len(111, 3 / 2 + 1))
    }
}
