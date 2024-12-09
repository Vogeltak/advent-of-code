use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Debug)]
struct Record {
    record: Vec<u8>,
    check: Vec<usize>,
}

impl Record {
    fn comb(&self) -> Vec<Vec<u8>> {
        let qmarks = self
            .record
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == b'?')
            .map(|(i, _)| i)
            .collect_vec();

        let mut candidate = vec![];

        qmarks
            .iter()
            .combinations(self.check.iter().sum::<usize>() - self.broken())
            .filter_map(|x| {
                candidate = self.record.to_vec();
                // Put broken springs combinatorically
                x.iter().for_each(|&i| candidate[*i] = b'#');
                // Put operational springs into remaining places
                qmarks
                    .iter()
                    .filter(|i| !x.contains(i))
                    .for_each(|&i| candidate[i] = b'.');
                self.is_valid(candidate.as_slice())
                    .then_some(candidate.clone())
            })
            .collect_vec()
    }

    fn dp(&self, cache: &HashMap<Record, usize>) -> usize {
        0
    }

    fn broken(&self) -> usize {
        self.record.iter().filter(|&c| *c == b'#').count()
    }

    fn is_valid(&self, candidate: &[u8]) -> bool {
        candidate
            .split(|&b| b == b'.')
            .map(|x| x.len())
            .filter(|l| *l > 0)
            .collect_vec()
            .eq(&self.check)
    }
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (record, check) = value.split_once(' ').unwrap();
        let check = check.split(',').filter_map(|x| x.parse().ok()).collect();
        Self {
            record: record.as_bytes().to_vec(),
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

    let p1 = records.iter().map(|r| r.comb().len()).sum();

    let records = input
        .lines()
        .map(|l| {
            let (record, check) = l.split_once(' ').unwrap();
            let record = (0..5).map(|_| record).join("?");
            let check = (0..5).map(|_| check).join(",");
            format!("{record} {check}")
        })
        .map(|l| Record::from(l.as_str()))
        .collect_vec();

    println!("starting part 2");

    let p2 = records.par_iter().map(|r| r.comb().len()).sum();

    (p1, p2)
}
