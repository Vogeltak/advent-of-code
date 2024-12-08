use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone)]
enum Op {
    Add,
    Multiply,
    Concat,
}

#[inline]
fn concat_digits(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn evaluate(parts: &[u64], ops: &[Op]) -> u64 {
    let mut res = parts[0];
    for i in 1..parts.len() {
        match ops[i - 1] {
            Op::Add => res += parts[i],
            Op::Multiply => res *= parts[i],
            Op::Concat => res = concat_digits(res, parts[i]),
        }
    }

    res
}

fn dfs(test: u64, used: &[u64], left: &[u64], ops: &[Op], all_ops: bool) -> bool {
    let Some((i, new_left)) = left.split_first() else {
        return test == evaluate(used, ops);
    };

    let op_candidates = match all_ops {
        true => [Op::Add, Op::Multiply, Op::Concat].to_vec(),
        false => [Op::Add, Op::Multiply].to_vec(),
    };
    op_candidates.into_par_iter().any(|op| {
        dfs(
            test,
            &[used, &[*i]].concat(),
            new_left,
            &[ops, &[op]].concat(),
            all_ops,
        )
    })
}

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let equations: HashMap<u64, Vec<u64>> = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(test, parts)| {
            (
                test.parse().unwrap(),
                parts
                    .split_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect();

    let (p1, p2) = [false, true]
        .into_iter()
        .map(|all_ops| {
            equations
                .iter()
                .filter(|(test, parts)| dfs(**test, &[], parts, &[], all_ops))
                .map(|(test, _)| *test as usize)
                .sum()
        })
        .collect_tuple()
        .unwrap();

    (p1, p2)
}
