use itertools::Itertools;

struct Report {
    levels: Vec<i16>,
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self {
            levels: value
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect_vec(),
        }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        // Early return if the report was already safe without removing a level
        if within_tolerance(&self.levels) {
            return true;
        }

        // Naive remove one level at a time and check if it's safe
        (0..self.levels.len()).any(|i| {
            let mut candidate = self.levels.clone();
            candidate.remove(i);
            within_tolerance(&candidate)
        })
    }
}

fn within_tolerance(levels: &[i16]) -> bool {
    let deltas = levels.windows(2).map(|s| s[0] - s[1]).collect_vec();
    deltas.iter().all(|d| (-3..0).contains(d)) || deltas.iter().all(|d| (1..4).contains(d))
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let reports = input.lines().map(Report::from).collect_vec();

    let p1 = reports
        .iter()
        .filter(|r| within_tolerance(&r.levels))
        .count();

    let p2 = reports.iter().filter(|r| r.is_safe()).count();

    (p1, p2)
}
