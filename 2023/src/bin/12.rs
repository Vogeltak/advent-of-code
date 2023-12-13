use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Record<'a> {
    record: &'a [u8],
    check: Vec<usize>,
}

impl<'a> Record<'a> {
    fn arrangements(&'a self) -> usize {
        let qmarks = self
            .record
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'?')
            .map(|(i, _)| i)
            .collect_vec();

        let mut candidate = vec![b'.'; self.record.len()];

        qmarks
            .iter()
            .combinations(self.check.iter().sum::<usize>() - self.broken())
            .filter_map(|x| {
                candidate.fill(b'.');
                x.iter().for_each(|&i| candidate[*i] = b'#');
                self.is_valid(candidate.as_slice()).then_some(0)
            })
            .count()
    }

    fn broken(&'a self) -> usize {
        self.record.iter().filter(|&c| *c == b'#').count()
    }

    fn is_valid(&'a self, candidate: &[u8]) -> bool {
        candidate
            .split(|&b| b == b'.')
            .map(|x| x.len())
            .filter(|l| *l > 0)
            .collect_vec()
            .eq(&self.check)
    }
}

impl<'a> From<&'a str> for Record<'a> {
    fn from(value: &'a str) -> Self {
        let (record, check) = value.split_once(' ').unwrap();
        let check = check.split(',').filter_map(|x| x.parse().ok()).collect();
        Self {
            record: record.as_bytes(),
            check,
        }
    }
}

#[aoc::main(12)]
fn main(input: &str) -> (usize, usize) {
    let records = input
        .lines()
        .map(Record::try_from)
        .filter_map(Result::ok)
        .collect_vec();

    let p1 = records.iter().map(|r| r.arrangements()).sum();

    (p1, 0)
}
